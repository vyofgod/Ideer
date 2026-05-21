# Product Gaps Before Publishing

This document captures the current gaps for publishing this Zed fork as an AI-first IDE product.

## Current State

- The core editor and AI crates have a strong base from Zed.
- `cargo check -p agent -p agent_ui -p zed` passes.
- Subagent foundations already exist: `spawn_agent`, subagent session context, UI cards, permission handling, persistence, and tests.
- The product identity is only partially renamed from Zed to Ideer.

## Must Fix Before Public GitHub Release

### Fork Identity

- Rewrite the root `README.md` so it describes Ideer, not Zed.
- Replace Zed badges, links, sponsorship text, hiring text, and installation links.
- Add a clear attribution section explaining that this is a fork of Zed.
- Decide the public product name, repository URL, app name, CLI name, and protocol scheme.
- Audit all visible `Zed`, `zed.dev`, `zed-industries`, `zed://`, and `ZED_*` references.
- Keep compatibility references only where intentionally required.

### Branding And Assets

- Ensure all app icons are production quality at the expected resolutions.
- Remove backup or temporary logo assets before release.
- Check macOS, Linux, Windows, snap, and desktop metadata for consistent naming.
- Verify app menu labels, about window text, onboarding, title bar, feedback, and updater UI.

### URL And Protocol Ownership

- Decide whether the product uses `ideer://`, keeps `zed://` compatibility, or supports both.
- Update deep-link parsing consistently across open listeners, install CLI, menus, tests, and docs.
- Replace Zed documentation URLs with Ideer URLs or local docs.
- Remove or disable links that still point users to Zed account, billing, jobs, and support pages.

### Cloud, Account, And Billing

- Decide whether Ideer is local-first, BYOK, or backed by a custom cloud service.
- Remove or replace Zed AI plan names, trials, upsells, token grants, and billing copy.
- Make provider setup explicit for OpenAI, Anthropic, Google, Ollama, LM Studio, OpenRouter, and other supported providers.
- Avoid shipping UI that promises a cloud service that does not exist yet.
- Ensure cloud errors reach the UI with meaningful product-specific messages.

### Agent And Subagent Product Experience

- Define the first real AI MVP instead of shipping a vague "agent swarm" promise.
- Add a visible task tree or timeline for parent agents and subagents.
- Show which agent is working, waiting for permission, failed, cancelled, or completed.
- Provide clear controls for stop, retry, resume, inspect, collapse, and export.
- Add role/profile support if the product promises specialized agents.
- Add coordination rules for parallel agents: disjoint file ownership, task boundaries, result synthesis, and duplicate-work prevention.
- Add cost, token, and context visibility for long-running agent work.
- Define a maximum depth, maximum parallelism, timeout policy, and cancellation policy.

### Swarm Architecture

- Introduce a real orchestration layer before marketing the feature as agent swarm.
- Track task graph state separately from chat transcript state.
- Persist parent-child relationships, agent labels, task status, produced diffs, and final summaries.
- Add aggregation logic for multiple subagent results.
- Add conflict detection when multiple agents edit related files.
- Add tests for concurrent subagents, cancellation propagation, failed subagents, resumed sessions, and permission handoff.

### Security And Permissions

- Treat terminal execution, file writes, destructive file operations, network access, and MCP/context server usage as first-class risk surfaces.
- Keep dangerous actions confirm-gated.
- Add audit log UI for agent actions.
- Make permission scopes understandable to normal users.
- Ensure subagents inherit only the permissions they should inherit.
- Add clear workspace trust behavior for project-local settings, MCP servers, language servers, and agent tools.
- Document privacy and security behavior honestly.

### Distribution And Release

- Replace Zed release workflows with Ideer release workflows.
- Decide release channels and names: stable, preview, nightly, dev.
- Set app identifiers, bundle identifiers, installer names, desktop file names, and update channel metadata.
- Prepare macOS signing and notarization.
- Prepare Windows signing and installer metadata.
- Prepare Linux AppImage, tarball, snap, flatpak, or package-manager strategy.
- Disable auto-update until an Ideer update server exists, or point it to a working Ideer endpoint.
- Verify clean install, update, uninstall, and protocol registration on each target platform.

### Legal And Compliance

- Preserve upstream license obligations.
- Keep third-party license generation working.
- Add clear fork attribution.
- Review AGPL/GPL/Apache obligations before binary distribution.
- Replace Zed legal pages with Ideer-specific policy pages only when they are real.
- Do not leave links to Zed privacy, terms, billing, or support pages unless intentionally referencing upstream.

### Documentation

- Add a concise public README:
  - What Ideer is.
  - How it differs from Zed.
  - Current status.
  - How to build.
  - How to run.
  - How AI providers are configured.
  - What is experimental.
  - License and attribution.
- Add an AI guide for agent, subagent, and future swarm workflows.
- Add a provider setup guide.
- Add a permission and security guide.
- Add a release/build guide for maintainers.
- Remove or rewrite Zed-specific docs before linking them publicly.

### Testing And Validation

- Keep `cargo check -p agent -p agent_ui -p zed` green.
- Run `./script/clippy` before release.
- Run focused tests for `agent`, `agent_ui`, `agent_servers`, `acp_thread`, and `acp_tools`.
- Run UI smoke checks for onboarding, model setup, agent panel, subagent cards, permissions, and settings.
- Test with at least one cloud provider and one local provider.
- Test first-run behavior without any API keys.
- Test offline behavior.
- Test cancellation and recovery during long-running agent work.
- Test protocol links and CLI installation.

## Current High-Risk Files And Areas

- `README.md`: still presents the project as Zed.
- `CONTRIBUTING.md`: still describes Zed contribution process and CLA.
- `CODE_OF_CONDUCT.md`: points to Zed's code of conduct page.
- `crates/client/src/zed_urls.rs`: still owns Zed URL construction.
- `crates/zed/src/zed/open_listener.rs`: still handles many `zed://` links.
- `crates/release_channel/src/lib.rs`: partially renamed release channel display names.
- `crates/ai_onboarding`: still contains Zed AI plan and onboarding assumptions.
- `crates/language_models`: still contains provider and cloud integration assumptions that need product review.
- `.github/workflows`: still represents Zed's CI and release automation.
- `script/bundle-*`, `script/install-linux`, and installer resources: need Ideer release ownership.

## Recommended Release Order

1. Publish a clean source fork with honest README, attribution, and build instructions.
2. Complete identity cleanup for app name, CLI, protocol, URLs, icons, and visible UI.
3. Disable or replace Zed-owned cloud, billing, updater, telemetry, and support paths.
4. Ship one excellent AI MVP: agent plus subagent delegation with visible task state and permission safety.
5. Add swarm orchestration only after task graph, conflict handling, cancellation, persistence, and UI controls are reliable.
6. Prepare signed downloadable builds after the source release is stable.

## Definition Of Ready For First Public Release

- A new user can understand what Ideer is from the README in under one minute.
- The app does not send users to Zed-owned account, billing, or support flows by accident.
- The app name, icon, protocol, and installer metadata are consistent.
- The AI feature set is described honestly and works without hidden cloud assumptions.
- Agent actions are visible, interruptible, and permissioned.
- Build instructions work on at least one supported platform.
- License, attribution, and third-party notices are present.
