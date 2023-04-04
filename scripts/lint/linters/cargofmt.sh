#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

(
  printf "\n\n🧪 cargofmt 🧪\n\n\n"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    cargo fmt --all --verbose --check
  else
    cargo fmt --all --verbose
  fi
)
