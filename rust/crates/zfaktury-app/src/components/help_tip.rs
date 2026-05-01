use gpui::*;

use crate::theme::ZfColors;

/// Renders a small "?" circle button that triggers a help drawer.
pub fn render_help_tip(
    id: impl Into<ElementId>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    div()
        .id(id.into())
        .flex()
        .items_center()
        .justify_center()
        .w(px(16.0))
        .h(px(16.0))
        .rounded_full()
        .border_1()
        .border_color(rgb(ZfColors::BORDER))
        .cursor_pointer()
        .text_color(rgb(ZfColors::TEXT_MUTED))
        .text_xs()
        .hover(|s| {
            s.text_color(rgb(ZfColors::TEXT_SECONDARY))
                .border_color(rgb(ZfColors::TEXT_SECONDARY))
        })
        .on_click(on_click)
        .child("?")
}
