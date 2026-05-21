use extension_host::ExtensionStore;
use gpui::{App, ClipboardItem, PromptLevel, Window, actions};
use system_specs::{CopySystemSpecsIntoClipboard, SystemSpecs};
use util::ResultExt;
use workspace::Workspace;
use zed_actions::feedback::{EmailZed, FileBugReport, RequestFeature};

actions!(
    zed,
    [
        /// Opens the Ideer repository on GitHub.
        OpenZedRepo,
        /// Copies installed extensions to the clipboard for bug reports.
        CopyInstalledExtensionsIntoClipboard
    ]
);

// TODO(ideer-rename): Ideer does not have its own bug tracker,
// feature request system, support email, or public repository URL
// yet. The `RequestFeature`, `FileBugReport`, `EmailZed`, and
// `OpenZedRepo` actions remain registered for backwards
// compatibility (so existing keybindings and command-palette
// entries do not error), but their handlers now show an
// "unavailable" prompt instead of opening upstream Zed pages.
const FEEDBACK_UNAVAILABLE_TITLE: &str = "Feedback is not available yet";
const FEEDBACK_UNAVAILABLE_BODY: &str = concat!(
    "Ideer does not have its own bug tracker, feature request system, ",
    "support email, or public repository URL yet. Once an Ideer-owned ",
    "destination exists, these actions will reopen.\n\n",
    "If you want to report a bug that also reproduces on upstream Zed, ",
    "report it at github.com/zed-industries/zed."
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, window, cx| {
                let specs = SystemSpecs::new(window, cx);

                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await.to_string();

                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(specs.clone()))
                    })
                    .log_err();

                    cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    )
                    .await
                })
                .detach();
            })
            .register_action(|_, _: &CopyInstalledExtensionsIntoClipboard, window, cx| {
                let clipboard_text = format_installed_extensions_for_clipboard(cx);
                cx.write_to_clipboard(ClipboardItem::new_string(clipboard_text.clone()));
                drop(window.prompt(
                    PromptLevel::Info,
                    "Copied into clipboard",
                    Some(&clipboard_text),
                    &["OK"],
                    cx,
                ));
            })
            .register_action(|_, _: &RequestFeature, window, cx| {
                show_feedback_unavailable(window, cx);
            })
            .register_action(|_, _: &FileBugReport, window, cx| {
                show_feedback_unavailable(window, cx);
            })
            .register_action(|_, _: &EmailZed, window, cx| {
                show_feedback_unavailable(window, cx);
            })
            .register_action(|_, _: &OpenZedRepo, window, cx| {
                show_feedback_unavailable(window, cx);
            });
    })
    .detach();
}

fn show_feedback_unavailable(window: &mut Window, cx: &mut App) {
    drop(window.prompt(
        PromptLevel::Info,
        FEEDBACK_UNAVAILABLE_TITLE,
        Some(FEEDBACK_UNAVAILABLE_BODY),
        &["OK"],
        cx,
    ));
}

fn format_installed_extensions_for_clipboard(cx: &mut App) -> String {
    let store = ExtensionStore::global(cx);
    let store = store.read(cx);
    let mut lines = Vec::with_capacity(store.extension_index.extensions.len());

    for (extension_id, entry) in store.extension_index.extensions.iter() {
        let line = format!(
            "- {} ({}) v{}{}",
            entry.manifest.name,
            extension_id,
            entry.manifest.version,
            if entry.dev { " (dev)" } else { "" }
        );
        lines.push(line);
    }

    lines.sort();

    if lines.is_empty() {
        return "No extensions installed.".to_string();
    }

    format!(
        "Installed extensions ({}):\n{}",
        lines.len(),
        lines.join("\n")
    )
}
