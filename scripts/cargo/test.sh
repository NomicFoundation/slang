#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/cargo/setup.sh"
)

(
  printf "\n\n🧪 Running Tests 🧪\n\n\n"

  cd "$REPO_ROOT"
  cargo test --no-fail-fast --offline --all --all-targets

  printf "\n\n✅ Tests Success ✅\n\n\n"
)
