# Ideer

Ideer is an AI-first code editor. It is a fork of [Zed](https://github.com/zed-industries/zed) with extra focus on agent and subagent workflows.

This repository is pre-release. There are no signed downloadable builds yet. To try Ideer today you build it from source.

## Status

- Source-only. No installer downloads, no auto-update server, no Ideer cloud, no Ideer account system.
- The editor core, language support, project, terminal, git, debugger, and panel layout are inherited from upstream Zed and continue to track upstream.
- The agent and subagent surfaces are where Ideer is being shaped. The product gaps are tracked in [`PRODUCT_GAPS.md`](./PRODUCT_GAPS.md).
- Telemetry, billing, plan upsells, and Zed-cloud-only features are disabled in the UI when no real Ideer endpoint exists.

## How Ideer differs from Zed

- Branding and product identity are Ideer's. App display name, menus, and onboarding copy say "Ideer".
- The deep-link URL scheme `ideer://` is registered alongside `zed://` for backwards compatibility.
- Zed-cloud paid features (trials, plan upsells, account management against `zed.dev`) are hidden until an Ideer-owned equivalent exists.
- AI provider configuration is BYOK. Bring your own API key for OpenAI, Anthropic, Google, OpenRouter, Ollama, LM Studio, or any other supported provider. Ideer does not ship a hosted model service.
- Auto-update is disabled by default. Ideer does not have its own release/update server yet. Updates happen by rebuilding from this repository.

## Building from source

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

The build commands and toolchain are identical to upstream Zed. Most of those docs are still Zed-branded; treat the `zed` binary name and the `zed` crate name in build instructions as the same thing as Ideer for now.

## Configuring AI providers

Ideer reads provider credentials from settings. Open the settings file and add API keys for the providers you want to use. There is no required account, no required cloud service, and no required network connection beyond the provider you choose.

If no provider is configured, the agent panel will tell you so and link to the relevant settings.

## Experimental areas

- Agent / subagent orchestration is being actively reshaped. Expect rough edges.
- The first-run layout choice between a VS Code-familiar shell and a Zed-native shell is in design. See [`UI_LAYOUT_PRESETS.md`](./UI_LAYOUT_PRESETS.md).
- Telemetry, sharing, and collaboration features inherited from Zed are not guaranteed to work end-to-end in Ideer.

## License and attribution

Ideer is a fork of Zed. Upstream Zed is developed by Zed Industries, Inc. and is available at https://github.com/zed-industries/zed.

This repository preserves the upstream license files:

- [LICENSE-AGPL](./LICENSE-AGPL)
- [LICENSE-APACHE](./LICENSE-APACHE)
- [LICENSE-GPL](./LICENSE-GPL)

When you redistribute binaries built from this repository, you must comply with the same license obligations as upstream Zed. Third-party license generation continues to use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about); see [`script/licenses/zed-licenses.toml`](./script/licenses/zed-licenses.toml) for the accepted-license configuration.

If you are looking for the upstream Zed editor product, downloadable builds, support, billing, or the Zed community, please go to [zed.dev](https://zed.dev) and [github.com/zed-industries/zed](https://github.com/zed-industries/zed). Ideer is a separate fork and is not affiliated with Zed Industries, Inc.
