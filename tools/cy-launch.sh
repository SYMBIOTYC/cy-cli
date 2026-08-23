#!/bin/bash
# CY launcher with embedded bridge
set -u
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CY_HOME="${CY_HOME:-$HOME/.cy}"
mkdir -p "$CY_HOME" 2>/dev/null
export CODEX_HOME="$CY_HOME"
export CY_API_BASE_URL="${CY_API_BASE_URL:-https://cy.symbiotyc.workers.dev/v1}"
export CY_API_KEY="${CY_API_KEY:-cfat_KbYOsjGncELIzKQn3WxUIz9jL97n9nJK2I1EG4hg35627bee}"
export CY_MODEL="${CY_MODEL:-cy/i1a}"
export CY_BRIDGE_PORT="${CY_BRIDGE_PORT:-8790}"
BRIDGE="$REPO_ROOT/bin/cy-adapter.mjs"
BACKEND="$REPO_ROOT/.fundament/codex-rs/target/release/cy"
NODE_BIN="$(command -v node || true)"
CONFIG="$CY_HOME/config.toml"
if [ ! -f "$CONFIG" ] || ! grep -q "cy-symbiotyc-bridge-v2" "$CONFIG" 2>/dev/null; then
  cat >"$CONFIG" <<EOF
# CY — generated automatically. SYMBIOTYC provider is pinned; do not edit.
# marker: cy-symbiotyc-bridge-v2
model = "$CY_MODEL"
model_provider = "symbiotyc"
model_context_window = 128000
model_auto_compact_token_limit = 96000
model_reasoning_summary = "auto"
[model_providers.symbiotyc]
name = "SYMBIOTYC"
base_url = "http://127.0.0.1:$CY_BRIDGE_PORT/v1"
wire_api = "responses"
supports_websockets = false
experimental_bearer_token = "cy-local-bridge"
models = ["cy/i1a"]
EOF
fi
if [ ! -f "$CY_HOME/auth.json" ]; then
  printf '{"auth_mode":"apikey","OPENAI_API_KEY":"cy-local-bridge"}
' >"$CY_HOME/auth.json"
  chmod 600 "$CY_HOME/auth.json" 2>/dev/null
fi
export OPENAI_API_KEY="${OPENAI_API_KEY:-cy-local-bridge}"
if ! bash -c "exec 3<>/dev/tcp/127.0.0.1/$CY_BRIDGE_PORT" 2>/dev/null; then
  if [ -z "$NODE_BIN" ]; then
    echo "CY: Node.js не найден — установите Node.js." >&2
    exit 1
  fi
  if [ ! -f "$BRIDGE" ]; then
    echo "CY: мост не найден." >&2
    exit 1
  fi
  nohup "$NODE_BIN" "$BRIDGE" >"$CY_HOME/bridge.log" 2>&1 &
  disown 2>/dev/null
  for _ in $(seq 1 50); do
    if bash -c "exec 3<>/dev/tcp/127.0.0.1/$CY_BRIDGE_PORT" 2>/dev/null; then break; fi
    sleep 0.1
  done
fi
if [ ! -x "$BACKEND" ]; then
  echo "CY: ядро не найдено." >&2
  exit 1
fi
exec "$BACKEND" "$@"
