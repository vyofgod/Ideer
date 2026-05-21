//! Ideer workspace layout preset support.
//!
//! Adds the durable `ideer.layout_preset` setting and the
//! VS Code-familiar left activity bar. The activity bar is only
//! rendered when the user has explicitly chosen
//! [`IdeerLayoutPreset::VscodeFamiliar`]; otherwise the workspace
//! renders identically to upstream Zed so that users who pick
//! `Zed Native` (or who never run onboarding) see no extra chrome.
//!
//! The activity bar dispatches existing panel actions by name through
//! the GPUI action registry. This avoids adding workspace-crate
//! dependencies on every panel crate while still wiring every visible
//! button to a real action — per the engineering constraint that the
//! activity bar must not contain fake buttons.

use anyhow::anyhow;
use gpui::{AnyElement, App, Context, Hsla, IntoElement, ParentElement, Styled, Window, div, px};
use settings::{RegisterSetting, Settings};
use theme::ActiveTheme;
use ui::{ButtonCommon, Clickable, IconButton, IconButtonShape, IconName, IconSize, Tooltip};

use crate::Workspace;

/// Resolved layout preset.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdeerLayoutPreset {
    /// VS Code-familiar shell.
    VscodeFamiliar,
    /// Upstream Zed layout (also used when the user has not yet picked
    /// a preset during onboarding).
    ZedNative,
}

impl IdeerLayoutPreset {
    /// Whether the left activity bar should be rendered in this
    /// preset.
    pub fn shows_activity_bar(self) -> bool {
        matches!(self, IdeerLayoutPreset::VscodeFamiliar)
    }
}

#[derive(Clone, Copy, Debug, RegisterSetting)]
pub struct IdeerLayoutSettings {
    pub preset: IdeerLayoutPreset,
}

impl Settings for IdeerLayoutSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let preset = match content.ideer_layout_preset {
            Some(settings::IdeerLayoutPresetContent::VscodeFamiliar) => {
                IdeerLayoutPreset::VscodeFamiliar
            }
            // Treat both an explicit `zed_native` and an unset value as
            // ZedNative so users who never ran onboarding see no
            // change in behavior.
            Some(settings::IdeerLayoutPresetContent::ZedNative) | None => {
                IdeerLayoutPreset::ZedNative
            }
        };
        Self { preset }
    }
}

/// One entry in the left activity bar.
struct ActivityBarItem {
    id: &'static str,
    icon: IconName,
    label: &'static str,
    /// Action to dispatch when clicked. Looked up in the GPUI action
    /// registry by name so this crate doesn't need to depend on
    /// project_panel / agent_ui / git_ui / debugger_ui / extensions_ui.
    action_name: &'static str,
}

const ACTIVITY_BAR_ITEMS: &[ActivityBarItem] = &[
    ActivityBarItem {
        id: "ideer-activity-bar-explorer",
        icon: IconName::FileTree,
        label: "Explorer",
        action_name: "project_panel::ToggleFocus",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-search",
        icon: IconName::MagnifyingGlass,
        label: "Search",
        action_name: "pane::DeploySearch",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-source-control",
        icon: IconName::GitBranch,
        label: "Source Control",
        action_name: "git_panel::ToggleFocus",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-run-and-debug",
        icon: IconName::Debug,
        label: "Run and Debug",
        action_name: "debug_panel::ToggleFocus",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-extensions",
        icon: IconName::Blocks,
        label: "Extensions",
        action_name: "zed::Extensions",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-agents",
        icon: IconName::ZedAssistant,
        label: "Agents",
        // Focus / toggle the right-docked AI panel. In `vscode_familiar`
        // this reveals the AI panel without replacing the left file
        // sidebar (see UI_LAYOUT_PRESETS.md "Activity Bar
        // Requirements").
        action_name: "agent::ToggleFocus",
    },
    ActivityBarItem {
        id: "ideer-activity-bar-settings",
        icon: IconName::Settings,
        label: "Settings",
        action_name: "zed::OpenSettings",
    },
];

/// Render the Ideer left activity bar.
///
/// Returns `None` when the active preset is not [`IdeerLayoutPreset::VscodeFamiliar`],
/// so that `Zed Native` users see no extra chrome.
pub fn render_activity_bar(
    _workspace: &Workspace,
    _window: &mut Window,
    cx: &App,
) -> Option<AnyElement> {
    if !IdeerLayoutSettings::get_global(cx)
        .preset
        .shows_activity_bar()
    {
        return None;
    }

    let bar_bg: Hsla = cx.theme().colors().status_bar_background;
    let border: Hsla = cx.theme().colors().border;

    let mut container = div()
        .flex()
        .flex_col()
        .h_full()
        .w(px(44.))
        .py_1()
        .gap_1()
        .items_center()
        .flex_none()
        .bg(bar_bg)
        .border_r_1()
        .border_color(border);

    for item in ACTIVITY_BAR_ITEMS {
        let action_name = item.action_name;
        let label = item.label;
        container = container.child(
            IconButton::new(item.id, item.icon)
                .icon_size(IconSize::Medium)
                .shape(IconButtonShape::Square)
                .tooltip(Tooltip::text(item.label))
                .on_click(move |_, window, cx| {
                    dispatch_named_action(action_name, label, window, cx)
                }),
        );
    }

    Some(container.into_any_element())
}

/// Build and dispatch an action by name. Errors are logged so they are
/// visible during development; a user-facing notification would require
/// a `Workspace` handle which we don't always have inside an
/// `on_click` closure.
fn dispatch_named_action(
    action_name: &'static str,
    label: &'static str,
    window: &mut Window,
    cx: &mut App,
) {
    match cx.build_action(action_name, None) {
        Ok(action) => window.dispatch_action(action, cx),
        Err(err) => {
            log::error!(
                "Ideer activity bar: failed to build action {action_name:?} for {label:?} button: {err:#}"
            );
        }
    }
}

/// Apply the VS Code-familiar default dock layout to `workspace`.
///
/// Opens the left dock (project panel area), the right dock (AI panel
/// area), and the bottom dock (terminal area) and focuses the project
/// panel so the file tree feels anchored on the left. Called from the
/// onboarding picker when the user selects "VS Code Familiar".
///
/// Existing user dock state is respected to the extent the dock API
/// allows: this only opens docks, it does not close or rearrange
/// panels the user already chose.
pub fn apply_vscode_familiar_layout(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> anyhow::Result<()> {
    use crate::dock::DockPosition;

    let mut any_dock_opened = false;
    for position in [DockPosition::Left, DockPosition::Right, DockPosition::Bottom] {
        let dock = match position {
            DockPosition::Left => workspace.left_dock().clone(),
            DockPosition::Right => workspace.right_dock().clone(),
            DockPosition::Bottom => workspace.bottom_dock().clone(),
        };
        dock.update(cx, |dock, cx| {
            if !dock.is_open() {
                dock.set_open(true, window, cx);
                any_dock_opened = true;
            }
        });
    }

    // Focus the project panel by dispatching the existing action so we
    // don't reach into the panel's internals from here.
    match cx.build_action("project_panel::ToggleFocus", None) {
        Ok(action) => window.dispatch_action(action, cx),
        Err(err) => {
            return Err(anyhow!(
                "failed to focus project panel after applying VS Code familiar layout: {err:#}"
            ));
        }
    }

    if any_dock_opened {
        workspace.serialize_workspace(window, cx);
    }
    Ok(())
}
