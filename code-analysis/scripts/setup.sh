#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  printf "\n\n📦 Installing Cargo Crates 📦\n\n\n"

  cd "$REPO_ROOT/code-analysis"

  if [[ "${CI:-}" ]]; then
    cargo fetch --locked
  else
    cargo fetch
  fi

  printf "\n\n✅ Cargo Crates Installed ✅\n\n\n"
)
