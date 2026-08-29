#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
minimum_supported_release="$(
  sed -n 's/^pub const MINIMUM_SUPPORTED_CODEX_VERSION: &str = "\([^"]*\)";$/\1/p' \
    "${repo_root}/cx-rs/exec-server-protocol/src/lib.rs"
)"
: "${minimum_supported_release:?minimum supported CX release is missing}"
release_directory="$(mktemp -d "${TMPDIR:-/tmp}/cx-exec-server-skew.XXXXXX")"
trap 'rm -rf "${release_directory:?}"' EXIT

if [[ $# -eq 0 ]]; then
  releases=(latest "${minimum_supported_release}")
else
  releases=("$@")
fi

case "$(uname -s):$(uname -m)" in
  Darwin:arm64) target="aarch64-apple-darwin" ;;
  Darwin:x86_64) target="x86_64-apple-darwin" ;;
  Linux:aarch64 | Linux:arm64) target="aarch64-unknown-linux-musl" ;;
  Linux:x86_64) target="x86_64-unknown-linux-musl" ;;
  *)
    echo "Unsupported platform: $(uname -s) $(uname -m)" >&2
    exit 1
    ;;
esac

asset="cx-${target}.tar.gz"
cd "${repo_root}/cx-rs"
cargo build -p cx-cli --bin cx
export CODEX_TEST_CURRENT_CODEX="${CARGO_TARGET_DIR:-${repo_root}/cx-rs/target}/debug/cx"

echo "Testing current CX compatibility through authenticated Noise"
export CODEX_TEST_RELEASED_CODEX="${CODEX_TEST_CURRENT_CODEX}"
just test -p cx-exec-server --test relay version_skew --test-threads 1

tested_release_version=""
for release in "${releases[@]}"; do
  release="${release#rust-v}"
  if [[ "${release}" == "${tested_release_version}" ]]; then
    echo "Skipping CX ${release}; this release was already tested"
    continue
  fi

  if [[ "${release}" == "latest" ]]; then
    release_url="https://github.com/openai/cx/releases/latest/download/${asset}"
  else
    release_url="https://github.com/openai/cx/releases/download/rust-v${release}/${asset}"
  fi

  binary_directory="${release_directory}/${release}"
  mkdir -p "${binary_directory}"
  echo "Downloading released CX from ${release_url}"
  curl -fsSL "${release_url}" -o "${binary_directory}/${asset}"
  tar -xzf "${binary_directory}/${asset}" -C "${binary_directory}"

  export CODEX_TEST_RELEASED_CODEX="${binary_directory}/cx-${target}"
  release_output="$("${CODEX_TEST_RELEASED_CODEX}" --version)"
  echo "${release_output}"
  tested_release_version="${release_output##* }"

  just test -p cx-exec-server --test relay version_skew --test-threads 1
done
