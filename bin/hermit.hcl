env = {
  // Infra:
  "IS_RUNNING_IN_NOMIC_FOUNDATION_SLANG_REPO": "true",
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Python:
  "LANG": "en_US.UTF-8",

  // Rust:
  "RUST_BACKTRACE": "full",
  "RUST_STABLE_VERSION": "1.72.0", // Keep in sync with other "RUST_STABLE_VERSION" references
  "RUST_NIGHTLY_VERSION": "nightly-2023-12-01", // Keep in sync with other "RUST_NIGHTLY_VERSION" references
  "RUSTC_WRAPPER": "${HERMIT_ENV}/bin/sccache",
  "SCCACHE_DIR": "${HERMIT_ENV}/.hermit/sccache",

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
