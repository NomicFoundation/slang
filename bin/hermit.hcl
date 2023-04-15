env = {
  // Used by build scripts to locate source files:
  "REPO_ROOT": "${HERMIT_ENV}",

  // Python:
  "LANG": "en_US.UTF-8",

  // Rust:
  "RUST_BACKTRACE": "FULL",
  "RUST_VERSION": "1.64.0",
  "RUSTFLAGS": "${RUSTFLAGS} --warn unused_crate_dependencies",

  // TypeScript:
  "TS_NODE_PROJECT": "${REPO_ROOT}/tsconfig.json",
}
