#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🧪 Running Tests 🧪\n\n\n"

  cd "$REPO_ROOT"
  cargo test --no-fail-fast --offline --all --all-targets --all-features

  printf "\n\n✅ Tests Success ✅\n\n\n"
)
