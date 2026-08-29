use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn cx_command(cx_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(cx_utils_cargo_bin::cargo_bin("cx")?);
    cmd.env("CODEX_HOME", cx_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let cx_home = TempDir::new()?;

    cx_command(cx_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`cx update` is not available in debug builds"));

    Ok(())
}
