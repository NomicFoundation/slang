#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

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
