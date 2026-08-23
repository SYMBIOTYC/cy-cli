# CY-CLI Developer Book

## Goal
- Make `cy` open the custom VolkovCommander-style 4-panel TUI by default.
- Show who is in the chat directly in the terminal title and TUI status bar.
- Keep fast subcommands: `cy q`, `cy m`, `cy ls`, `cy hist`, `cy b`, `cy tui`.
- Keep the app-server bridge (`cy-launch.sh` + `cy-adapter.mjs`) on startup.

## Repo layout
- Rust workspace root: `.fundament/codex-rs`
- Custom TUI: `.fundament/codex-rs/tui/src/ncview/mod.rs`
- CLI entry: `.fundament/codex-rs/cli/src/main.rs`
- Wrapper/launcher/bridge: `tools/cy-wrapper.sh`, `tools/cy-launch.sh`, `bin/cy-adapter.mjs`

## Key behavior changes
- `None` subcommand runs `codex_tui::ncview::run_ncview()` instead of the standard Codex TUI.
- `ncview` sets terminal title `CY | chat: <id> | Commander` and shows `chat: <id>` in the status bar.
- Wrapper prefers the local release binary, then falls back to repo build.

## Build
- Release: `cd .fundament/codex-rs && cargo build --bin cy --release`
- Debug: `cargo build --bin cy`

## Install
- Wrapper: `/Users/imac/.local/bin/cy` -> `tools/cy-wrapper.sh`
- Managed: `~/.local/share/cy/bin/cy`
