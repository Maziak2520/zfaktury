use gpui::*;

use crate::help_content::HelpTopic;
use crate::theme::ZfColors;

/// Event emitted when the help drawer should close.
pub struct HelpDrawerClosed;

/// Side-panel overlay displaying a help topic with simple + legal sections.
pub struct HelpDrawerView {
    topic: HelpTopic,
}

impl EventEmitter<HelpDrawerClosed> for HelpDrawerView {}

impl HelpDrawerView {
    pub fn new(topic: HelpTopic) -> Self {
        Self { topic }
    }
}

impl Render for HelpDrawerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Split paragraphs by double newline
        let simple_paragraphs: Vec<&str> = self.topic.simple.split("\n\n").collect();
        let legal_paragraphs: Vec<&str> = self.topic.legal.split("\n\n").collect();

        let mut body = div()
            .id("help-drawer-body")
            .overflow_y_scroll()
            .px(px(20.0))
            .py(px(20.0))
            .flex()
            .flex_col()
            .gap_4();

        // "Jednoduse" section
        body = body.child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(ZfColors::TEXT_MUTED))
                .child("JEDNODU\u{0160}E"),
        );

        let mut simple_card = div()
            .p_4()
            .bg(rgb(ZfColors::SURFACE_HOVER))
            .rounded_md()
            .flex()
            .flex_col()
            .gap_2();
        for para in &simple_paragraphs {
            simple_card = simple_card.child(
                div()
                    .text_sm()
                    .text_color(rgb(ZfColors::TEXT_PRIMARY))
                    .child(para.to_string()),
            );
        }
        body = body.child(simple_card);

        // Divider
        body = body.child(
            div()
                .h(px(1.0))
                .bg(rgb(ZfColors::BORDER))
                .my_2(),
        );

        // "Pravni ramec" section
        body = body.child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(ZfColors::TEXT_MUTED))
                .child("PR\u{00c1}VN\u{00cd} R\u{00c1}MEC"),
        );

        for para in &legal_paragraphs {
            body = body.child(
                div()
                    .text_sm()
                    .text_color(rgb(ZfColors::TEXT_SECONDARY))
                    .child(para.to_string()),
            );
        }

        // Full overlay
        deferred(
            div()
                .id("help-drawer-overlay")
                .absolute()
                .inset_0()
                .flex()
                .occlude()
                // Backdrop click = close
                .on_click(cx.listener(|_this, _event: &ClickEvent, _window, cx| {
                    cx.emit(HelpDrawerClosed);
                }))
                .child(
                    // Backdrop
                    div()
                        .absolute()
                        .inset_0()
                        .bg(rgba(0x000000aa)),
                )
                .child(
                    // Panel (right side)
                    div()
                        .id("help-drawer-panel")
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .right_0()
                        .w(px(400.0))
                        .bg(rgb(ZfColors::SURFACE))
                        .border_l_1()
                        .border_color(rgb(ZfColors::BORDER))
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        // Prevent click-through to backdrop
                        .on_click(|_event: &ClickEvent, _window, _cx| {})
                        // Header
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .px(px(20.0))
                                .py(px(16.0))
                                .border_b_1()
                                .border_color(rgb(ZfColors::BORDER))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(ZfColors::TEXT_PRIMARY))
                                        .child(self.topic.title.to_string()),
                                )
                                .child(
                                    div()
                                        .id("help-drawer-close")
                                        .cursor_pointer()
                                        .text_sm()
                                        .text_color(rgb(ZfColors::TEXT_MUTED))
                                        .hover(|s| s.text_color(rgb(ZfColors::TEXT_PRIMARY)))
                                        .on_click(cx.listener(
                                            |_this, _event: &ClickEvent, _window, cx| {
                                                cx.emit(HelpDrawerClosed);
                                            },
                                        ))
                                        .child("\u{2715}"),
                                ),
                        )
                        // Body
                        .child(body),
                ),
        )
        .with_priority(3)
    }
}
