# CY-CLI

A model-agnostic coding-agent CLI. Forked from the Rust
implementation of OpenAI Codex (see `NOTICE` for upstream attribution
and `LICENSE` for the Apache-2.0 terms).

## v0.2.7 highlights

- All upstream "Codex" / "oi CX" branding in user-visible strings has
  been replaced with "CY" / "cy".
- The macOS .app bundle ships with a local Python bridge
  (`cy_bridge.py`) that translates the Responses API to the Chat
  Completions API of the configured upstream, with a developer
  prompt override and a local tool-execution loop.
- Bridge v2.1 adds: parallel tool execution (ThreadPoolExecutor,
  max 4 workers), upstream retry with exponential backoff, per-request
  token-usage logging, and a workspace sandbox for shell/file tools.
- CI builds on every push; post-build verification grep-asserts that
  no `oi CX`, `0.0.0`, or `chatgpt.com/cx?app-landing-page=true`
  leaked into the binary or the bundled bridge script.

## Config location
- `~/.cy/config.toml` and `~/.cy/auth.json`


## API endpoint
- `https://cy.symbiotyc.workers.dev/v1` (Chat Completions)


## Binary path
- `.fundament/cx-rs/target/release/cy`


## Install

```bash
# Install (macOS / Linux)
curl -fsSL https://raw.githubusercontent.com/SYMBIOTYC/CY-CLI-releases/main/install-v2.sh | bash

# Install (Windows PowerShell)
irm https://raw.githubusercontent.com/SYMBIOTYC/CY-CLI-releases/main/install-v2.ps1 | iex

# Or download binary from GitHub Releases
# https://github.com/SYMBIOTYC/CY-CLI-releases/releases
```

## Building from source

```bash
cd .fundament/cx-rs
cargo build --bin cy --release
```

## CI/CD

Releases are built automatically via GitHub Actions for:
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Builds are also produced for `x86_64-unknown-linux-gnu`,
`aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc` via the
upstream codex-rs Bazel pipeline (see `.fundament/justfile`).

Each release ships a SHA-256 sidecar next to every DMG and binary.

## Tools exposed to the model

The macOS bridge exposes five local tools to the model:

| name          | purpose                                                |
|---------------|--------------------------------------------------------|
| `read_file`   | Read a UTF-8 file (capped at 65 KiB).                  |
| `write_file`  | Write UTF-8 text to a file, creating parent dirs.      |
| `list_dir`    | List a directory's entries as JSON.                    |
| `shell_exec`  | Run a shell command (default cwd, timeout 30s).        |
| `glob_files`  | Expand a glob pattern (max 1000 matches).              |

By default these are sandboxed to the bridge's current working
directory. `CY_BRIDGE_ROOT=/path` lifts the sandbox for that whole
tree (intended for trusted batch jobs, not interactive use).

See `packaging/macos/cy_bridge.README.md` in
`SYMBIOTYC/CY-CLI-releases` for the full reference.

## License

Apache-2.0. See `LICENSE` (in `.fundament/cx-rs/`) and `NOTICE` (in
the repo root) for the full text and the upstream attribution.

