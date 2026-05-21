# UI Layout Presets

This document defines the first-start layout direction for Ideer. The goal is to let users choose between a familiar VS Code-style interface and a Zed-style interface without weakening the editor's core speed and simplicity.

## Product Goal

Ideer should support two clear starting experiences:

- `VS Code Familiar`: for users who are used to VS Code, Cursor, Windsurf, and similar editors.
- `Zed Native`: for users who prefer the original Zed layout and interaction model.

The first-run screen should ask the user which layout they want, then save that preference as the default workspace preset.

## First-Run Screen

The initial onboarding should include a layout choice step with two large options.

### VS Code Familiar

This option should feel comfortable for users who expect a VS Code-like shell.

Default layout:

- Left activity bar visible.
- Project files open in the left sidebar.
- Search, Git, Debug, Extensions, and Agents available from the left activity bar.
- Editor in the center.
- AI chat and agent panel on the right.
- Terminal, diagnostics, tasks, and output in the bottom panel.
- Tabs visible above the editor.
- Status bar visible at the bottom.

Primary user expectation:

- Files are always easy to reach on the left.
- AI chat is always easy to reach on the right.
- Tool shortcuts live in a narrow vertical bar on the far left.
- The editor center stays the main work area.

### Zed Native

This option should preserve the upstream Zed feeling.

Default layout:

- Keep Zed's existing panel behavior as much as possible.
- Avoid adding a persistent VS Code-style activity bar by default.
- Preserve command-palette-first workflows.
- Keep AI surfaces close to the current Zed agent panel behavior.
- Keep the interface lighter and less chrome-heavy.

Primary user expectation:

- The UI feels fast, minimal, and Zed-like.
- Users who already like Zed do not feel forced into a VS Code clone.
- Ideer additions should be available without changing the original layout too aggressively.

## Activity Bar Requirements

The VS Code Familiar preset needs a real left activity bar.

Initial buttons:

- Explorer
- Search
- Source Control
- Run and Debug
- Extensions
- Agents
- Settings

Behavior:

- One click switches the active left sidebar view.
- Active item has a clear visual indicator.
- Hover shows a tooltip.
- Right click can show context actions later.
- Keyboard shortcuts should dispatch the same actions as the buttons.
- The Agents button should focus or reveal the AI panel on the right, not replace the file sidebar by default.

## Right AI Panel Requirements

For the VS Code Familiar preset, the AI panel should be right-docked by default.

Required sections:

- Current chat or agent thread.
- Composer.
- Model selector.
- Context attachment controls.
- Tool permission prompts.
- Subagent or task activity summary.

Behavior:

- The AI panel should stay open across restarts if the user leaves it open.
- The user should be able to hide, reveal, resize, and focus it quickly.
- Agent diffs and code changes should still appear in the main editor area when review is needed.

## Layout Preset Setting

Add a durable user setting for the selected shell style.

Suggested shape:

```json
{
  "ideer.layout_preset": "vscode_familiar"
}
```

Allowed values:

- `vscode_familiar`
- `zed_native`

Future values:

- `ai_first`
- `minimal`

## Implementation Areas To Investigate

- `crates/workspace`: panel layout, docks, pane groups, workspace restoration.
- `crates/sidebar`: sidebar behavior and possible activity bar integration.
- `crates/project_panel`: Explorer-like file tree behavior.
- `crates/search`: Search panel integration.
- `crates/git_ui`: Source Control panel integration.
- `crates/debugger_ui`: Run and Debug entry point.
- `crates/extensions_ui`: Extensions entry point.
- `crates/agent_ui`: right-docked AI chat and agent panel behavior.
- `crates/title_bar`: top-level shell controls.
- `crates/zed_actions`: shared actions and shortcuts for activity bar buttons.
- `crates/settings`: persistent layout preset setting.
- `crates/onboarding`: first-run layout choice UI.

## MVP Plan

1. Add the `ideer.layout_preset` setting.
2. Add first-run UI with `VS Code Familiar` and `Zed Native`.
3. Make `VS Code Familiar` open project files on the left and agent UI on the right by default.
4. Add a narrow left activity bar with Explorer and Agents first.
5. Add Search, Source Control, Debug, Extensions, and Settings buttons after the first version works.
6. Add tests or smoke checks for startup layout restoration.

## Open Questions

- Should the app default to `VS Code Familiar` for all new users, or ask on first launch?
- Should existing Zed users migrating to Ideer default to `Zed Native`?
- Should the AI panel always be right-docked in `VS Code Familiar`, or only when AI is enabled?
- Should layout presets be project-specific, user-global, or both?
- Should activity bar ordering be configurable?
