# RFC: Expense Items UI & OCR Integration

**Status:** Draft
**Created:** 2026-03-22
**Scope:** zfaktury-app crate (UI only -- backend is complete)

## Problem Statement

The Rust implementation of ZFaktury has a complete backend for expense line items and OCR
(domain structs, repository with atomic item management, services with totals calculation),
but the GPUI desktop UI is missing critical features that exist in the Go+SvelteKit version:

1. **No expense items editor** -- `ExpenseFormView` always creates expenses with `items: Vec::new()`
2. **No items display** -- `ExpenseDetailView` shows no line items section
3. **Broken review route** -- `Route::ExpenseReview` has no `i64` parameter, `ExpenseReviewView` always receives `expense_id: 0`
4. **OCR results discarded** -- `ExpenseImportView` runs OCR but logs and discards the result
5. **No OCR-to-form mapping** -- Review view cannot display or apply OCR-extracted data
6. **No OCR items in review** -- `OCRResult.items` are never shown to the user

## Architecture Decision: OCR Data Passing

**Problem:** After OCR runs in `ExpenseImportView`, the `OCRResult` needs to reach `ExpenseReviewView`.
The `Route` enum derives `Clone + PartialEq + Eq` and represents URL-like paths -- it should not carry data payloads.

**Decision:** Add a thread-safe one-shot cache on `AppServices`:
```rust
pub pending_ocr_result: Arc<std::sync::Mutex<Option<(i64, OCRResult)>>>
```
- Import view stores `(expense_id, ocr_result)` after successful OCR
- Root view retrieves and clears it with `.take()` when constructing `ExpenseReviewView`
- If the cache is empty (OCR failed or not configured), the review form loads from DB only

**Rationale:** Lightweight, no new crates, Mutex only briefly locked on set/get (never across awaits), matches desktop app transient data patterns.

## Architecture Decision: Separate ExpenseItemsEditor

**Problem:** `InvoiceItemsEditor` works with `InvoiceItem`. `ExpenseItem` has the same fields but different type (`expense_id` vs `invoice_id`). Should we generify?

**Decision:** Create a separate `ExpenseItemsEditor` component.

**Rationale:**
- Generifying requires a trait in `zfaktury-domain` (separate crate), changing InvoiceItemsEditor signatures, and modifying InvoiceFormView -- high risk for this change
- The two types may diverge (expenses could get category per item in future)
- ~445 lines of mostly UI boilerplate -- acceptable duplication
- Each editor can evolve independently

---

## Implementation Phases

### Phase 1: ExpenseItemsEditor Component

**New file:** `crates/zfaktury-app/src/components/expense_items_editor.rs`

**Modify:** `crates/zfaktury-app/src/components/mod.rs` -- add `pub mod expense_items_editor;`

**Design:** Clone the structure of `invoice_items_editor.rs` with these changes:

| Aspect | InvoiceItemsEditor | ExpenseItemsEditor |
|--------|-------------------|-------------------|
| Import | `InvoiceItem` | `ExpenseItem` |
| Event | `ItemsChanged` | `ExpenseItemsChanged` |
| Output | `to_invoice_items() -> Vec<InvoiceItem>` | `to_expense_items() -> Vec<ExpenseItem>` |
| Input | `set_items(&[InvoiceItem])` | `set_items(&[ExpenseItem])` |
| Field IDs | `item-desc-{idx}` | `exp-item-desc-{idx}` |

**Struct:**
```rust
pub struct ExpenseItemsChanged;

struct ItemRow {
    description: Entity<TextInput>,
    quantity: Entity<NumberInput>,
    unit: Entity<TextInput>,
    unit_price: Entity<NumberInput>,
    vat_rate: Entity<Select>,
}

pub struct ExpenseItemsEditor {
    items: Vec<ItemRow>,
}
```

**Methods (same signatures as InvoiceItemsEditor):**
- `new(cx) -> Self` -- creates editor with 1 empty row
- `to_expense_items(&self, cx: &App) -> Vec<ExpenseItem>` -- reads all rows, returns domain structs with `id: 0, expense_id: 0` (service sets these)
- `set_items(&mut self, items: &[ExpenseItem], cx)` -- populates from existing items (edit/OCR mode)
- `add_item(&mut self, cx)` -- appends empty row
- `remove_item(&mut self, index, cx)` -- removes row (only if >1 remain)
- `compute_totals(&self, cx) -> (Amount, Amount, Amount)` -- live (subtotal, vat, total)
- `render_header() -> Div` -- column headers
- `render_totals(&self, cx) -> Div` -- formatted totals display
- `Render::render()` -- header + rows + add button + totals

**No changes to:** InvoiceItemsEditor, domain types, services, or repository.

---

### Phase 2: OCR Data Cache

**File:** `crates/zfaktury-app/src/app.rs`

**Change:** Add field to `AppServices` struct:
```rust
use zfaktury_domain::OCRResult;

pub struct AppServices {
    // ... existing fields ...
    pub pending_ocr_result: Arc<std::sync::Mutex<Option<(i64, OCRResult)>>>,
}
```

**Initialize in `AppServices::new()`:**
```rust
pending_ocr_result: Arc::new(std::sync::Mutex::new(None)),
```

**Requires:** `OCRResult` must implement `Send` (it does -- only owned `String`, `Amount(i64)`, `Vec<OCRItem>`, `f64`).

---

### Phase 3: Fix Route::ExpenseReview

**File:** `crates/zfaktury-app/src/navigation.rs`

**Changes:**

1. Change enum variant:
```rust
// Before:
ExpenseReview,

// After:
ExpenseReview(i64),
```

2. Remove static match (line 89):
```rust
// Remove:
"/expenses/review" => Some(Route::ExpenseReview),
```

3. Add dynamic match in `parse_dynamic_route()`:
```rust
["expenses", id, "review"] => id.parse().ok().map(Route::ExpenseReview),
```
Must be placed BEFORE `["expenses", id]` to avoid `id="review"` matching ExpenseDetail.

4. Update `to_path()`:
```rust
Route::ExpenseReview(id) => format!("/expenses/{id}/review"),
```

5. Update `label()`:
```rust
Route::ExpenseReview(_) => "Kontrola nakladu",
```

**File:** `crates/zfaktury-app/src/root.rs`

**Changes to `create_content_view()`:**
```rust
Route::ExpenseReview(id) => {
    let svc = services.expenses.clone();
    let pending_ocr = services.pending_ocr_result.clone();
    let id = *id;
    let ocr_result = {
        let mut lock = pending_ocr.lock().unwrap();
        match lock.as_ref() {
            Some((eid, _)) if *eid == id => lock.take().map(|(_, r)| r),
            _ => None,
        }
    };
    ContentView::ExpenseReview(
        cx.new(move |cx| ExpenseReviewView::new(svc, id, ocr_result, cx)),
    )
}
```

---

### Phase 4: Import -> Review Flow with OCR Data

**File:** `crates/zfaktury-app/src/views/expense_import.rs`

**Changes:**

1. Add field to `ExpenseImportView`:
```rust
pub struct ExpenseImportView {
    // ... existing ...
    pending_ocr_cache: Arc<std::sync::Mutex<Option<(i64, OCRResult)>>>,
}
```

2. Update constructor signature:
```rust
pub fn new(
    import_service: Arc<ImportService>,
    document_service: Arc<DocumentService>,
    ocr_service: Option<Arc<OCRService>>,
    pending_ocr_cache: Arc<std::sync::Mutex<Option<(i64, OCRResult)>>>,
    _cx: &mut Context<Self>,
) -> Self
```

3. Change `process_file()` async block return type:
```rust
// Before:
Ok::<i64, String>(expense.id)

// After:
Ok::<(i64, Option<OCRResult>), String>((expense.id, ocr_result_out))
```

4. Capture OCR result instead of discarding:
```rust
let mut ocr_result_out = None;
if let Some(ref ocr) = ocr_svc {
    match ocr.process_bytes(&data, &content_type) {
        Ok(result) => {
            log::info!("OCR processing successful for expense {}", expense.id);
            ocr_result_out = Some(result);
        }
        Err(e) => {
            log::warn!("OCR processing failed for expense {}: {e}", expense.id);
        }
    }
}
Ok::<(i64, Option<OCRResult>), String>((expense.id, ocr_result_out))
```

5. In the `this.update()` callback, store result and navigate:
```rust
Ok((expense_id, ocr_result)) => {
    if let Some(result) = ocr_result {
        // Store OCR result for review view
        if let Ok(mut lock) = this.pending_ocr_cache.lock() {
            *lock = Some((expense_id, result));
        }
        // Navigate to review (OCR data available)
        cx.emit(NavigateEvent(Route::ExpenseReview(expense_id)));
    } else {
        // No OCR data -- go directly to detail
        cx.emit(NavigateEvent(Route::ExpenseDetail(expense_id)));
    }
}
```

**File:** `crates/zfaktury-app/src/root.rs` (ExpenseImport arm)

```rust
Route::ExpenseImport => {
    let import_svc = services.import.clone();
    let doc_svc = services.documents.clone();
    let ocr_svc = services.ocr_service.clone();
    let pending_ocr = services.pending_ocr_result.clone();
    ContentView::ExpenseImport(
        cx.new(|cx| ExpenseImportView::new(import_svc, doc_svc, ocr_svc, pending_ocr, cx)),
    )
}
```

---

### Phase 5: ExpenseReviewView with Items & OCR Population

**File:** `crates/zfaktury-app/src/views/expense_review.rs`

**Changes:**

1. Add imports:
```rust
use zfaktury_domain::{ExpenseItem, OCRResult};
use crate::components::expense_items_editor::{ExpenseItemsEditor, ExpenseItemsChanged};
```

2. Add fields to struct:
```rust
pub struct ExpenseReviewView {
    // ... existing fields ...
    items_editor: Entity<ExpenseItemsEditor>,
    pending_ocr_result: Option<OCRResult>,
}
```

3. Update constructor:
```rust
pub fn new(
    expense_service: Arc<ExpenseService>,
    expense_id: i64,
    ocr_result: Option<OCRResult>,
    cx: &mut Context<Self>,
) -> Self {
    let items_editor = cx.new(ExpenseItemsEditor::new);
    cx.subscribe(&items_editor, |_this: &mut Self, _, _: &ExpenseItemsChanged, cx| {
        cx.notify();
    }).detach();

    // ... existing field creation ...

    // In the async load callback, after populate_from_expense:
    // if let Some(ref ocr) = this.pending_ocr_result.take() {
    //     this.apply_ocr_result(ocr, cx);
    // }

    Self {
        // ... existing ...
        items_editor,
        pending_ocr_result: ocr_result,
    }
}
```

4. Add `apply_ocr_result()` method:
```rust
fn apply_ocr_result(&mut self, result: &OCRResult, cx: &mut Context<Self>) {
    if !result.description.is_empty() {
        self.description.update(cx, |t, cx| t.set_value(&result.description, cx));
    }
    if !result.invoice_number.is_empty() {
        self.invoice_number.update(cx, |t, cx| t.set_value(&result.invoice_number, cx));
    }
    if !result.issue_date.is_empty() {
        self.issue_date.update(cx, |d, cx| d.set_iso_value(&result.issue_date, cx));
    }
    if result.total_amount != Amount::ZERO {
        self.total_amount.update(cx, |n, cx| n.set_amount(result.total_amount, cx));
    }
    if result.vat_amount != Amount::ZERO {
        self.vat_amount.update(cx, |n, cx| n.set_amount(result.vat_amount, cx));
    }
    if result.vat_rate_percent != 0 {
        self.vat_rate.update(cx, |s, cx| {
            s.set_selected_value(&result.vat_rate_percent.to_string(), cx);
        });
    }
    if !result.currency_code.is_empty() {
        self.currency.update(cx, |s, cx| {
            s.set_selected_value(&result.currency_code, cx);
        });
    }
    // Apply line items from OCR
    if !result.items.is_empty() {
        let items: Vec<ExpenseItem> = result.items.iter().enumerate().map(|(i, ocr_item)| {
            ExpenseItem {
                id: 0,
                expense_id: 0,
                description: ocr_item.description.clone(),
                quantity: ocr_item.quantity,
                unit: "ks".to_string(),
                unit_price: ocr_item.unit_price,
                vat_rate_percent: ocr_item.vat_rate_percent,
                vat_amount: Amount::ZERO,
                total_amount: Amount::ZERO,
                sort_order: (i + 1) as i32,
            }
        }).collect();
        self.items_editor.update(cx, |editor, cx| editor.set_items(&items, cx));
    }
}
```

5. Update `save()` -- read items from editor:
```rust
// Before building expense struct, add:
let items = self.items_editor.read(cx).to_expense_items(cx);

// In the Expense struct:
items,  // was: items: Vec::new()
```

6. Update `render()` -- add items card between "Castka a DPH" and action buttons:
```rust
// After Card 2 (Castka a DPH), add:
outer = outer.child(render_card(
    "Polozky",
    div().child(self.items_editor.clone()),
));
```

---

### Phase 6: ExpenseFormView with Items Toggle

**File:** `crates/zfaktury-app/src/views/expense_form.rs`

**Changes:**

1. Add imports:
```rust
use crate::components::expense_items_editor::{ExpenseItemsEditor, ExpenseItemsChanged};
```

2. Add fields:
```rust
pub struct ExpenseFormView {
    // ... existing ...
    items_editor: Entity<ExpenseItemsEditor>,
    use_items: bool,
}
```

3. In `new_create()`:
```rust
let items_editor = cx.new(ExpenseItemsEditor::new);
cx.subscribe(&items_editor, |_this: &mut Self, _, _: &ExpenseItemsChanged, cx| {
    cx.notify();
}).detach();

// In Self { ... }:
items_editor,
use_items: false,
```

4. In `new_edit()`:
```rust
// Same items_editor creation and subscription
// In Self { ... }:
items_editor,
use_items: false,  // will be set true in populate_from_expense if items exist
```

5. In `populate_from_expense()`:
```rust
if !exp.items.is_empty() {
    self.use_items = true;
    self.items_editor.update(cx, |editor, cx| editor.set_items(&exp.items, cx));
}
```

6. In `save()`:
```rust
// Replace:
//   items: Vec::new(),
// With:
if self.use_items {
    let items = self.items_editor.read(cx).to_expense_items(cx);
    if items.is_empty() {
        self.error = Some("Pridejte alespon jednu polozku".into());
        cx.notify();
        return;
    }
    expense.items = items;
    // Amount will be recalculated by service from items
} else {
    expense.items = Vec::new();
}
```

When `use_items` is true, skip the amount validation (amount=0 is OK because service will calculate from items).

7. In `render()` -- add toggle and items card after Card 2 (Castka a DPH):
```rust
// Toggle button
let toggle_label = if self.use_items {
    "Prepnout na jednoduchou castku"
} else {
    "Pridat polozky"
};
let toggle_btn = div()
    .id("toggle-items-mode")
    .cursor_pointer()
    .px_3()
    .py_2()
    .bg(rgb(ZfColors::SURFACE))
    .border_1()
    .border_color(rgb(ZfColors::BORDER))
    .rounded_md()
    .text_sm()
    .text_color(rgb(ZfColors::ACCENT))
    .on_click(cx.listener(|this, _ev: &ClickEvent, _w, cx| {
        this.use_items = !this.use_items;
        cx.notify();
    }))
    .child(toggle_label);

outer = outer.child(div().flex().child(toggle_btn));

// Items editor (shown only in items mode)
if self.use_items {
    outer = outer.child(render_card(
        "Polozky nakladu",
        div().child(self.items_editor.clone()),
    ));
}
```

When `use_items` is true, the amount input in Card 2 should be visually de-emphasized (opacity 0.5) to indicate it will be auto-calculated. The actual hiding is optional for v1.

---

### Phase 7: Items Display in ExpenseDetailView

**File:** `crates/zfaktury-app/src/views/expense_detail.rs`

**Changes to `render_expense_content()`:**

Add items table after the DPH section (line ~273), before the Notes section:

```rust
// After DPH section, add items table
if !exp.items.is_empty() {
    let mut items_section = div()
        .p_4()
        .bg(rgb(ZfColors::SURFACE))
        .rounded_md()
        .border_1()
        .border_color(rgb(ZfColors::BORDER))
        .flex()
        .flex_col()
        .gap_3();

    // Title
    items_section = items_section.child(
        div()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(ZfColors::TEXT_PRIMARY))
            .child("Polozky"),
    );

    // Column headers
    items_section = items_section.child(
        div()
            .flex()
            .gap_2()
            .pb_2()
            .border_b_1()
            .border_color(rgb(ZfColors::BORDER_SUBTLE))
            .child(div().flex_1().text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).child("Popis"))
            .child(div().w(px(80.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).child("Mnozstvi"))
            .child(div().w(px(60.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).child("Jednotka"))
            .child(div().w(px(100.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).text_right().child("Cena/ks"))
            .child(div().w(px(60.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).text_right().child("DPH %"))
            .child(div().w(px(100.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).text_right().child("DPH"))
            .child(div().w(px(100.0)).text_xs().font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY)).text_right().child("Celkem")),
    );

    // Item rows
    for item in &exp.items {
        let qty_display = format!("{}", item.quantity.to_czk());
        items_section = items_section.child(
            div()
                .flex()
                .gap_2()
                .py_1()
                .child(div().flex_1().text_sm().text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .child(item.description.clone()))
                .child(div().w(px(80.0)).text_sm().text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .child(qty_display))
                .child(div().w(px(60.0)).text_sm().text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .child(item.unit.clone()))
                .child(div().w(px(100.0)).text_sm().text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .text_right().child(format_amount(item.unit_price)))
                .child(div().w(px(60.0)).text_sm().text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .text_right().child(format!("{}%", item.vat_rate_percent)))
                .child(div().w(px(100.0)).text_sm().text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .text_right().child(format_amount(item.vat_amount)))
                .child(div().w(px(100.0)).text_sm().text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .text_right().font_weight(FontWeight::MEDIUM)
                    .child(format_amount(item.total_amount))),
        );
    }

    content = content.child(items_section);
}
```

**Import needed:** `use crate::util::format::format_amount;` (already imported).

---

## File Change Summary

| # | File | Action | Phase |
|---|------|--------|-------|
| 1 | `components/expense_items_editor.rs` | **NEW** ~450 lines | 1 |
| 2 | `components/mod.rs` | ADD `pub mod expense_items_editor` | 1 |
| 3 | `app.rs` | ADD `pending_ocr_result` field + init | 2 |
| 4 | `navigation.rs` | FIX `ExpenseReview(i64)` + path parsing | 3 |
| 5 | `root.rs` | FIX review arm + import arm | 3, 4 |
| 6 | `views/expense_import.rs` | WIRE OCR cache + conditional navigation | 4 |
| 7 | `views/expense_review.rs` | ADD items editor + OCR apply + new constructor | 5 |
| 8 | `views/expense_form.rs` | ADD items toggle/editor | 6 |
| 9 | `views/expense_detail.rs` | ADD items display table | 7 |

## Parallelization Strategy

```
Batch 1 (independent, can run in parallel):
  [Phase 1] ExpenseItemsEditor component (new file)
  [Phase 2] OCR cache on AppServices (app.rs only)
  [Phase 7] Items display in ExpenseDetailView (read-only view)

Batch 2 (depends on Batch 1):
  [Phase 3] Fix Route::ExpenseReview (navigation.rs + root.rs)
            Depends on: Phase 2 (root.rs needs OCR cache)

Batch 3 (depends on Batch 2):
  [Phase 4] Import->Review flow (expense_import.rs + root.rs)
            Depends on: Phase 2, Phase 3
  [Phase 5] ExpenseReviewView (expense_review.rs)
            Depends on: Phase 1, Phase 3
  [Phase 6] ExpenseFormView items (expense_form.rs)
            Depends on: Phase 1
```

## Shared File Conflicts

Files touched by multiple phases (lead must merge manually):
- `root.rs` -- Phase 3 (review arm) + Phase 4 (import arm)
- `navigation.rs` -- Phase 3 only (single owner)
- `app.rs` -- Phase 2 only (single owner)

All view files (`expense_form.rs`, `expense_detail.rs`, `expense_review.rs`, `expense_import.rs`) are each owned by a single phase -- no conflicts.

## Tasklist

- [ ] **Phase 1:** Create `expense_items_editor.rs` with full editor component
- [ ] **Phase 1:** Register in `components/mod.rs`
- [ ] **Phase 2:** Add `pending_ocr_result` to `AppServices` struct and `new()`
- [ ] **Phase 3:** Change `Route::ExpenseReview` to `ExpenseReview(i64)`
- [ ] **Phase 3:** Update `from_path()`, `to_path()`, `label()` in navigation.rs
- [ ] **Phase 3:** Fix `create_content_view()` in root.rs for review route
- [ ] **Phase 4:** Add `pending_ocr_cache` field to `ExpenseImportView`
- [ ] **Phase 4:** Update constructor to accept cache
- [ ] **Phase 4:** Capture OCR result in `process_file()` instead of discarding
- [ ] **Phase 4:** Navigate to Review (OCR success) or Detail (OCR fail/absent)
- [ ] **Phase 4:** Pass cache from root.rs to ExpenseImportView
- [ ] **Phase 5:** Add `items_editor` and `pending_ocr_result` to ExpenseReviewView
- [ ] **Phase 5:** Update constructor to accept `Option<OCRResult>`
- [ ] **Phase 5:** Implement `apply_ocr_result()` method
- [ ] **Phase 5:** Call apply_ocr_result in load callback (after populate_from_expense)
- [ ] **Phase 5:** Read items from editor in `save()`
- [ ] **Phase 5:** Render items editor card in review form
- [ ] **Phase 6:** Add `items_editor` and `use_items` to ExpenseFormView
- [ ] **Phase 6:** Create editor in both constructors
- [ ] **Phase 6:** Populate items in `populate_from_expense()`
- [ ] **Phase 6:** Handle items in `save()` (read from editor, validate non-empty)
- [ ] **Phase 6:** Add toggle button and items card to render
- [ ] **Phase 7:** Add items table section to `render_expense_content()`

## Verification

1. **Build:** `cd rust && cargo build --workspace`
2. **Tests:** `cargo test --workspace`
3. **Clippy:** `cargo clippy --workspace -- -D warnings`
4. **Manual test -- Create expense with items:**
   - Go to Expenses -> New
   - Click "Pridat polozky" toggle
   - Add 2-3 items with descriptions, quantities, prices, VAT rates
   - Verify live totals update correctly
   - Save -> verify items appear in detail view
   - Edit -> verify items are pre-populated
5. **Manual test -- Import with OCR:**
   - Configure OCR in config.toml
   - Go to Import dokladu
   - Select a PDF/image with line items
   - Verify: navigates to Review (not Detail)
   - Verify: form fields populated from OCR
   - Verify: items editor populated from OCR items
   - Click Save -> verify detail shows all data including items
6. **Manual test -- Import without OCR:**
   - Disable OCR in config
   - Import a document
   - Verify: navigates directly to Detail (not Review)
7. **Manual test -- Edit existing expense with items:**
   - Open an expense that has items
   - Click Edit
   - Verify items editor shows existing items
   - Modify an item, save
   - Verify changes persist

## References

| Resource | Path |
|----------|------|
| InvoiceItemsEditor (template) | `crates/zfaktury-app/src/components/invoice_items_editor.rs` |
| Tax investments OCR pattern | `crates/zfaktury-app/src/views/tax_investments.rs:426-534` |
| Expense domain structs | `crates/zfaktury-domain/src/expense.rs` |
| OCR domain structs | `crates/zfaktury-domain/src/ocr.rs` |
| Expense.calculate_totals() | `crates/zfaktury-domain/src/expense.rs` (existing) |
| ExpenseService create/update | `crates/zfaktury-core/src/service/expense_svc.rs` (handles items) |
| Go implementation reference | `internal/handler/expense_handler.go`, `frontend/src/routes/expenses/` |
