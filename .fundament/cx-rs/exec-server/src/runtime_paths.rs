use std::path::PathBuf;

use cx_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the CX executable used to launch hidden helper modes.
    pub cx_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter CX by argv0.
    pub cx_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        cx_self_exe: Option<PathBuf>,
        cx_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let cx_self_exe = cx_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "CX executable path is not configured",
            )
        })?;
        Self::new(cx_self_exe, cx_linux_sandbox_exe)
    }

    pub fn new(
        cx_self_exe: PathBuf,
        cx_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            cx_self_exe: absolute_path(cx_self_exe)?,
            cx_linux_sandbox_exe: cx_linux_sandbox_exe.map(absolute_path).transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
