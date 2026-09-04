# CY-CLI v0.3.2 "GOD_MODE" — Full System Access & Browser Automation

## TL;DR
CY is now a full-access coding agent with built-in browser automation, chaotic LED status, hex morph branding, and zero approval prompts. Full access is on by default — users can dial it back anytime.

## What's New

### GOD_MODE (default)
- `approval_policy = "Never"` — CY never asks for permission
- `default_permissions = "danger-full-access"` — unrestricted filesystem + network access
- Users can override in `~/.cy/config.toml` or via the `/permissions` command anytime

### Browser Tools (built into the bridge)
- **`browser_fetch(url)`** — fetch any URL via curl, get clean text (HTML stripped automatically)
- **`browser_open(url)`** — open URLs in the system browser (new tab in Chrome/Safari)
- **`browser_screenshot()`** — capture screen regions via macOS screencapture
- CY no longer has an excuse for "no internet" — it CAN and DOES fetch URLs natively

### LED Indicator (bottom-left)
- Replaced the old "Working" shimmer wave with a **chaotic router-style LED blink**
- **Green** = working (fast, random flicker like a router LED)
- **Red** = error
- **Cyan** = complete/notification
- 30fps self-scheduling animation

### Hex Morph (top-right)
- SYMBIOTYC logo: square morphing into hexagon and back, continuously
- Pink on black, smooth easing curve, 4-second cycle
- Always visible so users can see CY is alive

### Popup VFX
- **Wink** (white flash) when popups appear
- **Poof** (pink particle burst) when popups disappear

## Quick Start
```bash
cy exec "fetch https://example.com and summarize"
cy exec "what files are in /Users/imac/Documents?"
```

## Install / Upgrade
Download DMG from GitHub releases → Drag to Applications → Launch from Terminal.

## Config (default — full GOD_MODE)
```toml
# ~/.cy/config.toml — generated automatically
model = "cy/i1a"
model_provider = "symbiotyc"
model_reasoning_effort = "none"
approval_policy = "Never"
default_permissions = "danger-full-access"
```

---
*CY — Symbiotic Coding Assistant*
*GitHub: @SYMBIOTYC/cy-cli*
*Server: cy.symbiotyc.workers.dev*