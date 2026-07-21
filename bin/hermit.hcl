sources = ["https://github.com/NomicFoundation/cargo-cooldown-check.git", "https://github.com/cashapp/hermit-packages.git"]

env = {
  // Infra:
  "PATH": "${HERMIT_ENV}/scripts/bin:${PATH}",
  "REPO_ROOT": "${HERMIT_ENV}",

  // Rust:
  "RUST_BACKTRACE": "full",
  "RUST_STABLE_VERSION": "1.97.1", // __RUST_STABLE_VERSION_MARKER__ (keep in sync)
  "RUST_NIGHTLY_VERSION": "nightly-2026-07-14", // __RUST_NIGHTLY_VERSION_MARKER__ (keep in sync)

  // TypeScript:
  "TS_NODE_PROJECT": "${HERMIT_ENV}/tsconfig.json",
}
