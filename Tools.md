# Tool.MD — CY-CLI Developer Commands Reference

> CY-CLI — SYMBIOTYC Cybernetic Intelligence CLI | Default provider: CY cyborg i1a via `https://api.cy.symbiotyc.workers.dev/v1`

---

## Quick Start

```bash
# Set the default model to CY cyborg i1a
cy model cy

# Check current model
cy m

# Run a one-shot prompt
echo "hello bro, whatsap" | cy exec

# Run interactively
cy
```

---

## Command Reference

### Core Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `cy [PROMPT]` | — | Start interactive session with optional prompt |
| `cy exec [PROMPT]` | `cy e` | Run non-interactively |
| `cy review` | — | Run a code review non-interactively |
| `cy apply` | `cy a` | Apply latest diff from agent as git apply |
| `cy resume` | — | Resume a previous session |
| `cy queue` | — | Queue a message for an existing session |
| `cy archive [ID]` | — | Archive a saved session |

### Model Management

| Command | Description |
|---------|-------------|
| `cy m` | Show current model |
| `cy m cy` | Set model to CY cyborg i1a |
| `cy model list` | List available models |
| `cy model set <model>` | Set default model |
| `cy -c model="cy"` | Override model for one run |

### Auth & Login

| Command | Description |
|---------|-------------|
| `cy login` | Start login flow |
| `cy logout` | Remove stored credentials |
| `cy doctor` | Diagnose local installation |
| `cy doctor network` | Check network/proxy settings |
| `cy doctor disk` | Check disk usage |
| `cy doctor updates` | Check for updates |

### Server & Tools

| Command | Description |
|---------|-------------|
| `cy agents` | Browse agent sessions on local app-server |
| `cy mcp` | Manage external MCP servers |
| `cy plugin` | Manage plugins |
| `cy skills` | Manage skills |
| `cy sandbox` | Run commands in a sandbox |

### System & Maintenance

| Command | Description |
|---------|-------------|
| `cy update` | Update to latest version |
| `cy completion` | Generate shell completion scripts |
| `cy debug` | Debugging tools |
| `cy app` | Launch the Desktop app |
| `cy install` | Install/update CY-CLI |

### Configuration

| Command | Description |
|---------|-------------|
| `cy -c key=value` | Override config value (e.g., `-c model="cy"`) |
| `cy --enable FEATURE` | Enable a feature flag |
| `cy --disable FEATURE` | Disable a feature flag |
| `~/.CY/config.toml` | Main config file (model, provider, sandbox settings) |
| `~/.CY/vibe-catalog.json` | Available models catalog |

---

## Configuration File: `~/.CY/config.toml`

```toml
model = "cy"
model_provider = "simbiotyc"

[model_providers.cy]
name = "CY"
base_url = "https://api.cy.symbiotyc.workers.dev/v1"
experimental_bearer_token = "cfat_KbYOsjGncELIzKQn3WxUIz9jL97n9nJK2I1EG4hg35627bee"
wire_api = "responses"
```

---

## Model Catalog: `~/.CY/vibe-catalog.json`

The catalog contains the available models. Currently only **cy** (CY cyborg i1a) is active:

```json
{
  "models": [
    {
      "slug": "cy",
      "display_name": "CY cyborg i1a",
      "supported_reasoning_levels": [],
      "shell_type": "shell_command",
      "visibility": "list",
      "supported_in_api": true,
      "priority": 0,
      "model_messages": {
        "instructions_template": "You are CY i1a, a helpful AI assistant.",
        "instructions_variables": {
          "personality_default": "",
          "personality_friendly": "",
          "personality_pragmatic": ""
        }
      }
    }
  ]
}
```

---

## CY URLs

| Service | URL |
|---------|-----|
| CY API (main) | `https://api.cy.symbiotyc.workers.dev/v1` |
| CY Auth | `https://auth.cy.symbiotyc.workers.dev` |
| CY Developer Docs | `https://developers.cy.symbiotyc.workers.dev` |
| CY API Gateway | `https://api.cy.symbiotyc.workers.dev` |
| CY Platform (tunnel) | `https://platform.symbiotyc.workers.dev` |

---

## Repository Structure

```
cy-cli/
├── .fundament/cx-rs/          # Rust source code
│   ├── cli/                    # CLI binary (cy)
│   ├── core/                   # Core engine
│   ├── tui/                    # Terminal UI
│   ├── config/                 # Configuration loading
│   ├── model-provider-info/    # Model provider info
│   ├── models-manager/         # Models manager (models.json)
│   ├── cx-*/                   # Individual crates
│   └── Cargo.toml              # Workspace manifest
├── scripts/                    # Install scripts
│   ├── install.sh              # Bash installer
│   └── install.ps1             # PowerShell installer
├── CY-CLI-releases/            # Release artifacts
│   ├── cy                      # Linux binary
│   ├── cy-x86_64-unknown-linux-gnu.tar.gz
│   └── *.app/                  # macOS app bundle
```

---

## Installation Commands

```bash
# From source (rebuild)
cd ~/.fundament/cx-rs
cargo build --release -p cy-cli
cp target/release/cy /Applications/CY-CLI.app/Contents/MacOS/cy

# Via install script
./scripts/install.sh

# Via releases repo
curl -L https://github.com/SYMBIOTYC/CY-CLI-releases/releases/latest/download/cy-x86_64-unknown-linux-gnu.tar.gz | tar xz
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `OI_FEDERATION_RULE_ID_ENV_VAR` | Federated rule ID for identity |
| `OI_IDENTITY_TOKEN_FILE_ENV_VAR` | Identity token file path |
| `OI_WORKLOAD_IDENTITY_CONTEXT_ENV_VAR` | Workload identity context |
| `CY_API_KEY` | CY API key |
| `CY_BASE_URL` | Override CY gateway URL |

---

## Common Workflows

### Interactive Session
```bash
cy
# Type your prompt at the interactive prompt
# Model: cy (CY cyborg i1a)
```

### One-Shot Command
```bash
echo "hello bro, whatsap" | cy exec
# or
cy exec "hello bro, whatsap"
```

### Code Review
```bash
cy review
# Reviews the current working tree changes
```

### Apply Changes
```bash
cy apply
# Applies the latest diff as a git apply
```

### Check Installation Health
```bash
cy doctor
cy doctor network
cy doctor disk
cy doctor updates
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `stdin is not a terminal` | Use `cy exec` instead of piping |
| `Not inside a trusted directory` | Use `cy exec --skip-git-repo-check` |
| `failed to parse model_catalog_json` | Check `~/.CY/vibe-catalog.json` format |
| `401 unauthorized` | Run `cy login` to re-authenticate |
| Model stuck on wrong value | Run `cy m cy` to reset |