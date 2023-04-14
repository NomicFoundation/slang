#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

(
  printf "\n\n📦 Installing NPM Packages 📦\n\n\n"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    _group_output npm install --ci
  else
    _group_output npm install
  fi

  printf "\n\n✅ NPM Packages Installed ✅\n\n\n"
)
