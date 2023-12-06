env = {
  // Infra:
  "IS_RUNNING_IN_NOMIC_FOUNDATION_SLANG_REPO": "true",
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Python:
  "LANG": "en_US.UTF-8",

  // Rust:
  "RUST_BACKTRACE": "FULL",
  "RUST_STABLE_VERSION": "1.72.0", // Keep this version in sync with "rust-version" in "$REPO_ROOT/Cargo.toml"
  "RUST_NIGHTLY_VERSION": "nightly-2023-12-01",
  "RUSTC_WRAPPER": "${HERMIT_ENV}/bin/sccache",
  "SCCACHE_DIR": "${HERMIT_ENV}/.hermit/sccache",

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
