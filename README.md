# CY-CLI

Более продвинутый CLI на фундаменте открытого Codex CLI (openai/codex).

- **Фундамент:** `.fundament/` — скопированная копия исходников Codex CLI (shallow clone).
- **Рассуждения/история:** см. `CY-CLI-DevBook.md`.
- Цель: добавить недостающие "очень быстрые" методы поверх нативного harness'а Codex и перевести расширение Codex (openai.chatgpt) на использование CY-CLI.

## Quick Start

```bash
# Install (macOS / Linux)
curl -fsSL https://raw.githubusercontent.com/SYMBIOTYC/cy-cli/main/scripts/install.sh | bash

# Install (Windows PowerShell)
irm https://raw.githubusercontent.com/SYMBIOTYC/cy-cli/main/scripts/install.ps1 | iex

# Or download binary from GitHub Releases
# https://github.com/SYMBIOTYC/cy-cli/releases
```

## Building from source

```bash
cd .fundament/codex-rs
cargo build --bin cy --release
```

## Subcommands

| Command | Alias | Description |
|---------|-------|-------------|
| `cy q <prompt>` | `cy quick` | Quick question, streaming response |
| `cy m [model]` | `cy model` | Show/set current model |
| `cy ls` | `cy list-models` | List available models |
| `cy hist [query]` | `cy history` | Session history with search |
| `cy b <instr> [files]` | `cy batch` | Batch process files |
| `cy tui` | `cy nc` | 4-panel PIE Commander TUI |

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
