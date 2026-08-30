#!/usr/bin/env bash
set -euo pipefail

CY_INSTALL_DIR="${CY_INSTALL_DIR:-${HOME}/.local/share/cy}"
CY_BIN_DIR="${CY_INSTALL_DIR}/bin"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# If invoked as CY, bypass the bridge and launch the release binary directly.
if [ "$(basename "$0")" = "CY" ]; then
  if [ -x "${REPO_ROOT}/.fundament/cx-rs/target/release/cy" ]; then
    exec "${REPO_ROOT}/.fundament/cx-rs/target/release/cy" "$@"
  fi
fi

mkdir -p "${CY_BIN_DIR}"

find_cy() {
  if [ -x "${CY_BIN_DIR}/cy" ]; then
    echo "${CY_BIN_DIR}/cy"
    return 0
  fi
  if [ -x "${REPO_ROOT}/.fundament/cx-rs/target/release/cy" ]; then
    echo "${REPO_ROOT}/.fundament/cx-rs/target/release/cy"
    return 0
  fi
  return 1
}

main() {
  local cy_binary
  cy_binary=$(find_cy) || {
    echo "[cy-wrapper] No cy binary found. Build or install CY first." >&2
    exit 1
  }
  exec "$cy_binary" "$@"
}

main "$@"
