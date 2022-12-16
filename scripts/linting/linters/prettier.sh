#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

(
  printf "\n\n🧪 prettier 🧪\n\n\n"
  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    prettier --check "$REPO_ROOT"
  else
    prettier --write "$REPO_ROOT"
  fi
)
