#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

(
  printf "\n\n📦 Installing Rustup %s 📦\n\n\n" "$RUST_VERSION"

  _group_output rustup default "$RUST_VERSION"

  printf "\n\n✅ Rustup Installed ✅\n\n\n"
)

(
  printf "\n\n📦 Installing Cargo Crates 📦\n\n\n"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    _group_output cargo fetch --locked
  else
    _group_output cargo fetch
  fi

  printf "\n\n✅ Cargo Crates Installed ✅\n\n\n"
)

if [[ ! "${CI:-}" ]]; then
  (
    printf "\n\n📦 Installing Rust Analyzer 📦\n\n\n"

    cd "$REPO_ROOT"
    rust-analyzer --version
    rust-src --version

    if [[ "$(rust-src --version)" != *"$RUST_VERSION"* ]]; then
      printf "\n\n❌ rust-src version does not match %s ❌\n\n\n" "$RUST_VERSION"
      exit 1
    fi

    printf "\n\n✅ Rust Analyzer Installed ✅\n\n\n"
  )
fi
