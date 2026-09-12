//! Automatic shell environment setup for first-run experience.
//!
//! Detects the user's current shell and, when appropriate, installs
//! Oh My Zsh plus the two most popular plugins (autosuggestions +
//! syntax-highlighting).  After a successful run a marker file is
//! written under `cx_home` so the work is performed only once.

use std::path::PathBuf;
use std::process::Command as StdCommand;

const SHELL_SETUP_MARKER: &str = ".shell_setup_done";
const ZSH_AUTOSUGGESTIONS_REPO: &str =
    "https://github.com/zsh-users/zsh-autosuggestions.git";
const ZSH_SYNTAX_HIGHLIGHTING_REPO: &str =
    "https://github.com/zsh-users/zsh-syntax-highlighting.git";
const OH_MY_ZSH_INSTALL_URL: &str =
    "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShellSetupOutcome {
    AlreadyDone,
    SkippedUnsupportedShell,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CurrentShell {
    Bash,
    Zsh,
    Fish,
    Unknown,
}

impl CurrentShell {
    pub(crate) fn detect() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_default();
        if shell.ends_with("zsh") || shell.ends_with("/zsh") {
            CurrentShell::Zsh
        } else if shell.ends_with("bash") || shell.ends_with("/bash") {
            CurrentShell::Bash
        } else if shell.ends_with("fish") || shell.ends_with("/fish") {
            CurrentShell::Fish
        } else {
            CurrentShell::Unknown
        }
    }
}

pub(crate) struct ShellSetup;

impl ShellSetup {
    pub(crate) fn marker_path(cx_home: &std::path::Path) -> PathBuf {
        cx_home.join(SHELL_SETUP_MARKER)
    }

    pub(crate) fn is_already_done(cx_home: &std::path::Path) -> bool {
        Self::marker_path(cx_home).exists()
    }

    pub(crate) fn needs_setup(cx_home: &std::path::Path) -> bool {
        if Self::is_already_done(cx_home) {
            return false;
        }
        match CurrentShell::detect() {
            CurrentShell::Bash | CurrentShell::Zsh => !Self::oh_my_zsh_installed(),
            CurrentShell::Fish | CurrentShell::Unknown => false,
        }
    }

    pub(crate) fn oh_my_zsh_installed() -> bool {
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return false,
        };
        let omz = home.join(".oh-my-zsh");
        omz.is_dir()
    }

    pub(crate) fn zsh_plugin_installed(name: &str) -> bool {
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return false,
        };
        let omz_custom = home.join(".oh-my-zsh").join("custom").join("plugins");
        omz_custom.join(name).is_dir()
    }

    pub(crate) fn run_setup(cx_home: &std::path::Path) -> ShellSetupOutcome {
        if Self::is_already_done(cx_home) {
            return ShellSetupOutcome::AlreadyDone;
        }

        let shell = CurrentShell::detect();
        if matches!(shell, CurrentShell::Fish | CurrentShell::Unknown) {
            return ShellSetupOutcome::SkippedUnsupportedShell;
        }

        if let Err(err) = Self::install_oh_my_zsh() {
            return ShellSetupOutcome::Failed(format!(
                "failed to install Oh My Zsh: {err}"
            ));
        }

        if let Err(err) = Self::install_plugin(ZSH_AUTOSUGGESTIONS_REPO, "zsh-autosuggestions") {
            return ShellSetupOutcome::Failed(format!(
                "failed to install zsh-autosuggestions: {err}"
            ));
        }

        if let Err(err) =
            Self::install_plugin(ZSH_SYNTAX_HIGHLIGHTING_REPO, "zsh-syntax-highlighting") {
            return ShellSetupOutcome::Failed(format!(
                "failed to install zsh-syntax-highlighting: {err}"
            ));
        }

        if let Err(err) = Self::enable_plugins_in_zshrc() {
            return ShellSetupOutcome::Failed(format!(
                "failed to update ~/.zshrc: {err}"
            ));
        }

        if let Err(err) = Self::set_zsh_as_default_shell() {
            tracing::warn!("failed to set zsh as default shell: {err}");
        }

        let marker = Self::marker_path(cx_home);
        if let Err(err) = std::fs::write(&marker, b"done") {
            tracing::warn!("failed to write shell setup marker: {err}");
        }

        ShellSetupOutcome::Completed
    }

    fn install_oh_my_zsh() -> anyhow::Result<()> {
        if Self::oh_my_zsh_installed() {
            return Ok(());
        }
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg(format!("curl -fsSL {} | sh", OH_MY_ZSH_INSTALL_URL))
            .output()
            .context("failed to spawn Oh My Zsh installer")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Oh My Zsh installer failed: {}", stderr);
        }
        Ok(())
    }

    fn install_plugin(repo: &str, name: &str) -> anyhow::Result<()> {
        if Self::zsh_plugin_installed(name) {
            return Ok(());
        }
        let home = dirs::home_dir().context("home directory not found")?;
        let dest = home
            .join(".oh-my-zsh")
            .join("custom")
            .join("plugins")
            .join(name);
        std::fs::create_dir_all(dest.parent().context("plugin parent dir")?)?;
        let output = StdCommand::new("git")
            .arg("clone")
            .arg("--depth=1")
            .arg(repo)
            .arg(&dest)
            .output()
            .context("failed to spawn git clone")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git clone failed: {}", stderr);
        }
        Ok(())
    }

    fn enable_plugins_in_zshrc() -> anyhow::Result<()> {
        let home = dirs::home_dir().context("home directory not found")?;
        let zshrc = home.join(".zshrc");
        if !zshrc.exists() {
            return Ok(());
        }
        let content = std::fs::read_to_string(&zshrc)?;
        let plugins_line = "plugins=(git zsh-autosuggestions zsh-syntax-highlighting)";
        if content.contains("plugins=(git") && !content.contains("zsh-autosuggestions") {
            let new_content = content.replace("plugins=(git)", plugins_line);
            std::fs::write(zshrc, new_content)?;
        } else if !content.contains("plugins=(git") && !content.contains("plugins=") {
            std::fs::write(zshrc, format!("{}\n{}", content, plugins_line))?;
        }
        Ok(())
    }

    fn set_zsh_as_default_shell() -> anyhow::Result<()> {
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg("command -v zsh")
            .output()
            .context("failed to locate zsh")?;
        if !output.status.success() {
            anyhow::bail!("zsh not found in PATH");
        }
        let zsh = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if zsh.is_empty() {
            anyhow::bail!("zsh not found in PATH");
        }
        let output = StdCommand::new("chsh")
            .arg("-s")
            .arg(&zsh)
            .output()
            .context("failed to spawn chsh")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("chsh failed: {}", stderr);
        }
        Ok(())
    }
}
