---
name: rust-gpui-development
description: Development skill for Rust GUI applications using GPUI framework (Zed's UI framework). Use when working on GPUI-based desktop applications - creating views, managing state, handling actions, or reviewing/refactoring existing code. Triggers on Rust GUI development, GPUI views, Entity/Context usage, or any desktop app built with GPUI.
---

# Rust GPUI Development

Patterns and API reference for building Rust applications with the GPUI framework.

## Reference Files

Load based on current task:

| Task | Reference | Content |
|------|-----------|---------|
| Working with GPUI | [gpui-patterns.md](references/gpui-patterns.md) | App architecture, context types, entities, actions, rendering, modals, drag & drop, menus, tooltips, testing |
| Designing state flow | [state-management.md](references/state-management.md) | Global state, view-model separation, state machines, derived state, unidirectional flow |

## Quick Reference

### GPUI Context Types

| Context | Scope | When Used |
|---------|-------|-----------|
| `&mut App` | Global | App init, global operations |
| `&mut Window` | Window | Rendering, events |
| `&mut Context<T>` | Entity | Entity methods, `render()` |

### View Structure

```rust
pub struct MyView {
    focus_handle: FocusHandle,
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MyView")
            .on_action(cx.listener(Self::handle_action))
            .child(/* ... */)
    }
}

impl MyView {
    fn handle_action(&mut self, _: &MyAction, window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }
}
```

### Actions

```rust
// Simple actions
actions!(my_namespace, [DoSomething, Cancel]);

// Parameterized actions
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = my_namespace)]
pub struct SelectItem { pub index: usize }

// Register in render() via cx.listener()
div().on_action(cx.listener(Self::handle_action))
```

### Global State

```rust
impl Global for AppSettings {}
cx.set_global(AppSettings::new());
let settings = cx.global::<AppSettings>();
cx.observe_global::<AppSettings>(|this, cx| { cx.notify(); }).detach();
```

### Critical Rules

1. **Never store `cx`** - always pass through method parameters
2. **Call `.detach()`** on subscriptions and observations
3. **Call `cx.notify()`** after state changes that affect rendering
4. **Use `cx.listener()`** for action/click handlers in `render()` - wraps entity access
5. **`window: &mut Window`** is a separate parameter in all entity callbacks

## ZFaktury Project-Specific Patterns

### Service Injection

Views receive services as `Arc<Service>` from `AppServices`:

```rust
pub struct InvoiceListView {
    service: Arc<InvoiceService>,
    loading: bool,
    error: Option<String>,
    items: Vec<Invoice>,
}

impl InvoiceListView {
    pub fn new(service: Arc<InvoiceService>, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            service,
            loading: true,
            error: None,
            items: Vec::new(),
        };
        view.load_data(cx);
        view
    }
}
```

### Background Data Loading

All blocking repo calls MUST use `cx.background_executor()`:

```rust
fn load_data(&mut self, cx: &mut Context<Self>) {
    let service = self.service.clone();
    cx.spawn(async move |this, cx| {
        let result = cx
            .background_executor()
            .spawn(async move { service.list_all() })
            .await;

        this.update(cx, |this, cx| {
            this.loading = false;
            match result {
                Ok(items) => this.items = items,
                Err(e) => this.error = Some(format!("Chyba: {e}")),
            }
            cx.notify();
        }).ok();
    }).detach();
}
```

### Navigation

Views emit `NavigateEvent` to request route changes. The root view subscribes and swaps views:

```rust
use crate::navigation::{NavigateEvent, Route};

// In a click handler:
cx.emit(NavigateEvent(Route::InvoiceDetail(id)));

// Root view subscribes:
cx.subscribe_in(&view, window, |this, _, event: &NavigateEvent, window, cx| {
    this.navigate(event.0.clone(), window, cx);
}).detach();
```

### Theme Colors

Use `ZfColors` constants from `crate::theme` for all colors:

```rust
use crate::theme::ZfColors;

div()
    .bg(rgb(ZfColors::SURFACE))
    .text_color(rgb(ZfColors::TEXT_PRIMARY))
    .border_color(rgb(ZfColors::BORDER))
```

### Shared Components

Reusable components live in `crate::components`:
- `button::render_button(ButtonVariant, ...)` - styled buttons
- `status_badge::render_status_badge(...)` - invoice/expense status badges
- `invoice_items_editor` - invoice line item editing

### Formatting Utilities

Use `crate::util::format` for display formatting:
- `format_amount(Amount)` - CZK currency formatting
- `format_date(NaiveDate)` - Czech date formatting
