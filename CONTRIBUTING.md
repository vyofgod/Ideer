# Contributing to Ideer

Thanks for considering a contribution. Ideer is a fork of [Zed](https://github.com/zed-industries/zed) and is pre-release. The contribution process is intentionally light while the fork stabilizes.

## Before you start

- Read [`PRODUCT_GAPS.md`](./PRODUCT_GAPS.md) for the current product readiness checklist.
- Read [`CLAUDE.md`](./CLAUDE.md) for the in-repo Rust and GPUI coding guidelines that apply to all changes.
- If your change is in agent / subagent / AI surfaces, also read [`UI_LAYOUT_PRESETS.md`](./UI_LAYOUT_PRESETS.md) for the layout direction Ideer is moving toward.

There is no Ideer CLA. There is no separate Ideer code of conduct enforcement contact yet; please follow the spirit of the [Contributor Covenant](./CODE_OF_CONDUCT.md).

## Working with upstream Zed

Ideer tracks upstream Zed. A lot of the codebase is unchanged Zed code.

- Bugs that reproduce on upstream Zed are best reported and fixed [upstream](https://github.com/zed-industries/zed/issues).
- Ideer-specific bugs (branding, the agent / subagent flow, BYOK provider setup, deep-link parsing, onboarding) belong in this repository.
- When you change shared editor code, try to keep the diff small and reviewable so future upstream merges are not painful.

## Sending changes

The best way to get a change considered is a focused pull request that:

- Solves one thing. Small bug fixes and small enhancements are easier to land than large refactors.
- Includes a clear description of what is changing and why.
- Includes tests where reasonable, especially for parser changes, agent behavior, and permission logic.
- Includes screenshots or a screen recording when it touches UI.
- Follows the Rust guidelines in [`CLAUDE.md`](./CLAUDE.md) (no `unwrap()` in new production code, propagate errors with `?`, no silently discarded `Result`s, prefer existing files over creating many small ones).

## UI/UX checklist

When your change affects UI, walk through this checklist before you ask for review.

**Accessibility / ergonomics**
- Do all keyboard shortcuts work as intended?
- Are shortcuts discoverable (tooltips, menus, docs)?
- Do drag, context menus, resizing, and scrolling work?
- Does the feature look right in light mode and dark mode?
- Are hover, focus, and active states clear and consistent?
- Is it usable without a mouse?

**Responsiveness**
- Does the UI scale on narrow panes, short panes, and high-DPI displays?
- Does resizing keep the UI usable?
- Do dialogs and modals stay within the viewport?

**Platform consistency**
- Is the feature usable on Windows, Linux, and macOS?
- Does it respect system-level fonts, scaling, and input methods?

**Performance**
- User interactions have instant feedback. Slow operations (LLM calls, large diffs) show progress.
- Large files and big projects do not degrade noticeably.

**Consistency**
- Spacing, typography, and icons match the rest of the app.
- Terminology and tone are consistent.

**Error and offline behavior**
- What happens on the unhappy path (errors, rejected permissions, missing providers)?
- What happens offline and unauthenticated?
- Are error messages actionable?

**Discoverability**
- Can a first-time user figure it out without docs?
- Are power features discoverable but not intrusive?

## Things that are likely to be deferred

- Things that should be an extension (new languages, new themes).
- Off-the-shelf icon submissions for the default icon set.
- Large refactors that touch many crates without a clear product win.
- Non-trivial changes without tests.
- Output that looks AI-generated without a clear human owner who understands it.

## Building and running

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

The build instructions still reference the upstream `zed` binary and `zed` crate names; those are the same things in Ideer. Use `./script/clippy` instead of `cargo clippy` for lint runs.
