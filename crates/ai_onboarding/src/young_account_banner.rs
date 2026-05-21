use gpui::{IntoElement, ParentElement};
use ui::{Banner, prelude::*};

#[derive(IntoElement)]
pub struct YoungAccountBanner;

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // TODO(ideer-rename): Ideer does not have its own billing
        // support contact. The original upstream copy referred users to
        // billing-support@zed.dev; restore an Ideer-owned contact here
        // once one exists.
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "GitHub accounts created fewer than 30 days ago are not eligible for the hosted Pro trial. Hosted plans and trials are not available in Ideer yet; use the agent with your own provider API keys instead.";

        let label = div()
            .w_full()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .child(YOUNG_ACCOUNT_DISCLAIMER);

        div()
            .max_w_full()
            .my_1()
            .child(Banner::new().severity(Severity::Warning).child(label))
    }
}
