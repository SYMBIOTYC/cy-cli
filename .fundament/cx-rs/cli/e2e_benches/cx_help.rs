#![allow(clippy::expect_used)]

use std::process::Command;

use divan::Bencher;

fn main() {
    divan::main();
}

/// Exercises the Bazel-backed end-to-end benchmark path with a cheap,
/// deterministic CX invocation. Richer scenarios can add separate
/// benchmark binaries without making the shared harness depend on them.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn cx_help(bencher: Bencher) {
    let cx = cx_utils_cargo_bin::cargo_bin("cx")
        .expect("cx binary should be available through Bazel runfiles");

    bencher.bench_local(move || {
        let output = Command::new(&cx)
            .arg("--help")
            .output()
            .expect("cx --help should run");
        assert!(output.status.success(), "cx --help should succeed");
    });
}
