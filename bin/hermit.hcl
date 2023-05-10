env = {
  // Used by build scripts to locate source files:
  "REPO_ROOT": "${HERMIT_ENV}",

  // Python:
  "LANG": "en_US.UTF-8",

  // Rust:
  "RUST_BACKTRACE": "FULL",
  "RUST_VERSION": "1.65.0", // Keep this version in sync with "rust-version" in "$REPO_ROOT/Cargo.toml"

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
