# CY-CLI Developer Book

## Version
- Current: `0.3.3` (commit `bfe0458`, tag `v0.3.3`)
- Repo: `SYMBIOTYC/cy-cli` at `/Volumes/Work/CY/structured/cy-cli`
- Release repo: `SYMBIOTYC/CY_CLI` at `/Volumes/Work/CY/structured/CY-CLI-releases`

## Status
- Commit pushed: `bfe0458` on `main`
- Tag created: `v0.3.3`
- CI/CD triggered:
  - macOS: https://github.com/SYMBIOTYC/CY_CLI/actions/runs/34015276140
  - Linux: https://github.com/SYMBIOTYC/CY_CLI/actions/runs/34015389472
  - Windows: https://github.com/SYMBIOTYC/CY_CLI/actions/runs/34015389429

## Config / paths
- Config: `~/.cy/config.toml`, `~/.cy/auth.json`
- API key: `~/.cy/auth.json` (`openai_api_key` field) or `CY_API_KEY` env
- CY server: `https://cy.symbiotyc.workers.dev/v1`
- Local bridge: `http://127.0.0.1:8790/v1`
- Rust workspace: `.fundament/cx-rs`
- Binary (release): `.fundament/cx-rs/target/release/cy`
- Binary (debug): `.fundament/cx-rs/target/debug/cy`
- macOS app: `/Applications/CY-CLI-intel.app`
- Launcher script: `packaging/macos/launcher`
- Bridge script: `packaging/macos/cy_bridge.py`

## Architecture
- Rust CLI (`cy`) talks to local Python bridge (`cy_bridge.py`) on `127.0.0.1:8790`
- Bridge converts Responses API requests → Chat Completions for upstream CY server
- Bridge executes local tools: `read_file`, `write_file`, `list_dir`, `shell_exec`, `glob_files`, `browser_open`, `browser_fetch`, `browser_screenshot`
- System prompt injected by bridge enforces "GOD MODE" behavior

## Browser tools
- Rust handlers registered in `cx-core`: `browser_open`, `browser_fetch`, `browser_screenshot`
- Handlers implemented in `core/src/tools/handlers/browser_*.rs`
- Registered via `add_browser_tools()` in `core/src/tools/spec_plan.rs`
- Rust handlers use flat tool names (`browser_open`, `browser_fetch`, `browser_screenshot`) matching bridge system prompt
- **Current execution path:** Python bridge (`cy_bridge.py`) handles browser tools locally
- Rust handlers are registered but not yet invoked in the bridge-forwarding path
- Bridge tool loop limit: `max_tool_rounds = 20`
- Bridge stability improvements:
  - Request timeout: 300s
  - Conversation history limit: 50 messages
  - Per-round tool call logging with truncated arguments
  - Explicit instruction to synthesize after 1-3 tool calls
  - Better exception handling with graceful error messages
- Bridge tested successfully with tehnoskarb.ua multi-tool task

## Build
- Release: `cd .fundament/cx-rs && cargo build --release -p cy-cli --target x86_64-apple-darwin`
- Debug: `cargo build -p cy-cli --target x86_64-apple-darwin`
- Check: `cargo check -p cx-core`
- Format: `cargo fmt -- core/src/tools/handlers/browser_*.rs`

## Install (macOS)
- Copy debug/release binary into app bundle: `/Applications/CY-CLI-intel.app/Contents/MacOS/cy`
- Ensure `Info.plist` exists in `/Applications/CY-CLI-intel.app/Contents/`
- Launcher starts bridge, writes `~/.cy/config.toml`, opens Terminal with splash screen
- Bridge auto-starts on port `8790` if not already running

## CI/CD
- Source repo: `SYMBIOTYC/cy-cli` → tag push triggers `release.yml` in source repo
- Release repo: `SYMBIOTYC/CY_CLI` → `repository_dispatch` triggers platform builds
  - macOS DMG: `build-macos.yml` (x86_64 + arm64)
  - Linux tarball: `build-linux.yml`
  - Windows ZIP: `build-windows.yml`
- Release assets are uploaded to GitHub Release in `SYMBIOTYC/CY_CLI`

## Config template (launcher-generated)
```toml
model = "cy/i1a"
model_provider = "symbiotyc"
model_context_window = 128000
model_auto_compact_token_limit = 96000
model_reasoning_summary = "auto"
model_reasoning_effort = "none"
approval_policy = "never"

[model_providers.symbiotyc]
name = "SYMBIOTYC"
base_url = "http://127.0.0.1:8790/v1"
wire_api = "responses"
supports_websockets = false
models = ["cy/i1a"]
```

## Key files
- `core/src/tools/handlers/browser_open.rs`
- `core/src/tools/handlers/browser_fetch.rs`
- `core/src/tools/handlers/browser_screenshot.rs`
- `core/src/tools/spec_plan.rs` — `add_browser_tools()` registration
- `packaging/macos/launcher` — app launcher script
- `packaging/macos/cy_bridge.py` — local bridge with tool execution and stability fixes

## Next steps
- Wait for CI/CD builds to complete
- Verify release artifacts in `SYMBIOTYC/CY_CLI`
- Install new version locally and test
- If stable, migrate tool execution from Python to Rust handlers

## Notes
- Do NOT use `default_permissions = "danger-full-access"` in config
- Use lowercase `approval_policy = "never"`
- Bridge log: `/private/tmp/cy_bridge.log`
- CY home: `~/.cy/`
