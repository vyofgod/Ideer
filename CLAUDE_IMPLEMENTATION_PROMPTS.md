# Claude Implementation Prompts

## Prompt 1: Fix Product Gaps First

Read `PRODUCT_GAPS.md` completely before changing code. Treat it as the product readiness checklist for this Zed fork, which is being turned into Ideer.

Your first job is not to redesign the UI. Your first job is to fix the current product/fork hygiene problems that make the project look unfinished or unsafe to publish.

Work in this order:

1. Audit the repository for visible Zed identity leaks:
   - `Zed`
   - `zed.dev`
   - `zed-industries`
   - `zed://`
   - `ZED_*`
   - Zed account, billing, jobs, support, privacy, and cloud URLs
2. Do not blindly replace every match. Classify each usage:
   - keep because it is upstream attribution or compatibility,
   - rename because it is visible Ideer product UI,
   - disable/remove because it points to a Zed-owned service Ideer does not own yet,
   - leave for later with a clear TODO only if changing it now would be risky.
3. Fix the most visible product issues first:
   - root `README.md`,
   - `CONTRIBUTING.md`,
   - `CODE_OF_CONDUCT.md`,
   - app display names,
   - menus,
   - onboarding copy,
   - release channel names,
   - installer metadata,
   - obvious support/help/docs links.
4. Preserve license and upstream attribution. Do not hide that this is a Zed fork.
5. Do not invent a fake Ideer cloud, billing system, update server, privacy page, or support endpoint. If no Ideer-owned endpoint exists, disable the feature, point to local documentation, or mark it honestly as unavailable.
6. Keep changes conservative and compile-safe. This is a large Rust workspace.
7. Follow the repository rules:
   - prefer existing files,
   - do not create many small files,
   - avoid `unwrap()` in new production code,
   - propagate errors to the UI where relevant,
   - use `./script/clippy` instead of `cargo clippy` if clippy is needed.

Validation requirements:

1. Run at least:
   - `cargo check -p agent -p agent_ui -p zed`
2. If you touch broader workspace code, run a broader check if feasible.
3. Report exactly what you changed, what remains intentionally unchanged, and which Zed references are still present because they are compatibility or attribution.

Expected result:

The project should look like an intentional Ideer fork, not a half-renamed Zed checkout. It should not send users to Zed-owned account, billing, support, or update flows by accident. The code should still compile.

## Prompt 2: Implement UI Layout Presets

Read `UI_LAYOUT_PRESETS.md` completely before changing code. Also skim `PRODUCT_GAPS.md` so the UI work stays aligned with the product direction.

Your goal is to implement the first version of Ideer's startup layout choice:

- `VS Code Familiar`
- `Zed Native`

Do not try to rebuild the entire editor shell in one pass. Implement the smallest solid foundation that lets the product move toward the requested UX.

Required behavior:

1. Add a durable setting for the layout preset.
   Suggested value shape:

   ```json
   {
     "ideer.layout_preset": "vscode_familiar"
   }
   ```

   Allowed values:
   - `vscode_familiar`
   - `zed_native`

2. Add first-run or onboarding UI where users can choose:
   - `VS Code Familiar`: for users who expect VS Code/Cursor/Windsurf-like layout.
   - `Zed Native`: for users who want the original Zed-style layout.

3. For `VS Code Familiar`, move the default workspace direction toward:
   - files/project panel on the left,
   - editor in the center,
   - AI chat/agent panel on the right,
   - terminal and diagnostics at the bottom,
   - visible tabs/status bar where the existing system supports it.

4. Add the first version of a left activity bar only if it can be done cleanly within existing architecture. If a full activity bar is too large for one pass, implement the foundation and document the remaining steps.

5. The first useful activity bar buttons should be:
   - Explorer
   - Agents

   Then prepare the structure for:
   - Search
   - Source Control
   - Run and Debug
   - Extensions
   - Settings

6. The Agents button in `VS Code Familiar` should reveal or focus the right AI panel. It should not replace the left file sidebar by default.

7. Preserve `Zed Native` behavior as much as possible. Users choosing it should not feel forced into a VS Code clone.

Implementation areas to inspect first:

- `crates/workspace`
- `crates/sidebar`
- `crates/project_panel`
- `crates/search`
- `crates/git_ui`
- `crates/debugger_ui`
- `crates/extensions_ui`
- `crates/agent_ui`
- `crates/title_bar`
- `crates/zed_actions`
- `crates/settings`
- `crates/onboarding`

Engineering constraints:

- Do not perform broad rewrites.
- Prefer established workspace/panel abstractions.
- Keep the first version stable over clever.
- Do not add visual clutter to `Zed Native`.
- Avoid fake buttons that do nothing. If a button exists, wire it to a real action or hide it until it can be wired.
- Make errors visible where async UI operations can fail.

Validation requirements:

1. Run:
   - `cargo check -p workspace -p sidebar -p project_panel -p agent_ui -p onboarding -p zed`
2. If those packages are not sufficient because of changed dependencies, run:
   - `cargo check -p agent -p agent_ui -p zed`
3. If UI tests or focused tests already exist for changed components, run the relevant ones.

Expected result:

New users can choose between a VS Code-like starting shell and a Zed-like shell. The VS Code-like preset should make the file tree feel anchored on the left and the AI panel feel anchored on the right. The Zed-like preset should remain close to upstream Zed behavior.
