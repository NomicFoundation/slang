#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

(
  printf "\n\n🧪 cargofmt 🧪\n\n\n"
  cd "$REPO_ROOT/code-analysis"

  if [[ "${CI:-}" ]]; then
    cargo fmt --check --all
  else
    cargo fmt --all
  fi
)
