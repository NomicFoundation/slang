use std::process::Command;

/// Workaround for `cargo-nextest` older than v0.9.72:
///
/// `nextest` (unlike `cargo test`) does not add the toolchain `libdir` to the dynamic
/// linker search path when launching test binaries. Proc-macro crates link `libstd`
/// dynamically (`-C prefer-dynamic`), so this crate's unit-test harness fails to start
/// under `nextest` with:
///
/// ```log
/// error while loading shared libraries: libstd-*.so: cannot open shared object file
/// ```
///
/// `nextest` v0.9.72 fixes this natively by detecting the host/target libdirs itself.
/// For now, we bake the `libdir` into the test binary's RUNPATH instead.
///
/// TODO: Remove this build script once `nextest` is upgraded to `0.9.72` or later.
///
/// `__NEXTEST_RPATH_WORKAROUND__` (keep in sync with other proc-macro crates in the workspace)
fn main() {
    let rustc_binary = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());

    let output = Command::new(rustc_binary)
        .args(["--print", "target-libdir"])
        .output()
        .expect("failed to invoke `rustc --print target-libdir`");

    assert!(output.status.success());

    let libdir = String::from_utf8(output.stdout).unwrap();

    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", libdir.trim());
}
