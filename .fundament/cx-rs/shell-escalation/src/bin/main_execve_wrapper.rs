#[cfg(not(unix))]
fn main() {
    eprintln!("cx-execve-wrapper is only implemented for UNIX");
    std::process::exit(1);
}

#[cfg(unix)]
pub use cx_shell_escalation::main_execve_wrapper as main;
