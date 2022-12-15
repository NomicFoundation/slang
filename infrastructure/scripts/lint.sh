#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Run setup first
  "$REPO_ROOT/infrastructure/scripts/setup.sh"
)

(
  printf "\n\n🧪 Running All Linters 🧪\n\n\n"

  "$REPO_ROOT/infrastructure/scripts/linters/bash.sh"
  "$REPO_ROOT/infrastructure/scripts/linters/cargofmt.sh"
  "$REPO_ROOT/infrastructure/scripts/linters/cspell.sh"
  "$REPO_ROOT/infrastructure/scripts/linters/markdown.sh"
  "$REPO_ROOT/infrastructure/scripts/linters/prettier.sh"

  printf "\n\n✅ Linting Success ✅\n\n\n"
)
