# Phase: Investment Document OCR - Implementation Plan

## Overview

Adding a "Dokumenty" tab to `TaxInvestmentsView` for uploading broker statements (PDF/CSV/images) and running AI extraction to auto-create capital income entries and security transactions.

The existing view at `zfaktury-app/src/views/tax_investments.rs` (2215 lines) already has full CRUD for capital income entries and security transactions across two tabs (`CapitalIncome`, `SecurityTransactions`). This phase adds a third `Documents` tab, wires the `InvestmentDocumentService` and `OcrProvider`, and implements the file upload + extraction pipeline.

## Current State Analysis

### What exists and works

1. **TaxInvestmentsView** (`zfaktury-app/src/views/tax_investments.rs`)
   - 2-tab system: `InvestmentTab::CapitalIncome`, `InvestmentTab::SecurityTransactions`
   - Full inline CRUD for both entity types (new/edit/delete with ConfirmDialog)
   - Year selector with `change_year()` + `load_data()`
   - Summary card always visible at top
   - Constructor takes only `Arc<InvestmentIncomeService>`

2. **InvestmentDocumentService** (`zfaktury-core/src/service/investment_document_svc.rs`)
   - `create_record(&mut InvestmentDocument)` -- validates year/filename, creates DB record
   - `get_by_id(id)` -- fetches document
   - `list_by_year(year)` -- lists documents for a year
   - `delete(id)` -- cascading delete (linked capital entries + security transactions + document itself)
   - `data_dir()` -- returns data directory path for file storage
   - Already wired in `AppServices` as `investment_documents: Arc<InvestmentDocumentService>`

3. **InvestmentDocumentRepo** trait (`zfaktury-core/src/repository/traits.rs:282-288`)
   - `create`, `get_by_id`, `list_by_year`, `delete`
   - `update_extraction(id, status, extraction_error)` -- updates extraction status

4. **CapitalIncomeRepo** and **SecurityTransactionRepo** traits
   - Both have `delete_by_document_id(document_id)` -- used by cascading delete
   - Both have `list_by_document_id(document_id)` -- useful for showing linked entries

5. **OcrProvider trait** (`zfaktury-api/src/ocr.rs:51-56`)
   - `process_image(&self, image_data: &[u8], content_type: &str) -> Result<OCRResult>`
   - Implementations: `AnthropicProvider`, `OpenAIProvider` (OpenAI/OpenRouter)
   - Currently processes invoices with a Czech system prompt returning `OCRResult` (vendor, amounts, VAT)

6. **OcrConfig** (`zfaktury-config/src/lib.rs:117-123`)
   - `provider: Option<String>` -- "anthropic", "openai", "openrouter"
   - `api_key: Option<String>`
   - `model: Option<String>`
   - `base_url: Option<String>`

7. **Domain types** (`zfaktury-domain/src/investment.rs`)
   - `InvestmentDocument` -- id, year, platform, filename, content_type, storage_path, size, extraction_status, extraction_error, timestamps
   - `Platform` -- Portu, Zonky, Trading212, Revolut, Other
   - `ExtractionStatus` -- Pending, Extracted, Failed
   - `CapitalIncomeEntry` -- has `document_id: Option<i64>`
   - `SecurityTransaction` -- has `document_id: Option<i64>`

8. **DomainError** (`zfaktury-domain/src/errors.rs`)
   - Does NOT have `ExternalError` variant (the RFC references it but it doesn't exist)
   - Available: NotFound, InvalidInput, PaidInvoice, NoItems, DuplicateNumber, etc.

### What is missing

1. **Documents tab** in TaxInvestmentsView -- no `InvestmentTab::Documents` variant
2. **OCR provider wiring** in `AppServices` -- `ocr_provider` field does not exist, config is not used
3. **`rfd` crate** not in `zfaktury-app/Cargo.toml` -- file picker placeholder in `expense_import.rs`
4. **`zfaktury-api` dependency** not in `zfaktury-app/Cargo.toml` -- needed for `OcrProvider` trait
5. **InvestmentExtractionService** does not exist -- no service to parse OCR output into investment domain types
6. **Investment-specific OCR prompt** -- current prompt extracts invoice fields, not broker statement data
7. **`ExternalError` DomainError variant** -- needed for OCR failure propagation

## Prerequisites

- `rfd` crate (version 0.15) -- also needed by expense OCR phase, must be added to `zfaktury-app/Cargo.toml`
- `zfaktury-api` crate dependency in `zfaktury-app/Cargo.toml` -- for `OcrProvider` trait
- `zfaktury-config` is already a dependency of `zfaktury-app`

## Step 1: Add `ExternalError` variant to DomainError

**File:** `zfaktury-domain/src/errors.rs`

Add a new variant to handle OCR/external API errors:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    // ... existing variants ...

    #[error("external service error: {0}")]
    ExternalError(String),
}
```

This is needed because OCR provider errors (API errors, parsing failures) need to bubble up through the service layer as `DomainError`.

## Step 2: Add `rfd` and `zfaktury-api` dependencies

**File:** `zfaktury-app/Cargo.toml`

```toml
[dependencies]
# ... existing ...
rfd = "0.15"
zfaktury-api = { path = "../zfaktury-api" }
```

**Note:** If the expense OCR phase runs first, `rfd` may already be added. Check before duplicating.

## Step 3: Wire OCR provider in AppServices

**File:** `zfaktury-app/src/app.rs`

### 3.1 Change `AppServices::new()` signature

Currently: `pub fn new(db_path: &Path, data_dir: &Path) -> Result<Self>`

New: `pub fn new(db_path: &Path, data_dir: &Path, config: &zfaktury_config::Config) -> Result<Self>`

### 3.2 Add `ocr_provider` field to AppServices

```rust
pub struct AppServices {
    // ... existing fields ...
    pub ocr_provider: Option<Arc<dyn zfaktury_api::ocr::OcrProvider + Send + Sync>>,
}
```

### 3.3 Construct OCR provider from config

After existing service wiring, add:

```rust
use zfaktury_api::ocr::{AnthropicProvider, OcrProvider, OpenAIProvider};

// OCR provider (optional, based on config)
let ocr_provider: Option<Arc<dyn OcrProvider + Send + Sync>> =
    config.ocr.as_ref().and_then(|ocr_cfg| {
        let api_key = ocr_cfg.api_key.as_deref().unwrap_or("");
        if api_key.is_empty() {
            return None;
        }
        let model = ocr_cfg.model.as_deref().unwrap_or("");
        match ocr_cfg.provider.as_deref().unwrap_or("") {
            "anthropic" | "claude" => {
                let mut p = AnthropicProvider::new(api_key, model);
                if let Some(ref url) = ocr_cfg.base_url {
                    p = p.with_base_url(url);
                }
                Some(Arc::new(p) as Arc<dyn OcrProvider + Send + Sync>)
            }
            "openai" => {
                let mut p = OpenAIProvider::new_openai(api_key, model);
                if let Some(ref url) = ocr_cfg.base_url {
                    p = p.with_base_url(url);
                }
                Some(Arc::new(p) as Arc<dyn OcrProvider + Send + Sync>)
            }
            "openrouter" => {
                let mut p = OpenAIProvider::new_openrouter(api_key, model);
                if let Some(ref url) = ocr_cfg.base_url {
                    p = p.with_base_url(url);
                }
                Some(Arc::new(p) as Arc<dyn OcrProvider + Send + Sync>)
            }
            _ => None,
        }
    });
```

### 3.4 Update callers

**File:** `zfaktury-app/src/main.rs` (line 138)

Change from:
```rust
let services = match AppServices::new(&db_path, &data_dir) {
```

To:
```rust
let config = zfaktury_config::Config::load().unwrap_or_default();
let services = match AppServices::new(&db_path, &data_dir, &config) {
```

The config is already loaded earlier for `resolve_db_path`/`resolve_data_dir` -- reuse that or load once.

## Step 4: Add investment-specific OCR extraction function

**File:** `zfaktury-api/src/ocr.rs`

Add a new public function and types for investment document extraction. The existing `OcrProvider` trait stays generic (it sends image data to the API), but we add a new system prompt and response parser for investment documents.

### 4.1 Investment OCR system prompt

```rust
const INVESTMENT_SYSTEM_PROMPT: &str = r#"Jsi asistent pro zpracovani brokerskych vypisu a investicnich dokumentu. Analyzuj dokument a extrahuj strukturovana data.

Vrat POUZE platny JSON objekt (bez markdown, bez komentaru) s nasledujici strukturou:
{
  "platform": "nazev platformy (portu/zonky/trading212/revolut/other)",
  "capital_income_entries": [
    {
      "category": "dividend_cz|dividend_foreign|interest|coupon|fund_distribution|other",
      "description": "popis prijmu",
      "income_date": "YYYY-MM-DD",
      "gross_amount": celkova brutto castka (cislo, napr. 1234.56),
      "withheld_tax_cz": srazena dan v CZ (cislo),
      "withheld_tax_foreign": srazena dan v zahranici (cislo),
      "country_code": "dvoupismenny ISO kod zeme (napr. CZ, US, DE)",
      "needs_declaring": true/false
    }
  ],
  "security_transactions": [
    {
      "asset_type": "stock|etf|bond|fund|crypto|other",
      "asset_name": "nazev aktiva",
      "isin": "ISIN kod (pokud dostupny)",
      "transaction_type": "buy|sell",
      "transaction_date": "YYYY-MM-DD",
      "quantity": pocet kusu (cislo, napr. 1.5),
      "unit_price": jednotkova cena (cislo),
      "total_amount": celkova castka (cislo),
      "fees": poplatky (cislo),
      "currency_code": "kod meny (CZK, EUR, USD)",
      "exchange_rate": kurz k CZK (cislo, 1.0 pro CZK)
    }
  ],
  "confidence": mira jistoty 0.0-1.0
}

Dulezite:
- Castky jako desetinna cisla (napr. 1234.56)
- Pokud udaj neni v dokumentu, pouzij prazdny retezec pro textova pole, 0 pro cisla
- Datum vzdy ve formatu YYYY-MM-DD
- Pokud nelze rozpoznat typ transakce, pouzij "other"
- Pro kazdy radek v dokumentu vytvor samostatny zaznam"#;

const INVESTMENT_USER_PROMPT: &str = "Analyzuj tento brokersky vypis nebo investicni dokument a extrahuj vsechny kapitalove prijmy (dividendy, uroky, kupony) a obchody s cennymi papiry do JSON formatu podle zadane struktury.";
```

### 4.2 Investment extraction response types

```rust
#[derive(Debug, Clone)]
pub struct InvestmentExtractionResult {
    pub platform: String,
    pub capital_entries: Vec<ExtractedCapitalEntry>,
    pub security_transactions: Vec<ExtractedSecurityTransaction>,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct ExtractedCapitalEntry {
    pub category: String,
    pub description: String,
    pub income_date: String,
    pub gross_amount: f64,
    pub withheld_tax_cz: f64,
    pub withheld_tax_foreign: f64,
    pub country_code: String,
    pub needs_declaring: bool,
}

#[derive(Debug, Clone)]
pub struct ExtractedSecurityTransaction {
    pub asset_type: String,
    pub asset_name: String,
    pub isin: String,
    pub transaction_type: String,
    pub transaction_date: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total_amount: f64,
    pub fees: f64,
    pub currency_code: String,
    pub exchange_rate: f64,
}
```

### 4.3 New extraction function

```rust
/// Process an investment document and extract structured data.
/// Uses the OcrProvider with a specialized investment system prompt.
pub fn extract_investment_document(
    provider: &dyn OcrProvider,
    image_data: &[u8],
    content_type: &str,
) -> Result<InvestmentExtractionResult> {
    // The OcrProvider trait only exposes process_image() with the invoice prompt.
    // We need to either:
    // (a) Add a process_with_prompt() method to OcrProvider, or
    // (b) Create a new InvestmentOcrProvider wrapper.
    //
    // Option (a) is cleaner -- add to trait:
    //   fn process_with_prompt(&self, data: &[u8], ct: &str, system: &str, user: &str) -> Result<String>;
    // Then parse the raw JSON response here.
}
```

**Important design decision:** The existing `OcrProvider::process_image()` has the invoice system prompt baked into each implementation (`AnthropicProvider`, `OpenAIProvider`). For investment extraction, we need a different prompt. Two approaches:

**Approach A (recommended):** Add a `process_raw()` method to the `OcrProvider` trait that returns raw text (no JSON parsing), and let the caller provide system/user prompts. This keeps the trait generic.

```rust
pub trait OcrProvider: Send + Sync {
    fn process_image(&self, image_data: &[u8], content_type: &str) -> Result<OCRResult>;
    fn process_with_prompts(
        &self,
        image_data: &[u8],
        content_type: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String>;
    fn name(&self) -> &str;
}
```

Both `AnthropicProvider` and `OpenAIProvider` already contain the HTTP call logic -- `process_with_prompts` would be a copy of `process_image` but using custom prompts and returning the raw text response instead of parsing it as `OCRResult`.

**Approach B (simpler, less clean):** Create standalone functions `extract_investment_anthropic()` / `extract_investment_openai()` that duplicate the HTTP call with different prompts. This works but duplicates code.

This plan recommends **Approach A**.

## Step 5: Create InvestmentExtractionService

**File:** `zfaktury-core/src/service/investment_extraction_svc.rs` (new file)

This service orchestrates the full extraction pipeline: read file -> call OCR -> parse result -> create domain entries -> update document status.

```rust
use std::sync::Arc;

use chrono::NaiveDate;
use zfaktury_domain::{
    Amount, CapitalIncomeEntry, DomainError, ExtractionStatus, InvestmentDocument,
    Platform, SecurityTransaction,
};

use super::investment_document_svc::InvestmentDocumentService;
use super::investment_income_svc::InvestmentIncomeService;

pub struct InvestmentExtractionService {
    document_service: Arc<InvestmentDocumentService>,
    income_service: Arc<InvestmentIncomeService>,
}

impl InvestmentExtractionService {
    pub fn new(
        document_service: Arc<InvestmentDocumentService>,
        income_service: Arc<InvestmentIncomeService>,
    ) -> Self {
        Self {
            document_service,
            income_service,
        }
    }

    /// Extract data from a document using the given OCR provider.
    /// 1. Reads file from storage path
    /// 2. Calls OCR provider with investment-specific prompt
    /// 3. Parses response into domain types
    /// 4. Creates capital income entries and security transactions
    /// 5. Updates document extraction status
    pub fn extract(
        &self,
        doc_id: i64,
        ocr_provider: &dyn zfaktury_api::ocr::OcrProvider,
    ) -> Result<(), DomainError> {
        let doc = self.document_service.get_by_id(doc_id)?;

        // Read file from storage
        let file_path = format!(
            "{}/investment_documents/{}_{}",
            self.document_service.data_dir(),
            doc.id,
            doc.filename
        );
        let data = std::fs::read(&file_path)
            .map_err(|e| DomainError::ExternalError(format!("reading file: {e}")))?;

        // Call OCR with investment prompt
        let raw_response = ocr_provider
            .process_with_prompts(
                &data,
                &doc.content_type,
                INVESTMENT_SYSTEM_PROMPT,
                INVESTMENT_USER_PROMPT,
            )
            .map_err(|e| DomainError::ExternalError(format!("OCR extraction: {e}")))?;

        // Parse response
        let extraction = parse_investment_json(&raw_response)
            .map_err(|e| DomainError::ExternalError(format!("parsing extraction result: {e}")))?;

        // Create capital income entries
        let now = chrono::Local::now().naive_local();
        for entry_data in &extraction.capital_entries {
            let income_date = NaiveDate::parse_from_str(&entry_data.income_date, "%Y-%m-%d")
                .unwrap_or(chrono::Local::now().date_naive());

            let mut entry = CapitalIncomeEntry {
                id: 0,
                year: doc.year,
                document_id: Some(doc.id),
                category: parse_capital_category(&entry_data.category),
                description: entry_data.description.clone(),
                income_date,
                gross_amount: Amount::from_float(entry_data.gross_amount),
                withheld_tax_cz: Amount::from_float(entry_data.withheld_tax_cz),
                withheld_tax_foreign: Amount::from_float(entry_data.withheld_tax_foreign),
                country_code: entry_data.country_code.clone(),
                needs_declaring: entry_data.needs_declaring,
                net_amount: Amount::ZERO, // computed by service
                created_at: now,
                updated_at: now,
            };
            self.income_service.create_capital_entry(&mut entry)?;
        }

        // Create security transactions
        for tx_data in &extraction.security_transactions {
            let tx_date = NaiveDate::parse_from_str(&tx_data.transaction_date, "%Y-%m-%d")
                .unwrap_or(chrono::Local::now().date_naive());

            let quantity = (tx_data.quantity * 10000.0).round() as i64;
            let exchange_rate = (tx_data.exchange_rate * 10000.0).round() as i64;
            let unit_price = Amount::from_float(tx_data.unit_price);
            let total_amount = if tx_data.total_amount > 0.0 {
                Amount::from_float(tx_data.total_amount)
            } else {
                // Auto-compute: quantity * unit_price
                Amount::from_halere(
                    (quantity as i128 * unit_price.halere() as i128 / 10000) as i64
                )
            };

            let mut tx = SecurityTransaction {
                id: 0,
                year: doc.year,
                document_id: Some(doc.id),
                asset_type: parse_asset_type(&tx_data.asset_type),
                asset_name: tx_data.asset_name.clone(),
                isin: tx_data.isin.clone(),
                transaction_type: parse_transaction_type(&tx_data.transaction_type),
                transaction_date: tx_date,
                quantity,
                unit_price,
                total_amount,
                fees: Amount::from_float(tx_data.fees),
                currency_code: tx_data.currency_code.clone(),
                exchange_rate,
                cost_basis: Amount::ZERO,
                computed_gain: Amount::ZERO,
                time_test_exempt: false,
                exempt_amount: Amount::ZERO,
                created_at: now,
                updated_at: now,
            };
            self.income_service.create_security_transaction(&mut tx)?;
        }

        // Update document status to Extracted
        // Need to call repo directly via document_service
        // ... update_extraction(doc_id, "extracted", "") ...

        Ok(())
    }
}
```

**Note:** The `InvestmentDocumentService` does not currently expose `update_extraction()` -- it exists on the repo trait. Either:
- (a) Add `update_extraction_status(id, status, error)` method to `InvestmentDocumentService`, or
- (b) Have the extraction service hold an `Arc<dyn InvestmentDocumentRepo>` directly.

Option (a) is cleaner -- add to `investment_document_svc.rs`:

```rust
pub fn update_extraction_status(
    &self,
    id: i64,
    status: &str,
    extraction_error: &str,
) -> Result<(), DomainError> {
    self.repo.update_extraction(id, status, extraction_error)?;
    if let Some(ref audit) = self.audit {
        audit.log("investment_document", id, "update_extraction", None, None);
    }
    Ok(())
}
```

Register the service in `mod.rs`:
```rust
pub mod investment_extraction_svc;
pub use investment_extraction_svc::InvestmentExtractionService;
```

## Step 6: Add Documents tab to TaxInvestmentsView

**File:** `zfaktury-app/src/views/tax_investments.rs`

### 6.1 Add tab variant

```rust
#[derive(Clone, PartialEq)]
enum InvestmentTab {
    Documents,      // NEW
    CapitalIncome,
    SecurityTransactions,
}
```

### 6.2 Add struct fields

```rust
pub struct TaxInvestmentsView {
    service: Arc<InvestmentIncomeService>,
    document_service: Arc<InvestmentDocumentService>,   // NEW
    ocr_provider: Option<Arc<dyn OcrProvider + Send + Sync>>,  // NEW

    // ... existing fields ...

    // Document tab state (NEW)
    documents: Vec<InvestmentDocument>,
    document_platform: Entity<Select>,
    uploading: bool,
    extracting_doc_id: Option<i64>,

    // Delete target extended
    // (extend DeleteTarget enum)
}
```

### 6.3 Extend DeleteTarget

```rust
#[derive(Clone)]
enum DeleteTarget {
    CapitalEntry(i64),
    SecurityTx(i64),
    Document(i64),  // NEW
}
```

### 6.4 Change constructor signature

From:
```rust
pub fn new(service: Arc<InvestmentIncomeService>, cx: &mut Context<Self>) -> Self
```

To:
```rust
pub fn new(
    service: Arc<InvestmentIncomeService>,
    document_service: Arc<InvestmentDocumentService>,
    ocr_provider: Option<Arc<dyn zfaktury_api::ocr::OcrProvider + Send + Sync>>,
    cx: &mut Context<Self>,
) -> Self
```

Initialize new fields:
```rust
let document_platform = cx.new(|_cx| {
    Select::new("doc-platform", "Platforma...", Self::platform_options())
});

Self {
    service,
    document_service,
    ocr_provider,
    // ... existing ...
    documents: Vec::new(),
    document_platform,
    uploading: false,
    extracting_doc_id: None,
}
```

### 6.5 Update load_data() to also load documents

In `load_data()`, add `document_service.list_by_year(year)` call:

```rust
fn load_data(&mut self, cx: &mut Context<Self>) {
    let service = self.service.clone();
    let doc_service = self.document_service.clone();
    let year = self.year;

    cx.spawn(async move |this, cx| {
        let result = cx
            .background_executor()
            .spawn(async move {
                let summary = service.get_year_summary(year)?;
                let capital = service.list_capital_entries(year)?;
                let securities = service.list_security_transactions(year)?;
                let documents = doc_service.list_by_year(year)?;
                Ok::<_, zfaktury_domain::DomainError>((summary, capital, securities, documents))
            })
            .await;

        this.update(cx, |this, cx| {
            this.loading = false;
            match result {
                Ok((summary, capital, securities, documents)) => {
                    this.summary = Some(summary);
                    this.capital_entries = capital;
                    this.security_transactions = securities;
                    this.documents = documents;
                }
                Err(e) => {
                    this.error = Some(format!("Chyba pri nacitani investic: {e}"));
                }
            }
            cx.notify();
        }).ok();
    }).detach();
}
```

### 6.6 Update tab bar to include Documents tab

In the `Render` impl, the tab bar section (around line 2182-2196) currently renders 2 tabs. Add a third:

```rust
content = content.child(
    div()
        .flex()
        .gap_2()
        .child(self.render_tab_button("Dokumenty", InvestmentTab::Documents, cx))
        .child(self.render_tab_button("Kapitalove prijmy (p.8)", InvestmentTab::CapitalIncome, cx))
        .child(self.render_tab_button("Obchody s CP (p.10)", InvestmentTab::SecurityTransactions, cx)),
);
```

### 6.7 Update tab content dispatch

```rust
match self.active_tab {
    InvestmentTab::Documents => {
        content = content.child(self.render_documents_tab(cx));
    }
    InvestmentTab::CapitalIncome => {
        content = content.child(self.render_capital_tab(cx));
    }
    InvestmentTab::SecurityTransactions => {
        content = content.child(self.render_security_tab(cx));
    }
}
```

### 6.8 Update render_tab_button to handle the new tab

The `render_tab_button` method (line 1083-1133) formats a tab_id using a match. Add the Documents arm:

```rust
let tab_id = format!(
    "inv-tab-{}",
    match tab {
        InvestmentTab::Documents => "documents",
        InvestmentTab::CapitalIncome => "capital",
        InvestmentTab::SecurityTransactions => "securities",
    }
);
```

## Step 7: Document list display (render_documents_tab)

**File:** `zfaktury-app/src/views/tax_investments.rs`

### 7.1 Platform helpers

```rust
fn platform_options() -> Vec<SelectOption> {
    vec![
        SelectOption { value: "portu".into(), label: "Portu".into() },
        SelectOption { value: "zonky".into(), label: "Zonky".into() },
        SelectOption { value: "trading212".into(), label: "Trading 212".into() },
        SelectOption { value: "revolut".into(), label: "Revolut".into() },
        SelectOption { value: "other".into(), label: "Ostatni".into() },
    ]
}

fn parse_platform(value: &str) -> Platform {
    match value {
        "portu" => Platform::Portu,
        "zonky" => Platform::Zonky,
        "trading212" => Platform::Trading212,
        "revolut" => Platform::Revolut,
        _ => Platform::Other,
    }
}

fn platform_label(p: &Platform) -> &'static str {
    match p {
        Platform::Portu => "Portu",
        Platform::Zonky => "Zonky",
        Platform::Trading212 => "Trading 212",
        Platform::Revolut => "Revolut",
        Platform::Other => "Ostatni",
    }
}

fn extraction_status_label(s: &ExtractionStatus) -> &'static str {
    match s {
        ExtractionStatus::Pending => "Cekajici",
        ExtractionStatus::Extracted => "Extrahovano",
        ExtractionStatus::Failed => "Chyba",
    }
}

fn extraction_status_colors(s: &ExtractionStatus) -> (u32, u32) {
    match s {
        ExtractionStatus::Pending => (ZfColors::STATUS_YELLOW, ZfColors::STATUS_YELLOW_BG),
        ExtractionStatus::Extracted => (ZfColors::STATUS_GREEN, ZfColors::STATUS_GREEN_BG),
        ExtractionStatus::Failed => (ZfColors::STATUS_RED, ZfColors::STATUS_RED_BG),
    }
}

fn format_file_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
```

### 7.2 render_documents_tab method

```rust
fn render_documents_tab(&self, cx: &mut Context<Self>) -> Div {
    let mut table = div()
        .flex()
        .flex_col()
        .bg(rgb(ZfColors::SURFACE))
        .rounded_md()
        .border_1()
        .border_color(rgb(ZfColors::BORDER))
        .overflow_hidden();

    // Header: platform selector + upload button
    let ocr_configured = self.ocr_provider.is_some();
    table = table.child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(rgb(ZfColors::BORDER))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .child("Dokumenty (brokerske vypisy)"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(self.document_platform.clone())
                    .child(render_button(
                        "btn-upload-doc",
                        "Nahrat dokument",
                        ButtonVariant::Primary,
                        self.uploading,
                        false,
                        cx.listener(|this, _event: &ClickEvent, _window, cx| {
                            this.upload_document(cx);
                        }),
                    )),
            ),
    );

    // OCR not configured warning
    if !ocr_configured {
        table = table.child(
            div()
                .px_4()
                .py_2()
                .bg(rgb(ZfColors::STATUS_YELLOW_BG))
                .text_xs()
                .text_color(rgb(ZfColors::STATUS_YELLOW))
                .child("OCR neni nakonfigurovano. Pridejte sekci [ocr] do config.toml pro automaticke extrakce."),
        );
    }

    // Column headers
    table = table.child(
        div()
            .flex()
            .px_4()
            .py_2()
            .text_xs()
            .text_color(rgb(ZfColors::TEXT_MUTED))
            .border_b_1()
            .border_color(rgb(ZfColors::BORDER_SUBTLE))
            .child(div().flex_1().child("Nazev souboru"))
            .child(div().w_28().child("Platforma"))
            .child(div().w_24().child("Stav"))
            .child(div().w_20().text_right().child("Velikost"))
            .child(div().w_48().text_right().child("Akce")),
    );

    // Document rows
    if self.documents.is_empty() {
        table = table.child(
            div()
                .px_4()
                .py_4()
                .text_sm()
                .text_color(rgb(ZfColors::TEXT_MUTED))
                .child("Zadne dokumenty pro tento rok."),
        );
    } else {
        for doc in &self.documents {
            let doc_id = doc.id;
            let (status_color, status_bg) = Self::extraction_status_colors(&doc.extraction_status);
            let status_label = Self::extraction_status_label(&doc.extraction_status);
            let can_extract = matches!(
                doc.extraction_status,
                ExtractionStatus::Pending | ExtractionStatus::Failed
            );
            let is_extracting = self.extracting_doc_id == Some(doc_id);

            table = table.child(
                div()
                    .flex()
                    .items_center()
                    .px_4()
                    .py_2()
                    .text_sm()
                    .border_t_1()
                    .border_color(rgb(ZfColors::BORDER_SUBTLE))
                    .hover(|s| s.bg(rgb(ZfColors::SURFACE_HOVER)))
                    // Filename
                    .child(
                        div()
                            .flex_1()
                            .text_color(rgb(ZfColors::TEXT_PRIMARY))
                            .child(doc.filename.clone()),
                    )
                    // Platform
                    .child(
                        div()
                            .w_28()
                            .text_color(rgb(ZfColors::TEXT_MUTED))
                            .child(Self::platform_label(&doc.platform)),
                    )
                    // Status badge
                    .child(
                        div().w_24().child(
                            div()
                                .px_2()
                                .py(px(2.0))
                                .rounded_sm()
                                .text_xs()
                                .bg(rgb(status_bg))
                                .text_color(rgb(status_color))
                                .child(status_label),
                        ),
                    )
                    // File size
                    .child(
                        div()
                            .w_20()
                            .text_right()
                            .text_color(rgb(ZfColors::TEXT_MUTED))
                            .child(Self::format_file_size(doc.size)),
                    )
                    // Action buttons
                    .child(
                        div()
                            .w_48()
                            .flex()
                            .justify_end()
                            .gap_1()
                            .when(can_extract && ocr_configured, |el| {
                                el.child(render_button(
                                    SharedString::from(format!("doc-extract-{doc_id}")),
                                    if is_extracting { "Extrakce..." } else { "Extrahovat" },
                                    ButtonVariant::Primary,
                                    is_extracting || self.uploading,
                                    false,
                                    cx.listener(move |this, _ev: &ClickEvent, _w, cx| {
                                        this.extract_document(doc_id, cx);
                                    }),
                                ))
                            })
                            .child(render_button(
                                SharedString::from(format!("doc-download-{doc_id}")),
                                "Stahnout",
                                ButtonVariant::Secondary,
                                false,
                                false,
                                cx.listener(move |this, _ev: &ClickEvent, _w, cx| {
                                    this.download_document(doc_id, cx);
                                }),
                            ))
                            .child(render_button(
                                SharedString::from(format!("doc-delete-{doc_id}")),
                                "Smazat",
                                ButtonVariant::Danger,
                                self.saving || is_extracting,
                                false,
                                cx.listener(move |this, _ev: &ClickEvent, _w, cx| {
                                    this.request_delete_document(doc_id, cx);
                                }),
                            )),
                    ),
            );
        }
    }

    table
}
```

## Step 8: File upload implementation

**File:** `zfaktury-app/src/views/tax_investments.rs`

Uses `rfd::FileDialog` for native file selection. File picker runs on background thread since it blocks.

```rust
fn upload_document(&mut self, cx: &mut Context<Self>) {
    if self.uploading {
        return;
    }

    let platform_value = self
        .document_platform
        .read(cx)
        .selected_value()
        .unwrap_or("other")
        .to_string();
    let year = self.year;
    let doc_svc = self.document_service.clone();

    self.uploading = true;
    self.error = None;
    cx.notify();

    cx.spawn(async move |this, cx| {
        // Step 1: Open file dialog (blocking)
        let file_result = cx
            .background_executor()
            .spawn(async move {
                rfd::FileDialog::new()
                    .add_filter("Dokumenty", &["pdf", "csv", "jpg", "jpeg", "png"])
                    .set_title("Vyberte dokument")
                    .pick_file()
            })
            .await;

        let Some(path) = file_result else {
            // User cancelled
            this.update(cx, |this, cx| {
                this.uploading = false;
                cx.notify();
            }).ok();
            return;
        };

        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let content_type = match path.extension().and_then(|e| e.to_str()) {
            Some("pdf") => "application/pdf",
            Some("csv") => "text/csv",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            _ => "application/octet-stream",
        }
        .to_string();

        // Step 2: Read file and create record (blocking)
        let doc_svc2 = doc_svc.clone();
        let filename2 = filename.clone();
        let content_type2 = content_type.clone();
        let platform = Self::parse_platform(&platform_value);
        let result = cx
            .background_executor()
            .spawn(async move {
                let data = std::fs::read(&path)
                    .map_err(|e| zfaktury_domain::DomainError::ExternalError(
                        format!("reading file: {e}")
                    ))?;

                let now = chrono::Local::now().naive_local();
                let mut doc = zfaktury_domain::InvestmentDocument {
                    id: 0,
                    year,
                    platform,
                    filename: filename2,
                    content_type: content_type2,
                    storage_path: String::new(),
                    size: data.len() as i64,
                    extraction_status: zfaktury_domain::ExtractionStatus::Pending,
                    extraction_error: String::new(),
                    created_at: now,
                    updated_at: now,
                };
                doc_svc2.create_record(&mut doc)?;

                // Write file to storage
                let storage_dir = format!("{}/investment_documents", doc_svc2.data_dir());
                std::fs::create_dir_all(&storage_dir).ok();
                let storage_path = format!("{}/{}_{}", storage_dir, doc.id, doc.filename);
                std::fs::write(&storage_path, &data)
                    .map_err(|e| zfaktury_domain::DomainError::ExternalError(
                        format!("writing file: {e}")
                    ))?;

                Ok::<(), zfaktury_domain::DomainError>(())
            })
            .await;

        this.update(cx, |this, cx| {
            this.uploading = false;
            if let Err(e) = result {
                this.error = Some(format!("Chyba pri nahravani: {e}"));
            }
            this.load_data(cx);
            cx.notify();
        }).ok();
    })
    .detach();
}
```

## Step 9: Document extraction

**File:** `zfaktury-app/src/views/tax_investments.rs`

```rust
fn extract_document(&mut self, doc_id: i64, cx: &mut Context<Self>) {
    if self.extracting_doc_id.is_some() {
        return;
    }

    let Some(ref ocr) = self.ocr_provider else {
        self.error = Some("OCR neni nakonfigurovano. Pridejte sekci [ocr] do config.toml.".into());
        cx.notify();
        return;
    };

    self.extracting_doc_id = Some(doc_id);
    self.error = None;
    cx.notify();

    let doc_svc = self.document_service.clone();
    let income_svc = self.service.clone();
    let ocr = ocr.clone();

    cx.spawn(async move |this, cx| {
        let result = cx
            .background_executor()
            .spawn(async move {
                let doc = doc_svc.get_by_id(doc_id)?;

                // Read file
                let file_path = format!(
                    "{}/investment_documents/{}_{}",
                    doc_svc.data_dir(), doc.id, doc.filename
                );
                let data = std::fs::read(&file_path)
                    .map_err(|e| zfaktury_domain::DomainError::ExternalError(
                        format!("reading file: {e}")
                    ))?;

                // Call OCR with investment-specific prompts
                let raw_response = ocr
                    .process_with_prompts(
                        &data,
                        &doc.content_type,
                        INVESTMENT_SYSTEM_PROMPT,
                        INVESTMENT_USER_PROMPT,
                    )
                    .map_err(|e| zfaktury_domain::DomainError::ExternalError(
                        format!("OCR: {e}")
                    ))?;

                // Parse and create entries
                let extraction = zfaktury_api::ocr::parse_investment_json(&raw_response)
                    .map_err(|e| zfaktury_domain::DomainError::ExternalError(
                        format!("parsing: {e}")
                    ))?;

                let now = chrono::Local::now().naive_local();

                for entry_data in &extraction.capital_entries {
                    let income_date = chrono::NaiveDate::parse_from_str(
                        &entry_data.income_date, "%Y-%m-%d"
                    ).unwrap_or(chrono::Local::now().date_naive());

                    let mut entry = zfaktury_domain::CapitalIncomeEntry {
                        id: 0,
                        year: doc.year,
                        document_id: Some(doc.id),
                        category: /* parse from entry_data.category */,
                        description: entry_data.description.clone(),
                        income_date,
                        gross_amount: zfaktury_domain::Amount::from_float(entry_data.gross_amount),
                        withheld_tax_cz: zfaktury_domain::Amount::from_float(entry_data.withheld_tax_cz),
                        withheld_tax_foreign: zfaktury_domain::Amount::from_float(entry_data.withheld_tax_foreign),
                        country_code: entry_data.country_code.clone(),
                        needs_declaring: entry_data.needs_declaring,
                        net_amount: zfaktury_domain::Amount::ZERO,
                        created_at: now,
                        updated_at: now,
                    };
                    income_svc.create_capital_entry(&mut entry)?;
                }

                for tx_data in &extraction.security_transactions {
                    let tx_date = chrono::NaiveDate::parse_from_str(
                        &tx_data.transaction_date, "%Y-%m-%d"
                    ).unwrap_or(chrono::Local::now().date_naive());

                    let quantity = (tx_data.quantity * 10000.0).round() as i64;
                    let exchange_rate = (tx_data.exchange_rate * 10000.0).round() as i64;
                    let unit_price = zfaktury_domain::Amount::from_float(tx_data.unit_price);

                    let mut tx = zfaktury_domain::SecurityTransaction {
                        id: 0,
                        year: doc.year,
                        document_id: Some(doc.id),
                        asset_type: /* parse from tx_data.asset_type */,
                        asset_name: tx_data.asset_name.clone(),
                        isin: tx_data.isin.clone(),
                        transaction_type: /* parse */,
                        transaction_date: tx_date,
                        quantity,
                        unit_price,
                        total_amount: zfaktury_domain::Amount::from_float(tx_data.total_amount),
                        fees: zfaktury_domain::Amount::from_float(tx_data.fees),
                        currency_code: tx_data.currency_code.clone(),
                        exchange_rate,
                        cost_basis: zfaktury_domain::Amount::ZERO,
                        computed_gain: zfaktury_domain::Amount::ZERO,
                        time_test_exempt: false,
                        exempt_amount: zfaktury_domain::Amount::ZERO,
                        created_at: now,
                        updated_at: now,
                    };
                    income_svc.create_security_transaction(&mut tx)?;
                }

                // Update document status
                doc_svc.update_extraction_status(doc_id, "extracted", "")?;

                Ok::<(), zfaktury_domain::DomainError>(())
            })
            .await;

        this.update(cx, |this, cx| {
            this.extracting_doc_id = None;
            match result {
                Ok(()) => {
                    // Switch to CapitalIncome tab to show results
                    // Or stay on Documents tab and just reload
                }
                Err(e) => {
                    this.error = Some(format!("Chyba pri extrakci: {e}"));
                    // Update document status to failed (fire and forget)
                    let doc_svc = this.document_service.clone();
                    let err_msg = format!("{e}");
                    cx.background_executor()
                        .spawn(async move {
                            let _ = doc_svc.update_extraction_status(
                                doc_id, "failed", &err_msg
                            );
                        })
                        .detach();
                }
            }
            this.load_data(cx);
            cx.notify();
        }).ok();
    })
    .detach();
}
```

## Step 10: Document download and delete

**File:** `zfaktury-app/src/views/tax_investments.rs`

### 10.1 Download (open with system viewer)

Following existing pattern from `invoice_detail.rs` and `reports.rs` which use `xdg-open`:

```rust
fn download_document(&mut self, doc_id: i64, cx: &mut Context<Self>) {
    let doc_svc = self.document_service.clone();

    cx.spawn(async move |this, cx| {
        let result = cx
            .background_executor()
            .spawn(async move {
                let doc = doc_svc.get_by_id(doc_id)?;
                let file_path = format!(
                    "{}/investment_documents/{}_{}",
                    doc_svc.data_dir(), doc.id, doc.filename
                );

                // Verify file exists
                if !std::path::Path::new(&file_path).exists() {
                    return Err(zfaktury_domain::DomainError::NotFound);
                }

                // Open with system viewer
                let _ = std::process::Command::new("xdg-open")
                    .arg(&file_path)
                    .spawn();

                Ok::<(), zfaktury_domain::DomainError>(())
            })
            .await;

        if let Err(e) = result {
            this.update(cx, |this, cx| {
                this.error = Some(format!("Chyba pri otevirani souboru: {e}"));
                cx.notify();
            }).ok();
        }
    })
    .detach();
}
```

### 10.2 Delete with confirmation

```rust
fn request_delete_document(&mut self, id: i64, cx: &mut Context<Self>) {
    self.delete_target = Some(DeleteTarget::Document(id));
    let dialog = cx.new(|_cx| {
        ConfirmDialog::new(
            "Smazat dokument",
            "Opravdu chcete smazat tento dokument? Budou smazany i vsechny propojene zaznamy.",
            "Smazat",
        )
    });
    cx.subscribe(&dialog, Self::on_confirm_result).detach();
    self.confirm_dialog = Some(dialog);
    cx.notify();
}
```

And add the `Document` arm in `on_confirm_result`:

```rust
fn on_confirm_result(
    &mut self,
    _dialog: Entity<ConfirmDialog>,
    event: &ConfirmDialogResult,
    cx: &mut Context<Self>,
) {
    match event {
        ConfirmDialogResult::Confirmed => {
            if let Some(target) = self.delete_target.take() {
                self.confirm_dialog = None;
                match target {
                    DeleteTarget::CapitalEntry(id) => self.do_delete_capital(id, cx),
                    DeleteTarget::SecurityTx(id) => self.do_delete_security(id, cx),
                    DeleteTarget::Document(id) => self.do_delete_document(id, cx),
                }
            }
        }
        ConfirmDialogResult::Cancelled => {
            self.delete_target = None;
            self.confirm_dialog = None;
            cx.notify();
        }
    }
}

fn do_delete_document(&mut self, id: i64, cx: &mut Context<Self>) {
    self.saving = true;
    self.error = None;
    cx.notify();

    let doc_svc = self.document_service.clone();
    let service = self.service.clone();
    let year = self.year;

    cx.spawn(async move |this, cx| {
        let result = cx
            .background_executor()
            .spawn(async move {
                // Also delete file from disk
                if let Ok(doc) = doc_svc.get_by_id(id) {
                    let file_path = format!(
                        "{}/investment_documents/{}_{}",
                        doc_svc.data_dir(), doc.id, doc.filename
                    );
                    let _ = std::fs::remove_file(&file_path);
                }

                // Cascading delete (document + linked entries)
                doc_svc.delete(id)?;

                // Reload summary
                let summary = service.get_year_summary(year)?;
                let capital = service.list_capital_entries(year)?;
                let securities = service.list_security_transactions(year)?;
                let documents = doc_svc.list_by_year(year)?;
                Ok::<_, zfaktury_domain::DomainError>((summary, capital, securities, documents))
            })
            .await;

        this.update(cx, |this, cx| {
            this.saving = false;
            match result {
                Ok((summary, capital, securities, documents)) => {
                    this.summary = Some(summary);
                    this.capital_entries = capital;
                    this.security_transactions = securities;
                    this.documents = documents;
                }
                Err(e) => this.error = Some(format!("Chyba pri mazani: {e}")),
            }
            cx.notify();
        }).ok();
    })
    .detach();
}
```

## Step 11: Update root.rs constructor call

**File:** `zfaktury-app/src/root.rs`

Change the `Route::TaxInvestments` handler (around line 402-404):

From:
```rust
Route::TaxInvestments => {
    let svc = services.investment_income.clone();
    ContentView::TaxInvestments(cx.new(|cx| TaxInvestmentsView::new(svc, cx)))
}
```

To:
```rust
Route::TaxInvestments => {
    let income_svc = services.investment_income.clone();
    let doc_svc = services.investment_documents.clone();
    let ocr = services.ocr_provider.clone();
    ContentView::TaxInvestments(cx.new(|cx| {
        TaxInvestmentsView::new(income_svc, doc_svc, ocr, cx)
    }))
}
```

Also add the `use zfaktury_api::ocr::OcrProvider;` import if needed by the type in root.rs (it may not be needed since the type is hidden behind `Arc<dyn ...>`).

## Step 12: Add update_extraction_status to InvestmentDocumentService

**File:** `zfaktury-core/src/service/investment_document_svc.rs`

```rust
pub fn update_extraction_status(
    &self,
    id: i64,
    status: &str,
    extraction_error: &str,
) -> Result<(), DomainError> {
    if id == 0 {
        return Err(DomainError::InvalidInput);
    }
    self.repo.update_extraction(id, status, extraction_error)?;
    if let Some(ref audit) = self.audit {
        audit.log(
            "investment_document",
            id,
            "update_extraction",
            None,
            Some(&format!("status={}, error={}", status, extraction_error)),
        );
    }
    Ok(())
}
```

## Files to modify (exact list)

| File | Change type | Description |
|------|------------|-------------|
| `zfaktury-domain/src/errors.rs` | Modify | Add `ExternalError(String)` variant to `DomainError` |
| `zfaktury-app/Cargo.toml` | Modify | Add `rfd = "0.15"` and `zfaktury-api` dependency |
| `zfaktury-app/src/app.rs` | Modify | Add `ocr_provider` field, change `new()` signature to accept `&Config`, construct OCR provider |
| `zfaktury-app/src/main.rs` | Modify | Pass `&config` to `AppServices::new()` |
| `zfaktury-app/src/root.rs` | Modify | Pass `investment_documents` + `ocr_provider` to `TaxInvestmentsView::new()` |
| `zfaktury-app/src/views/tax_investments.rs` | Major rewrite | Add Documents tab, upload, extract, download, delete; change constructor |
| `zfaktury-core/src/service/investment_document_svc.rs` | Modify | Add `update_extraction_status()` method |
| `zfaktury-api/src/ocr.rs` | Modify | Add `process_with_prompts()` to `OcrProvider` trait + impls; add `InvestmentExtractionResult` types; add `parse_investment_json()` |
| `zfaktury-core/src/service/mod.rs` | Modify | (Optional) Register `InvestmentExtractionService` if created as separate service |
| `zfaktury-core/src/service/investment_extraction_svc.rs` | New file (optional) | Extraction pipeline service -- can be inlined in view instead |

## Dependency on expense OCR phase

The following may already be done by the expense OCR phase. Check and reuse if available:

| Component | Needed here | May already exist from expense OCR |
|-----------|-------------|-------------------------------------|
| `rfd` in Cargo.toml | Yes | Yes -- expense_import.rs currently has a placeholder comment |
| OCR provider wiring in `AppServices` | Yes | Possibly -- if expense OCR wires `OcrProvider` into AppServices |
| `ExternalError` DomainError variant | Yes | Possibly |
| `process_with_prompts()` on OcrProvider | Yes (investment needs different prompt) | No -- expense OCR uses the existing invoice prompt |

If the expense OCR phase runs first, Steps 1, 2 (rfd part), and 3 may already be complete.

## Implementation order

Recommended sequential order for minimal conflict:

1. **Step 1** -- `ExternalError` variant (1 line change, no conflicts)
2. **Step 2** -- Dependencies in Cargo.toml (2 lines)
3. **Step 4** -- Investment OCR types + prompt in `zfaktury-api/src/ocr.rs` (new types, `process_with_prompts` on trait)
4. **Step 12** -- `update_extraction_status` method on `InvestmentDocumentService`
5. **Step 3** -- OCR provider wiring in `AppServices` (changes `app.rs`, `main.rs`)
6. **Steps 6-10** -- All view changes (single file `tax_investments.rs`)
7. **Step 11** -- `root.rs` constructor update (last, depends on step 6)

## Testing plan

### Unit tests

1. **`zfaktury-api/src/ocr.rs`** -- Test `parse_investment_json()` with sample broker statement JSON:
   - Valid response with mixed capital entries and transactions
   - Response with empty arrays
   - Response with code fences
   - Invalid JSON response
   - Test `process_with_prompts` via wiremock (both Anthropic and OpenAI providers)

2. **`zfaktury-core/src/service/investment_document_svc.rs`** -- Test `update_extraction_status()`:
   - Status updates from pending to extracted
   - Status updates from pending to failed with error message

3. **`zfaktury-domain/src/errors.rs`** -- Verify `ExternalError` displays correctly

### Integration tests

1. **OCR provider construction** -- Test that `AppServices::new()` with an `[ocr]` config section creates a non-None `ocr_provider`
2. **OCR provider construction** -- Test that `AppServices::new()` without `[ocr]` config results in `ocr_provider = None`
3. **Document upload flow** -- Create a document via service, verify file is written to disk
4. **Document delete cascade** -- Create document + linked entries, delete document, verify entries are also deleted

### Manual testing

1. Start app without `[ocr]` config -- Documents tab should show yellow warning bar, Extract buttons hidden
2. Upload a PDF file -- verify it appears in the list with "Cekajici" status
3. Click "Stahnout" -- verify file opens in system viewer
4. Click "Smazat" on document -- verify confirm dialog, then document + linked entries deleted
5. Configure `[ocr]` in config.toml, restart -- verify Extract buttons appear
6. Upload a broker statement, click "Extrahovat" -- verify entries are created in Capital/Security tabs

## Risks and mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `rfd::FileDialog` blocks the main thread | UI freeze | Already mitigated: `cx.background_executor().spawn()` is used for file dialog (same pattern as RFC specifies). Note: `rfd::FileDialog` is synchronous but runs in a background task. |
| OCR returns garbage for broker statements | Bad data created | Show extracted entries count in success message, user can review on other tabs and delete individual bad entries. Extraction button shows "Extrahovat" again for retry. |
| `process_with_prompts()` is a breaking trait change | All trait implementors must update | Only 2 implementors exist (`AnthropicProvider`, `OpenAIProvider`), both in the same file. Change is mechanical: copy `process_image` logic, replace hardcoded prompts with parameters, return raw text. |
| `ExternalError` variant changes `DomainError` exhaustive match | Compile errors in existing match arms | `DomainError` is only matched in `zfaktury-db/src/helpers.rs` (From impl) -- add the new variant there. All service code uses `?` propagation, not matching. |
| Large file uploads (100MB+ PDF) | Memory pressure, slow upload | Check file size before reading into memory. Reject files > 50MB with user-friendly error. |
| `AppServices::new()` signature change breaks callers | Compile error | Only one caller in `main.rs` -- simple fix. |
| Config not loaded before AppServices in main.rs | Runtime error | Config is already used for `resolve_db_path()` and `resolve_data_dir()` -- just need to pass it through. |
