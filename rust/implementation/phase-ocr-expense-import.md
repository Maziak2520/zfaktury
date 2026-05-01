# Phase: Expense OCR Import - Implementation Plan

## Overview

Wire the existing OCR infrastructure into the GPUI desktop app and make the ExpenseImport and ExpenseReview views fully functional. The backend (OCR providers, services, domain types) is complete -- the gap is:

1. `rfd` crate is missing (no native file picker)
2. `zfaktury-api` is not a dependency of `zfaktury-app` (no access to OCR providers)
3. `OCRService` is not wired in `AppServices` (line 358-361 of `app.rs` has `None`)
4. `ExpenseImportView` has a placeholder `select_file()` that shows an error message
5. `ExpenseReviewView` always receives `expense_id: 0` (no data passing from import)
6. `Route::ExpenseReview` has no `i64` parameter (cannot identify which expense to review)
7. The two `OCRProvider` traits (`zfaktury_core::service::ocr_svc::OCRProvider` vs `zfaktury_api::ocr::OcrProvider`) have different error types and need an adapter

## Prerequisites

### Dependencies to add

| Crate | Cargo.toml file | Purpose |
|-------|----------------|---------|
| `rfd = "0.15"` | `crates/zfaktury-app/Cargo.toml` | Native file dialogs (GTK on Linux) |
| `zfaktury-api = { path = "../zfaktury-api" }` | `crates/zfaktury-app/Cargo.toml` | Access to `OcrProvider` implementations |

### NixOS dev shell

`rfd` on Linux requires GTK3 (already available since we use GPUI with Wayland/X11). No additional nix packages should be needed, but verify that `pkg-config` can find `gtk+-3.0` inside the dev shell.

### Config changes

No schema changes needed. The `OcrConfig` struct in `zfaktury-config` already has all required fields:

```rust
pub struct OcrConfig {
    pub provider: Option<String>,   // "openai" | "claude" | "openrouter"
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
}
```

Example `config.toml`:
```toml
[ocr]
provider = "claude"
api_key = "sk-ant-..."
model = "claude-sonnet-4-20250514"
```

---

## Step 1: Add `rfd` and `zfaktury-api` dependencies

### File: `crates/zfaktury-app/Cargo.toml`

Add these two lines to `[dependencies]`:

```toml
rfd = "0.15"
zfaktury-api = { path = "../zfaktury-api" }
```

Full `[dependencies]` section after change:

```toml
[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_platform = { git = "https://github.com/zed-industries/zed", features = ["font-kit", "wayland", "x11"] }
zfaktury-core = { path = "../zfaktury-core" }
zfaktury-domain = { path = "../zfaktury-domain" }
zfaktury-gen = { path = "../zfaktury-gen" }
zfaktury-db = { path = "../zfaktury-db" }
zfaktury-config = { path = "../zfaktury-config" }
zfaktury-api = { path = "../zfaktury-api" }
chrono = { version = "0.4", default-features = false, features = ["std", "clock"] }
log = "0.4"
rfd = "0.15"
```

### Verification

```bash
cd rust && cargo check -p zfaktury-app
```

---

## Step 2: Change `Route::ExpenseReview` to carry an expense ID

### Problem

`Route::ExpenseReview` is currently a unit variant with no data. The review view needs to know which expense to load and display. The import flow creates a skeleton expense, then navigates to review with the expense's ID.

### File: `crates/zfaktury-app/src/navigation.rs`

Change the enum variant:

```rust
// Before:
ExpenseReview,

// After:
ExpenseReview(i64),
```

Update the `from_path` match -- remove the static path (review is only reachable via navigation, not direct URL):

```rust
// Before:
"/expenses/review" => Some(Route::ExpenseReview),

// After:
// Remove this line entirely. ExpenseReview is now only reachable via
// NavigateEvent from ExpenseImportView, not via URL.
```

Add dynamic route parsing:

```rust
// In parse_dynamic_route, add this arm:
["expenses", "review", id] => id.parse().ok().map(Route::ExpenseReview),
```

Update the `label` match:

```rust
// Before:
Route::ExpenseReview => "Kontrola nakladu",
// After:
Route::ExpenseReview(_) => "Kontrola nakladu",
```

### File: `crates/zfaktury-app/src/sidebar.rs`

Update the pattern match that groups expense-related routes. Change:

```rust
| Route::ExpenseReview
```

to:

```rust
| Route::ExpenseReview(_)
```

### File: `crates/zfaktury-app/src/root.rs`

Update the `create_content_view` match arm:

```rust
// Before:
Route::ExpenseReview => {
    let svc = services.expenses.clone();
    ContentView::ExpenseReview(cx.new(|cx| ExpenseReviewView::new(svc, 0, cx)))
}

// After:
Route::ExpenseReview(id) => {
    let svc = services.expenses.clone();
    let id = *id;
    ContentView::ExpenseReview(cx.new(|cx| ExpenseReviewView::new(svc, id, cx)))
}
```

---

## Step 3: Create OCR provider adapter and wire into `AppServices`

### Problem: Two incompatible OCR provider traits

The core crate defines:
```rust
// zfaktury_core::service::ocr_svc
pub trait OCRProvider: Send + Sync {
    fn process_image(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError>;
}
```

The API crate defines:
```rust
// zfaktury_api::ocr
pub trait OcrProvider: Send + Sync {
    fn process_image(&self, image_data: &[u8], content_type: &str) -> Result<OCRResult, ApiError>;
    fn name(&self) -> &str;
}
```

The concrete providers (`AnthropicProvider`, `OpenAIProvider`) implement `zfaktury_api::ocr::OcrProvider`, which returns `Result<OCRResult, ApiError>`. But `OCRService` expects `zfaktury_core::service::ocr_svc::OCRProvider`, which returns `Result<OCRResult, DomainError>`.

### Solution: Adapter in `app.rs`

Create a thin adapter struct in `app.rs` that wraps an `Arc<dyn zfaktury_api::ocr::OcrProvider>` and implements `zfaktury_core::service::ocr_svc::OCRProvider`.

### File: `crates/zfaktury-app/src/app.rs`

Add imports:

```rust
use zfaktury_api::ocr::{AnthropicProvider, OcrProvider as ApiOcrProvider, OpenAIProvider};
use zfaktury_config::Config;
use zfaktury_core::service::ocr_svc::{OCRProvider, OCRService};
```

Add the adapter struct (before or after `AppServices`):

```rust
/// Adapter bridging `zfaktury_api::ocr::OcrProvider` (returns ApiError)
/// to `zfaktury_core::service::ocr_svc::OCRProvider` (returns DomainError).
struct OcrProviderAdapter {
    inner: Arc<dyn ApiOcrProvider>,
}

impl OCRProvider for OcrProviderAdapter {
    fn process_image(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
        self.inner
            .process_image(data, content_type)
            .map_err(|e| {
                log::error!("OCR provider error: {e}");
                DomainError::InvalidInput
            })
    }
}
```

Add a factory function:

```rust
/// Create an OCR provider from config, if configured.
/// Returns None if OCR is not configured or the API key is missing.
fn create_ocr_provider(config: &Config) -> Option<Arc<dyn OCRProvider>> {
    let ocr_config = config.ocr.as_ref()?;
    let api_key = ocr_config.api_key.as_deref().filter(|k| !k.is_empty())?;
    let model = ocr_config.model.as_deref().unwrap_or("");

    let provider_name = ocr_config.provider.as_deref().unwrap_or("claude");

    let api_provider: Arc<dyn ApiOcrProvider> = match provider_name {
        "openai" => {
            let mut p = OpenAIProvider::new_openai(api_key, model);
            if let Some(url) = ocr_config.base_url.as_deref().filter(|u| !u.is_empty()) {
                p = p.with_base_url(url);
            }
            Arc::new(p)
        }
        "openrouter" => {
            let mut p = OpenAIProvider::new_openrouter(api_key, model);
            if let Some(url) = ocr_config.base_url.as_deref().filter(|u| !u.is_empty()) {
                p = p.with_base_url(url);
            }
            Arc::new(p)
        }
        "claude" | "anthropic" | _ => {
            let mut p = AnthropicProvider::new(api_key, model);
            if let Some(url) = ocr_config.base_url.as_deref().filter(|u| !u.is_empty()) {
                p = p.with_base_url(url);
            }
            Arc::new(p)
        }
    };

    log::info!("OCR provider configured: {}", provider_name);
    Some(Arc::new(OcrProviderAdapter { inner: api_provider }))
}
```

### Modify `AppServices`

Add field:

```rust
pub struct AppServices {
    // ... existing fields ...
    pub ocr: Option<Arc<OCRService>>,
}
```

Change `AppServices::new` signature to accept config:

```rust
pub fn new(db_path: &Path, data_dir: &Path, config: &Config) -> Result<Self> {
```

Wire OCR service (replace the `ImportService` block at the end of `new`):

```rust
// OCRService (optional, depends on config)
let ocr_provider = create_ocr_provider(config);
let ocr = ocr_provider.map(|provider| {
    Arc::new(OCRService::new(provider, documents.clone()))
});

// ImportService (depends on ExpenseService, DocumentService, OCRService)
let import = Arc::new(ImportService::new(
    expenses.clone(),
    documents.clone(),
    ocr.clone(),
));

Ok(Self {
    // ... existing fields ...
    ocr,
    import,
})
```

### File: `crates/zfaktury-app/src/main.rs`

Update `cmd_gui` to load config once and pass it to `AppServices::new`:

```rust
fn cmd_gui(cli: Cli) {
    let config = match zfaktury_config::Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };

    let db_path = if let Some(ref db) = cli.db {
        db.clone()
    } else {
        config.database_path()
    };
    ensure_parent_dir(&db_path);

    let data_dir = if let Some(ref db) = cli.db {
        db.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        config.data_dir()
    };
    ensure_parent_dir_of(&data_dir);

    let services = match AppServices::new(&db_path, &data_dir, &config) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Error initializing application: {e}");
            std::process::exit(1);
        }
    };
    // ... rest unchanged ...
}
```

Also update `resolve_db_path` and `resolve_data_dir` -- or better, remove them in favor of the inlined config loading above.

---

## Step 4: Update `ExpenseImportView` -- file picker + OCR flow

### Current state

- Has `import_service` and `document_service` fields
- `select_file()` shows a hardcoded error message ("rfd not available")
- `process_file()` is `#[allow(dead_code)]`, creates skeleton expense and navigates to detail
- No OCR integration at all
- Renders an OCR status badge that always says "not configured"

### Target state

- Add `ocr_service: Option<Arc<OCRService>>` field
- `select_file()` uses `rfd::AsyncFileDialog` to open a native file picker
- After file selection: validate size/type, create skeleton expense, store document, run OCR if available
- If OCR succeeds: navigate to `Route::ExpenseReview(expense_id)` with data
- If OCR fails or not configured: navigate to `Route::ExpenseDetail(expense_id)`
- OCR status badge dynamically shows green/yellow based on `ocr_service.is_some()`

### File: `crates/zfaktury-app/src/views/expense_import.rs`

#### Imports to add

```rust
use std::path::PathBuf;
use zfaktury_core::service::ocr_svc::OCRService;
```

#### Struct changes

```rust
pub struct ExpenseImportView {
    import_service: Arc<ImportService>,
    document_service: Arc<DocumentService>,
    ocr_service: Option<Arc<OCRService>>,

    // State
    processing: bool,
    error: Option<String>,
    success_message: Option<String>,
    selected_filename: Option<String>,
}
```

#### Constructor change

```rust
pub fn new(
    import_service: Arc<ImportService>,
    document_service: Arc<DocumentService>,
    ocr_service: Option<Arc<OCRService>>,
    _cx: &mut Context<Self>,
) -> Self {
    Self {
        import_service,
        document_service,
        ocr_service,
        processing: false,
        error: None,
        success_message: None,
        selected_filename: None,
    }
}
```

#### Replace `select_file` method

```rust
fn select_file(&mut self, cx: &mut Context<Self>) {
    if self.processing {
        return;
    }

    self.processing = true;
    self.error = None;
    self.success_message = None;
    cx.notify();

    let import_svc = self.import_service.clone();
    let doc_svc = self.document_service.clone();
    let ocr_svc = self.ocr_service.clone();

    cx.spawn(async move |this, cx| {
        // 1. Open native file dialog
        let file = rfd::AsyncFileDialog::new()
            .add_filter("Doklady", &["pdf", "jpg", "jpeg", "png"])
            .set_title("Vyberte doklad")
            .pick_file()
            .await;

        let file = match file {
            Some(f) => f,
            None => {
                // User cancelled the dialog
                this.update(cx, |this, cx| {
                    this.processing = false;
                    cx.notify();
                }).ok();
                return;
            }
        };

        let path = file.path().to_path_buf();
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "document".to_string());

        // 2. Read file data (on background executor)
        let filename_clone = filename.clone();
        let read_result = cx
            .background_executor()
            .spawn(async move {
                let data = std::fs::read(&path)
                    .map_err(|e| format!("Nelze precist soubor: {e}"))?;

                // Validate size (20 MB max)
                if data.len() > 20 * 1024 * 1024 {
                    return Err("Soubor je prilis velky (max 20 MB)".to_string());
                }

                // Determine content type from extension
                let content_type = match path.extension().and_then(|e| e.to_str()) {
                    Some("pdf") => "application/pdf",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("png") => "image/png",
                    _ => return Err("Nepodporovany format souboru".to_string()),
                };

                Ok::<(Vec<u8>, String), String>((data, content_type.to_string()))
            })
            .await;

        let (data, content_type) = match read_result {
            Ok(pair) => pair,
            Err(msg) => {
                this.update(cx, |this, cx| {
                    this.processing = false;
                    this.error = Some(msg);
                    cx.notify();
                }).ok();
                return;
            }
        };

        // 3. Create skeleton expense + store document (on background executor)
        let import_svc2 = import_svc.clone();
        let doc_svc2 = doc_svc.clone();
        let filename2 = filename_clone.clone();
        let content_type2 = content_type.clone();
        let data2 = data.clone();

        let create_result = cx
            .background_executor()
            .spawn(async move {
                // Create skeleton expense
                let expense = import_svc2.create_skeleton_expense(&filename2)?;

                // Create document record
                let data_dir = doc_svc2.data_dir().to_string();
                let doc_dir = std::path::Path::new(&data_dir).join("documents");
                std::fs::create_dir_all(&doc_dir)
                    .map_err(|e| zfaktury_domain::DomainError::InvalidInput)?;

                let storage_filename = format!("{}_{}", expense.id, filename2);
                let storage_path = doc_dir.join(&storage_filename);
                std::fs::write(&storage_path, &data2)
                    .map_err(|e| zfaktury_domain::DomainError::InvalidInput)?;

                let mut doc = zfaktury_domain::ExpenseDocument {
                    id: 0,
                    expense_id: expense.id,
                    filename: filename2,
                    content_type: content_type2,
                    storage_path: storage_path.to_string_lossy().to_string(),
                    size: data2.len() as i64,
                    created_at: chrono::Local::now().naive_local(),
                    deleted_at: None,
                };
                doc_svc2.create_record(&mut doc)?;

                Ok::<(zfaktury_domain::Expense, zfaktury_domain::ExpenseDocument), zfaktury_domain::DomainError>((expense, doc))
            })
            .await;

        let (expense, document) = match create_result {
            Ok(pair) => pair,
            Err(e) => {
                this.update(cx, |this, cx| {
                    this.processing = false;
                    this.error = Some(format!("Chyba pri importu: {e}"));
                    cx.notify();
                }).ok();
                return;
            }
        };

        // 4. Run OCR if available
        if let Some(ref ocr) = ocr_svc {
            let ocr_clone = ocr.clone();
            let ocr_data = data;
            let ocr_content_type = content_type;

            let ocr_result = cx
                .background_executor()
                .spawn(async move {
                    ocr_clone.provider_process_image(&ocr_data, &ocr_content_type)
                })
                .await;

            match ocr_result {
                Ok(_ocr_data) => {
                    // OCR succeeded -- navigate to review page
                    this.update(cx, |this, cx| {
                        this.processing = false;
                        this.success_message = Some(format!(
                            "Doklad '{}' importovan, OCR dokonceno",
                            filename_clone
                        ));
                        cx.emit(NavigateEvent(Route::ExpenseReview(expense.id)));
                        cx.notify();
                    }).ok();
                    return;
                }
                Err(e) => {
                    log::warn!("OCR failed for document {}: {e}", document.id);
                    // Fall through to navigate to detail instead
                }
            }
        }

        // 5. No OCR or OCR failed -- navigate to expense detail for manual edit
        this.update(cx, |this, cx| {
            this.processing = false;
            this.success_message = Some(format!(
                "Doklad '{}' importovan",
                filename_clone
            ));
            cx.emit(NavigateEvent(Route::ExpenseDetail(expense.id)));
            cx.notify();
        }).ok();
    })
    .detach();
}
```

**Note on OCR call:** The `OCRService::process_document` currently has a bug -- it tries to read the file from the document's storage_path but returns `DomainError::InvalidInput` as a stub. Instead of going through `process_document()`, we should call the provider directly with the bytes we already have in memory. Two options:

**Option A:** Add a `process_bytes` method to `OCRService`:
```rust
// In ocr_svc.rs
pub fn process_bytes(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
    self.provider.process_image(data, content_type)
}
```

**Option B:** Make `process_document` actually read the file from disk using `storage_path`.

**Recommendation:** Option A is simpler and avoids the file-read responsibility debate. Add `process_bytes` to `OCRService` and call it from the view.

#### Update `render_ocr_status`

```rust
fn render_ocr_status(&self) -> Div {
    if self.ocr_service.is_some() {
        // OCR is configured -- green indicator
        div()
            .p_3()
            .bg(rgb(ZfColors::STATUS_GREEN_BG))
            .rounded_md()
            .border_1()
            .border_color(rgb(ZfColors::BORDER))
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg(rgb(ZfColors::STATUS_GREEN)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .child("OCR aktivni - doklad bude automaticky rozpoznan"),
            )
    } else {
        // OCR not configured -- yellow warning
        div()
            .p_3()
            .bg(rgb(ZfColors::STATUS_YELLOW_BG))
            .rounded_md()
            .border_1()
            .border_color(rgb(ZfColors::BORDER))
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg(rgb(ZfColors::STATUS_YELLOW)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .child("OCR neni nastaveno - doklad bude importovan bez rozpoznani"),
            )
    }
}
```

#### Update upload area button description

When processing, show the selected filename:

```rust
// In render_upload_area, update the processing branch:
if self.processing {
    let label = if let Some(ref name) = self.selected_filename {
        format!("Zpracovani: {}...", name)
    } else {
        "Zpracovani...".to_string()
    };
    area = area.child(
        div()
            .px_4()
            .py_2()
            .bg(rgb(ZfColors::ACCENT))
            .rounded_md()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(0xffffff))
            .opacity(0.5)
            .child(label),
    );
}
```

#### Remove dead code

Remove the old `process_file` method (its logic is now inlined in `select_file`). Remove the `#[allow(dead_code)]` attribute.

---

## Step 5: Add `process_bytes` to `OCRService`

### File: `crates/zfaktury-core/src/service/ocr_svc.rs`

Add a new public method:

```rust
impl OCRService {
    // ... existing new() and process_document() ...

    /// Process raw image/PDF bytes through the OCR provider.
    /// This is the preferred method when the caller already has the file data in memory.
    pub fn process_bytes(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
        if data.is_empty() {
            return Err(DomainError::InvalidInput);
        }
        let supported = ["image/jpeg", "image/png", "application/pdf"];
        if !supported.contains(&content_type) {
            return Err(DomainError::InvalidInput);
        }
        self.provider.process_image(data, content_type)
    }
}
```

Remove `#[allow(dead_code)]` from the struct definition since it will now be used.

---

## Step 6: Update `ExpenseReviewView` -- pre-populate from OCR

### Current state

- Takes `expense_service: Arc<ExpenseService>` and `expense_id: i64`
- Loads existing expense from DB and populates form
- Has form fields: description, invoice_number, issue_date, total_amount, vat_amount, vat_rate, currency
- Save writes back to the same expense

### Target changes

The review page currently works fine for reviewing an expense after import. The key gap is that after OCR, the skeleton expense has placeholder data (filename as description, 1 CZK amount). The OCR results need to be written to the expense before the review page loads.

### Strategy: Update expense with OCR data before navigating

Instead of modifying `ExpenseReviewView` to accept OCR data directly (which would require complex state passing between views), update the expense record with OCR data inside `ExpenseImportView.select_file()` before emitting the navigation event. The review page then loads the already-updated expense.

In the `select_file` method (Step 4 code), after OCR succeeds, add an expense update:

```rust
Ok(ocr_result) => {
    // Update expense with OCR data before navigating
    let expense_svc = import_svc.expenses_ref().clone(); // need to expose this
    let expense_id = expense.id;
    let update_result = cx
        .background_executor()
        .spawn(async move {
            // We need access to the expense to update it.
            // Use import_svc or expense_svc to update.
            let mut exp = expense;
            exp.description = if ocr_result.description.is_empty() {
                filename_clone.clone()
            } else {
                ocr_result.description.clone()
            };
            exp.expense_number = ocr_result.invoice_number.clone();
            exp.amount = ocr_result.total_amount;
            exp.vat_amount = ocr_result.vat_amount;
            exp.vat_rate_percent = ocr_result.vat_rate_percent;
            exp.currency_code = if ocr_result.currency_code.is_empty() {
                "CZK".to_string()
            } else {
                ocr_result.currency_code.clone()
            };
            if let Ok(date) = chrono::NaiveDate::parse_from_str(
                &ocr_result.issue_date, "%Y-%m-%d"
            ) {
                exp.issue_date = date;
            }
            expense_svc.update(&mut exp)?;
            Ok::<i64, zfaktury_domain::DomainError>(exp.id)
        })
        .await;
    // Navigate to review regardless of update result
    this.update(cx, |this, cx| {
        this.processing = false;
        cx.emit(NavigateEvent(Route::ExpenseReview(expense_id)));
        cx.notify();
    }).ok();
}
```

**Alternative approach if `ImportService` doesn't expose `expenses`:** Pass `ExpenseService` as an additional dependency to `ExpenseImportView`, OR add a method to `ImportService` like:

```rust
/// Update expense with OCR results.
pub fn apply_ocr_results(&self, expense_id: i64, ocr: &OCRResult) -> Result<(), DomainError> {
    let mut expense = self.expenses.get_by_id(expense_id)?;
    if !ocr.description.is_empty() {
        expense.description = ocr.description.clone();
    }
    expense.expense_number = ocr.invoice_number.clone();
    expense.amount = ocr.total_amount;
    expense.vat_amount = ocr.vat_amount;
    expense.vat_rate_percent = ocr.vat_rate_percent;
    if !ocr.currency_code.is_empty() {
        expense.currency_code = ocr.currency_code.clone();
    }
    if let Ok(date) = chrono::NaiveDate::parse_from_str(&ocr.issue_date, "%Y-%m-%d") {
        expense.issue_date = date;
    }
    self.expenses.update(&mut expense)?;
    Ok(())
}
```

**Recommendation:** Add `apply_ocr_results` to `ImportService` (cleaner separation of concerns).

### File: `crates/zfaktury-core/src/service/import_svc.rs`

Add the method above to `ImportService`. Also add `use zfaktury_domain::OCRResult;` at the top.

---

## Step 7: Update `root.rs` -- pass OCR service to import view

### File: `crates/zfaktury-app/src/root.rs`

Update the `ExpenseImport` routing:

```rust
// Before:
Route::ExpenseImport => {
    let import_svc = services.import.clone();
    let doc_svc = services.documents.clone();
    ContentView::ExpenseImport(
        cx.new(|cx| ExpenseImportView::new(import_svc, doc_svc, cx)),
    )
}

// After:
Route::ExpenseImport => {
    let import_svc = services.import.clone();
    let doc_svc = services.documents.clone();
    let ocr_svc = services.ocr.clone();
    ContentView::ExpenseImport(
        cx.new(|cx| ExpenseImportView::new(import_svc, doc_svc, ocr_svc, cx)),
    )
}
```

---

## Step 8: Add confidence display to `ExpenseReviewView`

### File: `crates/zfaktury-app/src/views/expense_review.rs`

This is a nice-to-have enhancement. Since the OCR confidence is written to the expense (or could be stored elsewhere), the review page could show a confidence badge. However, `Expense` does not have a `confidence` field.

**Decision:** Skip confidence display for now. The user reviews and edits the form fields directly. Confidence display can be added later when the `Expense` domain struct gets an `ocr_confidence` field.

No changes needed to `expense_review.rs` -- it already loads and displays expense data correctly.

---

## Files to modify (exact list)

| File | Change type | Description |
|------|------------|-------------|
| `crates/zfaktury-app/Cargo.toml` | MODIFY | Add `rfd = "0.15"` and `zfaktury-api` dependency |
| `crates/zfaktury-app/src/navigation.rs` | MODIFY | Change `ExpenseReview` to `ExpenseReview(i64)`, update match arms |
| `crates/zfaktury-app/src/sidebar.rs` | MODIFY | Update pattern match for `ExpenseReview(_)` |
| `crates/zfaktury-app/src/root.rs` | MODIFY | Pass OCR service to import view, fix review routing with ID |
| `crates/zfaktury-app/src/main.rs` | MODIFY | Load config once and pass to `AppServices::new` |
| `crates/zfaktury-app/src/app.rs` | MODIFY | Add OCR adapter, factory, wire OCRService, accept Config param |
| `crates/zfaktury-app/src/views/expense_import.rs` | MODIFY | Add rfd file picker, OCR flow, dynamic OCR status |
| `crates/zfaktury-core/src/service/ocr_svc.rs` | MODIFY | Add `process_bytes` method |
| `crates/zfaktury-core/src/service/import_svc.rs` | MODIFY | Add `apply_ocr_results` method |

## Files NOT modified

| File | Reason |
|------|--------|
| `crates/zfaktury-app/src/views/expense_review.rs` | Already works -- loads expense data from DB and shows editable form |
| `crates/zfaktury-domain/src/ocr.rs` | Domain types are complete |
| `crates/zfaktury-api/src/ocr.rs` | Providers are complete |
| `crates/zfaktury-config/src/lib.rs` | OcrConfig already defined |

---

## Implementation order

The steps have dependencies:

```
Step 1 (deps)  ─────────────────────────────────────────┐
Step 2 (Route::ExpenseReview(i64)) ──┐                  │
Step 5 (process_bytes on OCRService) ┤                  │
Step 6 (apply_ocr_results on ImportService) ┤           │
                                     ├── Step 3 (wire)  │
                                     │                  │
                                     └── Step 4 (import view) ← Step 7 (root.rs)
```

Practical order:
1. Step 1 -- add dependencies (Cargo.toml)
2. Step 2 -- change Route enum (navigation.rs, sidebar.rs)
3. Step 5 -- add `process_bytes` (ocr_svc.rs)
4. Step 6 -- add `apply_ocr_results` (import_svc.rs)
5. Step 3 -- wire OCR in AppServices (app.rs, main.rs)
6. Step 4 -- update ExpenseImportView (expense_import.rs)
7. Step 7 -- update root.rs routing
8. `cargo build --workspace` and fix compile errors
9. `cargo test --workspace` and fix test failures
10. Manual test: run app without OCR config, verify file picker works, expense is created
11. Manual test: run app with OCR config, verify OCR runs and review page shows data

---

## Testing plan

### Unit tests

1. **`ocr_svc.rs` -- `process_bytes`**
   - Test with empty data returns `DomainError::InvalidInput`
   - Test with unsupported content type returns error
   - Test with valid data calls provider (mock OCRProvider)

2. **`import_svc.rs` -- `apply_ocr_results`**
   - Test updates expense fields from OCRResult
   - Test with empty OCR fields preserves original values
   - Test with invalid date string keeps original date

3. **`app.rs` -- `create_ocr_provider`**
   - Test returns None when config.ocr is None
   - Test returns None when api_key is empty
   - Test returns Some for "openai" provider
   - Test returns Some for "claude" provider
   - Test "anthropic" maps to AnthropicProvider

### Integration tests

1. **Full flow without OCR:** Select file -> skeleton expense created -> navigates to expense detail
2. **Full flow with OCR:** Select file -> expense created -> OCR runs -> expense updated -> navigates to review
3. **OCR failure graceful:** Select file -> expense created -> OCR fails -> navigates to expense detail (not review)

### Manual testing checklist

- [ ] File picker opens and shows correct file filters (PDF, JPG, PNG)
- [ ] Cancelling file picker returns to import page without error
- [ ] File >20MB shows error message
- [ ] Unsupported file type shows error message
- [ ] Without OCR config: file imported, navigates to expense detail
- [ ] With OCR config: file imported, OCR runs, navigates to review page
- [ ] Review page shows OCR-extracted data in form fields
- [ ] Save on review page updates expense and navigates to detail
- [ ] Discard on review page deletes expense and navigates to list
- [ ] OCR status badge shows green when configured, yellow when not
- [ ] Back button on import page navigates to expense list

---

## Risks and mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `rfd` 0.15 doesn't compile with our GPUI revision | Blocks file picker | Try `rfd` 0.14 or 0.13. If none work, use a CLI fallback (`std::process::Command` to call `zenity --file-selection`) |
| `rfd::AsyncFileDialog` blocks the GPUI event loop | UI freezes during file selection | `rfd::AsyncFileDialog` uses async/await which should work inside `cx.spawn`. If it blocks, try `rfd::FileDialog` on a `background_executor` thread (blocking API) |
| OCR API call takes >30s, user thinks app is frozen | Bad UX | Show a spinner with "OCR zpracovani..." text. The `reqwest::blocking::Client` has a 25-30s timeout. Add a cancel button in future iteration |
| `OcrProviderAdapter` loses error detail (maps all errors to `DomainError::InvalidInput`) | Hard to debug OCR failures | Log the original `ApiError` at `error!` level before converting. Consider adding a `DomainError::ExternalService(String)` variant in the future |
| GTK file dialog requires GTK3 runtime | Won't work on minimal Linux installs | Already required by GPUI (Wayland/X11 backend needs GTK). Document in README |
| Concurrent file dialog opens if user double-clicks | Two dialogs open | Already guarded: `if self.processing { return; }` prevents re-entry |
