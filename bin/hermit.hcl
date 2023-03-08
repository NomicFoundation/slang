env = {
  // Used by build scripts to locate source files:
  "REPO_ROOT": "${HERMIT_ENV}",

  // Adds local script shortcuts to user PATH:
  "PATH": "${HERMIT_ENV}/scripts/cargo/run:${PATH}",

  // Used by any invocation of `cargo` within the repository:
  "RUST_BACKTRACE": "FULL",
  "RUSTFLAGS": "--warn unused_crate_dependencies",
}
