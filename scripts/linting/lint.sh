#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/linting/setup.sh"
)

(
  printf "\n\n🧪 Running All Linters 🧪\n\n\n"

  "$REPO_ROOT/scripts/linting/linters/bash.sh"
  "$REPO_ROOT/scripts/linting/linters/cargofmt.sh"
  "$REPO_ROOT/scripts/linting/linters/cspell.sh"
  "$REPO_ROOT/scripts/linting/linters/markdown.sh"
  "$REPO_ROOT/scripts/linting/linters/prettier.sh"

  printf "\n\n✅ Linting Success ✅\n\n\n"
)
