use gpui::*;

use crate::theme::ZfColors;

/// Standard labeled field (backward-compatible replacement for per-view copies).
pub fn render_labeled_field(label: &str, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY))
                .child(label.to_string()),
        )
        .child(child)
}

/// Labeled field with optional "?" help tip and/or hint text below.
pub fn render_labeled_field_with_help(
    label: &str,
    child: impl IntoElement,
    help_tip: Option<Stateful<Div>>,
    hint: Option<&str>,
) -> Div {
    // Label row (label text + optional help tip)
    let mut label_row = div()
        .flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ZfColors::TEXT_SECONDARY))
                .child(label.to_string()),
        );
    if let Some(tip) = help_tip {
        label_row = label_row.child(tip);
    }

    let mut wrapper = div().flex().flex_col().gap_1().child(label_row).child(child);

    if let Some(hint_text) = hint {
        wrapper = wrapper.child(
            div()
                .text_xs()
                .text_color(rgb(ZfColors::TEXT_MUTED))
                .child(hint_text.to_string()),
        );
    }

    wrapper
}

/// Page header with title + optional description subtitle.
pub fn render_page_header(title: &str, description: Option<&str>) -> Div {
    let mut header = div().flex().flex_col().gap_1().child(
        div()
            .text_xl()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(ZfColors::TEXT_PRIMARY))
            .child(title.to_string()),
    );
    if let Some(desc) = description {
        header = header.child(
            div()
                .text_sm()
                .text_color(rgb(ZfColors::TEXT_SECONDARY))
                .child(desc.to_string()),
        );
    }
    header
}

/// Card wrapper with title and content.
pub fn render_card(title: &str, content: Div) -> Div {
    div()
        .p_4()
        .bg(rgb(ZfColors::SURFACE))
        .rounded_md()
        .border_1()
        .border_color(rgb(ZfColors::BORDER))
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(ZfColors::TEXT_PRIMARY))
                .child(title.to_string()),
        )
        .child(content)
}
