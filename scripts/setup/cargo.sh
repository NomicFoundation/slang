#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

if [ -z "${CI:-}" ]; then
  (
    printf "\n\n📦 Installing Rust Analyzer 📦\n\n\n"

    cd "$REPO_ROOT"
    "$REPO_ROOT/bin/rust-analyzer" --version
    "$REPO_ROOT/bin/rust-src" --version

    printf "\n\n✅ Rust Analyzer Installed ✅\n\n\n"

  )
fi

(
  printf "\n\n📦 Installing Cargo Crates 📦\n\n\n"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    cargo fetch --locked
  else
    cargo fetch
  fi

  printf "\n\n✅ Cargo Crates Installed ✅\n\n\n"
)
