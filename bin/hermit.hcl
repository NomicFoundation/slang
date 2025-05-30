env = {
  // Infra:
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Rust:
  "RUST_BACKTRACE": "full",
  "RUST_STABLE_VERSION": "1.86.0", // __RUST_STABLE_VERSION_MARKER__ (keep in sync)
  "RUST_NIGHTLY_VERSION": "nightly-2025-04-03", // __RUST_NIGHTLY_VERSION_MARKER__ (keep in sync)

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
