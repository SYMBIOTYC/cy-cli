# CY-CLI Developer Book

## Version
- Current: `0.3.2`
- Repo: `SYMBIOTYC/cy-cli` at `/Volumes/Work/CY/structured/cy-cli`
- Release repo: `SYMBIOTYC/CY_CLI` at `/Volumes/Work/CY/structured/CY-CLI-releases`

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
- Rust handlers now use flat tool names (`browser_open`, `browser_fetch`, `browser_screenshot`) matching the bridge's system prompt
- **Current execution path:** Python bridge (`cy_bridge.py`) handles browser tools locally
- Rust handlers are registered but not yet invoked in the bridge-forwarding path
- Bridge tool loop limit: `max_tool_rounds = 12`
- Bridge stability improvements:
  - Request timeout: 300s
  - Conversation history limit: 50 messages
  - Per-round logging for tool execution
  - Better exception handling with graceful error messages

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
- Test improved bridge with complex multi-tool task (e.g., OLX search)
- If stable, migrate tool execution from Python to Rust handlers (requires bridge protocol changes)
- Package and release updated binary

## Notes
- Do NOT use `default_permissions = "danger-full-access"` in config
- Use lowercase `approval_policy = "never"`
- Bridge log: `/private/tmp/cy_bridge.log`
- CY home: `~/.cy/`
