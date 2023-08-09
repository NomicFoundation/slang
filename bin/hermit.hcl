env = {
  // Infra:
  "IS_RUNNING_IN_NOMIC_FOUNDATION_SLANG_REPO": "true",
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Python:
  "LANG": "en_US.UTF-8",

  // Rust:
  "RUST_BACKTRACE": "FULL",
  "RUST_VERSION": "1.69.0", // Keep this version in sync with "rust-version" in "$REPO_ROOT/Cargo.toml" and "rust-src" in "$REPO_ROOT/bin"

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
