use gpui::{IntoElement, ParentElement};
use ui::{List, ListBulletItem, prelude::*};

// TODO(ideer-rename): These plan descriptions are placeholders that
// preserve the shape upstream Zed UI expects. Ideer does not currently
// offer paid plans, hosted token grants, or a billing system. The
// surrounding UI hides trial / upgrade buttons (see `ai_onboarding.rs`
// and `end_trial_upsell.rs`), so these bullet lists are effectively
// reference text. Replace the copy here once Ideer-owned plans exist.

/// Centralized placeholder definitions for AI plans. Used by the
/// upstream Zed onboarding surfaces while Ideer's billing story is
/// undefined.
pub struct PlanDefinitions;

impl PlanDefinitions {
    pub fn free_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(
                "Bring your own API keys for any supported provider",
            ))
            .child(ListBulletItem::new("Unlimited use of external agents"))
            .child(ListBulletItem::new(
                "No hosted models, billing, or trial included",
            ))
    }

    pub fn pro_trial(&self, period: bool) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(
                "Hosted model access is not available in Ideer yet",
            ))
            .child(ListBulletItem::new(
                "Use the agent with your own provider API keys instead",
            ))
            .when(period, |this| {
                this.child(ListBulletItem::new(
                    "No trial period is offered",
                ))
            })
    }

    pub fn pro_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(
                "Paid plans are not available in Ideer yet",
            ))
            .child(ListBulletItem::new(
                "Bring your own API keys to use the agent today",
            ))
    }

    pub fn business_plan(&self) -> impl IntoElement {
        List::new().child(ListBulletItem::new(
            "Business plans are not available in Ideer yet",
        ))
    }

    pub fn student_plan(&self) -> impl IntoElement {
        List::new().child(ListBulletItem::new(
            "Student plans are not available in Ideer yet",
        ))
    }
}
