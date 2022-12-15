#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  printf "\n\n📦 Installing NPM Packages 📦\n\n\n"

  export NPM_CONFIG_CACHE="$NPM_CONFIG_PREFIX/cache"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    npm install --ci
  else
    npm install
  fi

  printf "\n\n✅ NPM Packages Installed ✅\n\n\n"
)
