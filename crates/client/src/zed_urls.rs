//! Helpers for constructing URLs to Ideer / upstream-Zed web pages.
//!
//! Ideer does not yet own an account system, billing surface, trial
//! flow, legal pages, or product documentation site. To avoid silently
//! sending users to Zed-owned destinations, every URL constructor that
//! used to point at zed.dev returns `None`. Callers are expected to
//! hide the corresponding button / banner / link when the URL is
//! unavailable, instead of opening a dead page.
//!
//! Once Ideer-owned destinations exist, restore the URLs here and
//! delete this notice.

use gpui::App;

/// Returns the URL to the account page, if a real destination exists.
///
/// Ideer does not currently have an account system. Returns `None`.
pub fn account_url(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to start a trial, if a real destination exists.
///
/// Ideer does not currently offer a trial. Returns `None`.
pub fn start_trial_url(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to upgrade to a paid plan, if a real destination
/// exists.
///
/// Ideer does not currently offer paid plans. Returns `None`.
pub fn upgrade_to_zed_pro_url(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to the product terms of service, if a real
/// destination exists.
///
/// Ideer does not currently host its own legal pages. Returns `None`.
pub fn terms_of_service(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to the AI privacy and security docs, if a real
/// destination exists. Returns `None` while Ideer-owned docs do not
/// exist.
pub fn ai_privacy_and_security(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to the edit prediction documentation, if a real
/// destination exists. Returns `None` while Ideer-owned docs do not
/// exist.
pub fn edit_prediction_docs(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to the ACP registry blog post, if a real
/// destination exists. Returns `None` while Ideer-owned posts do not
/// exist.
pub fn acp_registry_blog(_cx: &App) -> Option<String> {
    None
}

/// Returns the URL to the Parallel Agents blog post, if a real
/// destination exists. Returns `None` while Ideer-owned posts do not
/// exist.
pub fn parallel_agents_blog(_cx: &App) -> Option<String> {
    None
}

/// Returns the deep-link URL for a shared agent thread.
///
/// New shared-thread links use the `ideer://` scheme; the deep-link
/// parser in `crates/zed/src/zed/open_listener.rs` also accepts the
/// legacy `zed://` prefix for backwards compatibility.
pub fn shared_agent_thread_url(session_id: &str) -> String {
    format!("ideer://agent/shared/{}", session_id)
}
