env = {
  // Infra:
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Rust:
  "RUST_BACKTRACE": "full",
  "RUST_STABLE_VERSION": "1.79.0", // __RUST_STABLE_VERSION_MARKER__ (keep in sync)
  "RUST_NIGHTLY_VERSION": "nightly-2024-06-17", // __RUST_NIGHTLY_VERSION_MARKER__ (keep in sync)
  "RUSTC_WRAPPER": "${HERMIT_ENV}/bin/sccache",
  "SCCACHE_DIR": "${HERMIT_ENV}/.hermit/sccache",

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
