#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/setup/cargo.sh"
)

(
  printf "\n\n🧪 Running Tests 🧪\n\n\n"

  cd "$REPO_ROOT"
  cargo test --no-fail-fast --offline --all --all-targets

  printf "\n\n✅ Tests Success ✅\n\n\n"
)
