# Phase: OCR Infrastructure - Implementation Plan

## Overview

Shared infrastructure needed by both expense OCR import (RFC-020) and investment document extraction (RFC-023). This phase adds:
1. The `rfd` crate for native file dialogs
2. OCR provider construction from config
3. Wiring of `OCRService` into `AppServices`
4. Config flow from `main.rs` through `AppServices::new()`

After this phase, both expense import and investment document views can consume OCR via `services.ocr_service: Option<Arc<OCRService>>`.

---

## Current State

### Two OCR Provider Traits (Problem)

There are **two separate** `OcrProvider` traits that need to be reconciled:

1. **`zfaktury-core/src/service/ocr_svc.rs`** -- `OCRProvider` (PascalCase)
   - Returns `Result<OCRResult, DomainError>`
   - Used by `OCRService` (core layer)
   - This is the trait that services depend on

2. **`zfaktury-api/src/ocr.rs`** -- `OcrProvider` (camelCase)
   - Returns `Result<OCRResult>` (where `Result` = `Result<T, ApiError>`)
   - Implemented by `AnthropicProvider` and `OpenAIProvider`
   - This is where the actual API implementations live

The API implementations cannot directly satisfy the core trait because the error types differ (`ApiError` vs `DomainError`). We need an **adapter** that wraps `zfaktury_api::ocr::OcrProvider` and implements `zfaktury_core::service::ocr_svc::OCRProvider`.

### AppServices::new() Signature

Currently:
```rust
pub fn new(db_path: &Path, data_dir: &Path) -> Result<Self>
```

Does NOT receive config. Config is only loaded in `main.rs` for `resolve_db_path()` / `resolve_data_dir()` and then thrown away. The OCR config (`Config.ocr: Option<OcrConfig>`) never reaches `AppServices`.

### ImportService OCR Gap

```rust
// app.rs line 358-362
let import = Arc::new(ImportService::new(
    expenses.clone(),
    documents.clone(),
    None, // OCR provider not configured for desktop app
));
```

`ImportService` takes `Option<Arc<OCRService>>`, currently hardcoded to `None`.

### rfd Not in Dependencies

`rfd` is not in any `Cargo.toml`. The `ExpenseImportView::select_file()` method has a placeholder error message.

---

## Step 1: Add rfd dependency to zfaktury-app

### Cargo.toml entry

```toml
# rust/crates/zfaktury-app/Cargo.toml
[dependencies]
rfd = "0.15"
```

No feature flags needed. On Linux, `rfd` 0.15 uses the XDG Desktop Portal (via `ashpd`) by default, with GTK3 fallback. Since GPUI already requires GTK3 + WebKitGTK for desktop builds (configured in `~/Code/Nix/home/packages/programming/go.nix`), the GTK fallback path is guaranteed to work.

### Platform considerations

- **Wayland:** `rfd` 0.15 supports Wayland via XDG Desktop Portal. No extra config needed.
- **X11:** Falls back to GTK3 file chooser. Works out of the box.
- **NixOS:** GTK3 is already a dependency for GPUI. No additional Nix packages required.

### Integration with GPUI async model

`rfd` provides both sync (`FileDialog`) and async (`AsyncFileDialog`) APIs. For GPUI:
- Use `rfd::FileDialog` (blocking) inside `cx.background_executor().spawn(...)` -- this is the pattern from RFC-023.
- Do NOT use `rfd::AsyncFileDialog` because its async runtime (async-std internally) may conflict with GPUI's executor.

```rust
// Correct pattern for GPUI:
fn select_file(&mut self, cx: &mut Context<Self>) {
    self.processing = true;
    cx.notify();

    cx.spawn(async move |this, cx| {
        let file_result = cx.background_executor().spawn(async {
            rfd::FileDialog::new()
                .add_filter("Documents", &["pdf", "jpg", "jpeg", "png"])
                .set_title("Vyberte doklad")
                .pick_file()
        }).await;

        if let Some(path) = file_result {
            let data = std::fs::read(&path).ok();
            let filename = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let content_type = match path.extension().and_then(|e| e.to_str()) {
                Some("pdf") => "application/pdf",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                _ => "application/octet-stream",
            };

            if let Some(data) = data {
                this.update(cx, |this, cx| {
                    this.process_file(filename, content_type.to_string(), data, cx);
                }).ok();
            }
        } else {
            this.update(cx, |this, cx| {
                this.processing = false;
                cx.notify();
            }).ok();
        }
    }).detach();
}
```

---

## Step 2: Add zfaktury-api dependency to zfaktury-app

Currently `zfaktury-app/Cargo.toml` depends on `zfaktury-core` but NOT on `zfaktury-api`. The OCR provider implementations live in `zfaktury-api`. We need to add it:

```toml
# rust/crates/zfaktury-app/Cargo.toml
[dependencies]
zfaktury-api = { path = "../zfaktury-api" }
```

---

## Step 3: Create OCR provider adapter in app.rs

Bridge the gap between `zfaktury_api::ocr::OcrProvider` (returns `ApiError`) and `zfaktury_core::service::ocr_svc::OCRProvider` (returns `DomainError`).

### Adapter struct

Add to `app.rs`:

```rust
use zfaktury_api::ocr::{AnthropicProvider, OpenAIProvider, OcrProvider as ApiOcrProvider};
use zfaktury_core::service::ocr_svc::OCRProvider;
use zfaktury_domain::{DomainError, OCRResult};

/// Adapter that wraps an API-layer OcrProvider to satisfy the core OCRProvider trait.
struct OcrProviderAdapter {
    inner: Box<dyn ApiOcrProvider>,
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

### Provider factory function

```rust
use zfaktury_config::OcrConfig;

/// Construct an OCR provider from config. Returns None if not configured.
fn create_ocr_provider(config: &OcrConfig) -> Option<Arc<dyn OCRProvider>> {
    let api_key = config.api_key.as_deref().unwrap_or("");
    if api_key.is_empty() {
        log::info!("OCR not configured: no api_key");
        return None;
    }

    let provider_name = config.provider.as_deref().unwrap_or("openai");
    let model = config.model.as_deref().unwrap_or("");

    let api_provider: Box<dyn ApiOcrProvider> = match provider_name {
        "anthropic" | "claude" => {
            let mut p = AnthropicProvider::new(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            Box::new(p)
        }
        "openrouter" => {
            let mut p = OpenAIProvider::new_openrouter(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            Box::new(p)
        }
        "openai" | "gemini" | "mistral" | _ => {
            // OpenAI-compatible API (also works for Gemini and Mistral via base_url override)
            let mut p = OpenAIProvider::new_openai(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            Box::new(p)
        }
    };

    log::info!("OCR provider configured: {provider_name}");
    Some(Arc::new(OcrProviderAdapter {
        inner: api_provider,
    }))
}
```

---

## Step 4: Change AppServices::new() to accept config

### Current signature

```rust
pub fn new(db_path: &Path, data_dir: &Path) -> Result<Self>
```

### New signature

```rust
pub fn new(db_path: &Path, data_dir: &Path, config: &zfaktury_config::Config) -> Result<Self>
```

We pass the full `Config` (not just `OcrConfig`) because future features (SMTP wiring for ReminderService, FIO bank wiring) will also need config sections. This avoids changing the signature again later.

### Struct changes

Add one field:

```rust
pub struct AppServices {
    // ... existing 31 fields ...
    pub ocr_service: Option<Arc<OCRService>>,
}
```

---

## Step 5: Wire OCR into AppServices::new()

After the `DocumentService` construction (line ~180) and before `ImportService` construction (line ~358), add:

```rust
// --- OCR Service (optional, depends on config) ---
let ocr_provider = config.ocr.as_ref().and_then(create_ocr_provider);
let ocr_service = ocr_provider.map(|provider| {
    Arc::new(OCRService::new(provider, documents.clone()))
});
```

Then update `ImportService` construction:

```rust
// ImportService (depends on ExpenseService, DocumentService, OCRService)
let import = Arc::new(ImportService::new(
    expenses.clone(),
    documents.clone(),
    ocr_service.clone(),
));
```

And add to the struct construction:

```rust
Ok(Self {
    // ... existing fields ...
    import,
    ocr_service,
})
```

---

## Step 6: Update main.rs to pass config to AppServices

### Current flow (main.rs)

```rust
fn cmd_gui(cli: Cli) {
    let db_path = resolve_db_path(&cli);   // loads Config, extracts db_path, drops Config
    let data_dir = resolve_data_dir(&cli); // loads Config again, extracts data_dir, drops Config
    let services = AppServices::new(&db_path, &data_dir)?;
    // ...
}
```

Config is loaded twice and discarded each time.

### New flow

```rust
fn cmd_gui(cli: Cli) {
    let config = match zfaktury_config::Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };

    let db_path = cli.db.clone().unwrap_or_else(|| config.database_path());
    let data_dir = if cli.db.is_some() {
        cli.db.as_ref().unwrap()
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        config.data_dir()
    };

    ensure_parent_dir(&db_path);
    ensure_parent_dir_of(&data_dir);

    let services = match AppServices::new(&db_path, &data_dir, &config) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Error initializing application: {e}");
            std::process::exit(1);
        }
    };
    // ... rest unchanged
}
```

The `resolve_db_path()` and `resolve_data_dir()` helper functions can be simplified or removed. Keep them for `cmd_migrate()` which doesn't need full config.

---

## Step 7: Fix OCRService.process_document() (optional but recommended)

The current `OCRService::process_document()` always returns `Err(DomainError::InvalidInput)` with a comment saying "file read must happen at a higher layer." This should be fixed to actually read the file and call the provider:

```rust
pub fn process_document(&self, document_id: i64) -> Result<OCRResult, DomainError> {
    if document_id == 0 {
        return Err(DomainError::InvalidInput);
    }
    let doc = self.documents.get_by_id(document_id)?;

    let supported = ["image/jpeg", "image/png", "application/pdf"];
    if !supported.contains(&doc.content_type.as_str()) {
        return Err(DomainError::InvalidInput);
    }

    let data = std::fs::read(&doc.storage_path).map_err(|e| {
        log::error!("reading document file {}: {e}", doc.storage_path);
        DomainError::NotFound
    })?;

    self.provider.process_image(&data, &doc.content_type)
}
```

Alternatively, both expense import and investment views can call the provider directly with in-memory bytes (from `rfd` file selection), bypassing `OCRService::process_document()` entirely. In that case, views need `Option<Arc<dyn OCRProvider>>` alongside `OCRService`. The simpler approach is to fix `process_document()`.

For the **direct approach** (views call provider with bytes, no document ID needed yet):

```rust
pub fn process_bytes(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
    let supported = ["image/jpeg", "image/png", "application/pdf"];
    if !supported.contains(&content_type) {
        return Err(DomainError::InvalidInput);
    }
    self.provider.process_image(data, content_type)
}
```

Add `process_bytes()` as a new method so both approaches work.

---

## Files to Modify (exact list)

| File | Lines | Change |
|------|-------|--------|
| `rust/crates/zfaktury-app/Cargo.toml` | 7-17 | Add `rfd = "0.15"` and `zfaktury-api = { path = "../zfaktury-api" }` |
| `rust/crates/zfaktury-app/src/app.rs` | 1-13 | Add imports for `zfaktury_api::ocr::*`, `zfaktury_config::OcrConfig` |
| `rust/crates/zfaktury-app/src/app.rs` | 50-84 | Add `ocr_service: Option<Arc<OCRService>>` field |
| `rust/crates/zfaktury-app/src/app.rs` | 89 | Change signature: `new(db_path, data_dir, config: &Config)` |
| `rust/crates/zfaktury-app/src/app.rs` | ~180 (after documents) | Add OCR provider construction |
| `rust/crates/zfaktury-app/src/app.rs` | 357-362 | Replace `None` with `ocr_service.clone()` for ImportService |
| `rust/crates/zfaktury-app/src/app.rs` | 364-396 | Add `ocr_service` to struct literal |
| `rust/crates/zfaktury-app/src/app.rs` | (new, after struct) | Add `OcrProviderAdapter` struct + `create_ocr_provider()` fn |
| `rust/crates/zfaktury-app/src/main.rs` | 130-145 | Refactor `cmd_gui()` to load config once and pass to `AppServices::new()` |
| `rust/crates/zfaktury-core/src/service/ocr_svc.rs` | 27-44 | Add `process_bytes()` method, optionally fix `process_document()` |

---

## Exact Code Changes

### 1. `rust/crates/zfaktury-app/Cargo.toml`

Add after the `zfaktury-config` line:

```toml
zfaktury-api = { path = "../zfaktury-api" }
rfd = "0.15"
```

### 2. `rust/crates/zfaktury-app/src/app.rs` -- Full modified file

```rust
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use zfaktury_api::ocr::{AnthropicProvider, OpenAIProvider, OcrProvider as ApiOcrProvider};
use zfaktury_config::{Config, OcrConfig};
use zfaktury_core::service::{
    AuditService, BackupService, CategoryService, ContactService, DashboardService,
    DocumentService, ExpenseService, HealthInsuranceService, ImportService, IncomeTaxReturnService,
    InvestmentDocumentService, InvestmentIncomeService, InvoiceDocumentService, InvoiceService,
    OCRService, OverdueService, RecurringExpenseService, RecurringInvoiceService, ReminderService,
    ReportService, SequenceService, SettingsService, SocialInsuranceService, TaxCalendarService,
    TaxCreditsService, TaxDeductionDocumentService, TaxYearSettingsService,
    VATControlStatementService, VATReturnService, VIESSummaryService,
};
use zfaktury_core::service::ocr_svc::OCRProvider;
use zfaktury_db::connection::open_connection;
use zfaktury_db::migrate::run_migrations;
// ... all existing repo imports unchanged ...
use zfaktury_domain::{DomainError, OCRResult};

/// Shared application state holding all service instances.
#[allow(dead_code)]
pub struct AppServices {
    // --- Core entity services (12, existing) ---
    pub dashboard: Arc<DashboardService>,
    pub invoices: Arc<InvoiceService>,
    pub expenses: Arc<ExpenseService>,
    pub contacts: Arc<ContactService>,
    pub settings: Arc<SettingsService>,
    pub categories: Arc<CategoryService>,
    pub sequences: Arc<SequenceService>,
    pub audit: Arc<AuditService>,
    pub recurring_invoices: Arc<RecurringInvoiceService>,
    pub recurring_expenses: Arc<RecurringExpenseService>,
    pub vat_returns: Arc<VATReturnService>,
    pub reports: Arc<ReportService>,

    // --- Newly wired services (18 + 1) ---
    pub backup: Arc<BackupService>,
    pub documents: Arc<DocumentService>,
    pub health_insurance: Arc<HealthInsuranceService>,
    pub income_tax: Arc<IncomeTaxReturnService>,
    pub investment_documents: Arc<InvestmentDocumentService>,
    pub investment_income: Arc<InvestmentIncomeService>,
    pub invoice_documents: Arc<InvoiceDocumentService>,
    pub overdue: Arc<OverdueService>,
    pub reminders: Arc<ReminderService>,
    pub social_insurance: Arc<SocialInsuranceService>,
    pub tax_calendar: Arc<TaxCalendarService>,
    pub tax_credits: Arc<TaxCreditsService>,
    pub tax_deduction_documents: Arc<TaxDeductionDocumentService>,
    pub tax_year_settings: Arc<TaxYearSettingsService>,
    pub vat_control: Arc<VATControlStatementService>,
    pub vies: Arc<VIESSummaryService>,
    pub import: Arc<ImportService>,

    // --- OCR (optional) ---
    pub ocr_service: Option<Arc<OCRService>>,
}

impl AppServices {
    pub fn new(db_path: &Path, data_dir: &Path, config: &Config) -> Result<Self> {
        // ... everything unchanged until after DocumentService construction ...

        // --- OCR Service (optional, depends on config) ---
        let ocr_provider = config.ocr.as_ref().and_then(create_ocr_provider);
        let ocr_service = ocr_provider.map(|provider| {
            Arc::new(OCRService::new(provider, documents.clone()))
        });

        // ImportService (depends on ExpenseService, DocumentService, OCRService)
        let import = Arc::new(ImportService::new(
            expenses.clone(),
            documents.clone(),
            ocr_service.clone(),
        ));

        Ok(Self {
            // ... all existing fields ...
            import,
            ocr_service,
        })
    }
}

// --- OCR infrastructure ---

/// Adapter that wraps an API-layer OcrProvider to satisfy the core OCRProvider trait.
struct OcrProviderAdapter {
    inner: Box<dyn ApiOcrProvider>,
}

impl OCRProvider for OcrProviderAdapter {
    fn process_image(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
        self.inner.process_image(data, content_type).map_err(|e| {
            log::error!("OCR provider error: {e}");
            DomainError::InvalidInput
        })
    }
}

/// Construct an OCR provider from config. Returns None if not configured.
fn create_ocr_provider(config: &OcrConfig) -> Option<Arc<dyn OCRProvider>> {
    let api_key = config.api_key.as_deref().unwrap_or("");
    if api_key.is_empty() {
        log::info!("OCR not configured: no api_key in [ocr] section");
        return None;
    }

    let provider_name = config.provider.as_deref().unwrap_or("openai");
    let model = config.model.as_deref().unwrap_or("");

    let api_provider: Box<dyn ApiOcrProvider> = match provider_name {
        "anthropic" | "claude" => {
            let mut p = AnthropicProvider::new(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            Box::new(p)
        }
        "openrouter" => {
            let mut p = OpenAIProvider::new_openrouter(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            Box::new(p)
        }
        _ => {
            // Default to OpenAI-compatible API (works for openai, gemini, mistral with base_url override)
            let mut p = OpenAIProvider::new_openai(api_key, model);
            if let Some(ref url) = config.base_url {
                if !url.is_empty() {
                    p = p.with_base_url(url);
                }
            }
            if provider_name != "openai" {
                log::warn!(
                    "Unknown OCR provider '{provider_name}', using OpenAI-compatible API. \
                     Set base_url if the provider has a custom endpoint."
                );
            }
            Box::new(p)
        }
    };

    log::info!("OCR provider configured: {provider_name}");
    Some(Arc::new(OcrProviderAdapter {
        inner: api_provider,
    }))
}
```

### 3. `rust/crates/zfaktury-app/src/main.rs` -- cmd_gui() changes

```rust
fn cmd_gui(cli: Cli) {
    // Load config once.
    let config = match zfaktury_config::Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };

    // Resolve paths (CLI overrides config).
    let db_path = cli.db.clone().unwrap_or_else(|| config.database_path());
    let data_dir = if cli.db.is_some() {
        cli.db
            .as_ref()
            .unwrap()
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        config.data_dir()
    };

    ensure_parent_dir(&db_path);
    ensure_parent_dir_of(&data_dir);

    // Initialize services (now receives config for OCR, SMTP, etc.).
    let services = match AppServices::new(&db_path, &data_dir, &config) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Error initializing application: {e}");
            std::process::exit(1);
        }
    };

    // ... rest unchanged (parse initial_route, open GPUI window, etc.)
}
```

### 4. `rust/crates/zfaktury-core/src/service/ocr_svc.rs` -- Add process_bytes()

```rust
impl OCRService {
    pub fn new(provider: Arc<dyn OCRProvider>, documents: Arc<DocumentService>) -> Self {
        Self { provider, documents }
    }

    /// Process raw bytes through OCR without requiring a stored document.
    /// Used by views that have file bytes from rfd file picker.
    pub fn process_bytes(&self, data: &[u8], content_type: &str) -> Result<OCRResult, DomainError> {
        let supported = ["image/jpeg", "image/png", "application/pdf"];
        if !supported.contains(&content_type) {
            return Err(DomainError::InvalidInput);
        }
        if data.is_empty() {
            return Err(DomainError::InvalidInput);
        }
        self.provider.process_image(data, content_type)
    }

    /// Processes a stored document by ID through OCR.
    pub fn process_document(&self, document_id: i64) -> Result<OCRResult, DomainError> {
        if document_id == 0 {
            return Err(DomainError::InvalidInput);
        }
        let doc = self.documents.get_by_id(document_id)?;

        let supported = ["image/jpeg", "image/png", "application/pdf"];
        if !supported.contains(&doc.content_type.as_str()) {
            return Err(DomainError::InvalidInput);
        }

        let data = std::fs::read(&doc.storage_path).map_err(|e| {
            log::error!("reading document file {}: {e}", doc.storage_path);
            DomainError::NotFound
        })?;

        self.provider.process_image(&data, &doc.content_type)
    }
}
```

---

## Config Example

```toml
# ~/.zfaktury/config.toml

[ocr]
provider = "openai"           # openai | anthropic | claude | openrouter | gemini | mistral
api_key = "sk-..."            # required for OCR to be enabled
model = ""                    # optional, uses provider default (gpt-4o, claude-sonnet-4, etc.)
base_url = ""                 # optional, override API endpoint (useful for gemini/mistral)
```

### Provider defaults

| Provider value | Default model | Default base URL |
|----------------|---------------|------------------|
| `openai` | `gpt-4o` | `https://api.openai.com/v1/chat/completions` |
| `anthropic` / `claude` | `claude-sonnet-4-20250514` | `https://api.anthropic.com/v1/messages` |
| `openrouter` | `google/gemini-2.0-flash-001` | `https://openrouter.ai/api/v1/chat/completions` |
| `gemini` | `gpt-4o` (needs base_url override) | needs `base_url` set |
| `mistral` | `gpt-4o` (needs base_url override) | needs `base_url` set |

---

## Error Handling

| Scenario | Behavior |
|----------|----------|
| No `[ocr]` section in config | `ocr_service = None`, log info, OCR disabled silently |
| `[ocr]` present but `api_key` empty/missing | `ocr_service = None`, log info, OCR disabled silently |
| Unknown provider name | Warning log, fall back to OpenAI-compatible API |
| API key present but API call fails at runtime | `DomainError::InvalidInput` returned, view shows error message |
| File too large for OCR (>20MB base64 ~27MB) | Provider will likely return HTTP 413, mapped to error |
| Unsupported content type | `DomainError::InvalidInput` before API call |

Key principle: **Missing config = OCR disabled (not an error).** The app starts normally without OCR. Users see a yellow status badge ("OCR neni nastaveno") in expense import and investment document views.

---

## How Views Will Consume This

### ExpenseImportView (RFC-020)

```rust
pub struct ExpenseImportView {
    import_service: Arc<ImportService>,
    document_service: Arc<DocumentService>,
    ocr_service: Option<Arc<OCRService>>,  // NEW: from services.ocr_service
    // ...
}

// In RootView::navigate(), when creating ExpenseImportView:
Route::ExpenseImport => {
    let view = cx.new(|cx| ExpenseImportView::new(
        services.import.clone(),
        services.documents.clone(),
        services.ocr_service.clone(),  // Pass OCR
        cx,
    ));
}
```

### TaxInvestmentsView (RFC-023)

```rust
pub struct TaxInvestmentsView {
    income_service: Arc<InvestmentIncomeService>,
    document_service: Arc<InvestmentDocumentService>,
    ocr_service: Option<Arc<OCRService>>,  // NEW: from services.ocr_service
    // ...
}
```

Both views check `self.ocr_service.is_some()` to render either:
- Green badge: "OCR aktivni (provider: openai)"
- Yellow badge: "OCR neni nastaveno"

---

## Testing

### Unit tests (no real API key needed)

1. **`create_ocr_provider()` with various configs:**
   - `None` config -> returns `None`
   - Empty api_key -> returns `None`
   - Valid config with "openai" -> returns `Some(...)`
   - Valid config with "anthropic" -> returns `Some(...)`
   - Unknown provider -> returns `Some(...)` with warning log

2. **`OcrProviderAdapter` error mapping:**
   - API error -> `DomainError::InvalidInput`

3. **`OCRService::process_bytes()`:**
   - Empty data -> `DomainError::InvalidInput`
   - Unsupported content type -> `DomainError::InvalidInput`
   - Valid data + mock provider -> returns `OCRResult`

### Mock provider for tests

```rust
#[cfg(test)]
struct MockOcrProvider {
    result: Result<OCRResult, DomainError>,
}

#[cfg(test)]
impl OCRProvider for MockOcrProvider {
    fn process_image(&self, _data: &[u8], _content_type: &str) -> Result<OCRResult, DomainError> {
        self.result.clone()
    }
}
```

Note: `OCRResult` needs `Clone` derive (it already has it via `#[derive(Debug, Clone)]` in `domain/ocr.rs`).

### Integration test (wiremock)

The existing tests in `zfaktury-api/src/ocr.rs` already cover API provider behavior with wiremock. No new integration tests needed for the adapter layer.

---

## Dependency Graph

```
zfaktury-config  (OcrConfig struct)
       |
       v
zfaktury-app/app.rs  (create_ocr_provider, OcrProviderAdapter)
       |
       +---> zfaktury-api/ocr.rs  (AnthropicProvider, OpenAIProvider -- actual HTTP calls)
       |
       +---> zfaktury-core/ocr_svc.rs  (OCRProvider trait, OCRService)
       |           |
       |           +---> zfaktury-core/document_svc.rs  (for process_document)
       |           |
       |           +---> zfaktury-domain/ocr.rs  (OCRResult, OCRItem)
       |
       +---> zfaktury-core/import_svc.rs  (receives Option<Arc<OCRService>>)
```

---

## Implementation Order

1. Add `rfd` + `zfaktury-api` dependencies to `Cargo.toml`
2. Add `process_bytes()` to `OCRService` in `ocr_svc.rs`
3. Add `OcrProviderAdapter`, `create_ocr_provider()` to `app.rs`
4. Add `ocr_service` field to `AppServices` struct
5. Change `AppServices::new()` signature to accept `&Config`
6. Wire OCR construction inside `AppServices::new()`
7. Update `ImportService` construction to pass `ocr_service`
8. Update `main.rs` `cmd_gui()` to load config once and pass to `AppServices::new()`
9. Build and test: `cargo build --workspace && cargo test --workspace`

Estimated scope: ~120 lines of new code, ~20 lines modified.
