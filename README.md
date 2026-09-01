# CY-CLI

## Config location
- **NEW:** `~/.cy/config.toml` and `~/.cy/auth.json`


## API endpoint
- **NEW:** `https://api.cy.symbiotyc.workers.dev/v1`


## Binary path
- **NEW:** `.fundament/cx-rs/target/release/cy`


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
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

Artifacts are published to [GitHub Releases](https://github.com/SYMBIOTYC/cy-cli/releases).

## License

Apache-2.0
# TEMP CI TRIGGER
