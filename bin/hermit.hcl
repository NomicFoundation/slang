env = {
  // Infra:
  // _SLANG_INFRA_SOLANG_LLVM_PATH_ (keep in sync)
  "PATH": "${HERMIT_ENV}/scripts/bin:${HERMIT_ENV}/bin/solang-llvm/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Rust:
  "RUST_BACKTRACE": "full",
  "RUST_STABLE_VERSION": "1.87.0", // __RUST_STABLE_VERSION_MARKER__ (keep in sync)
  "RUST_NIGHTLY_VERSION": "nightly-2025-06-08", // __RUST_NIGHTLY_VERSION_MARKER__ (keep in sync)

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
