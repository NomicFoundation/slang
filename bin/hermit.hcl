env = {
  // Used by build scripts to locate source files:
  "REPO_ROOT": "${HERMIT_ENV}",

  // Used by any invocation of `cargo` within the repository:
  "RUST_BACKTRACE": "FULL",
  "RUSTFLAGS": "--warn unused_crate_dependencies",
}
