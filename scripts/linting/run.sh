#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/setup/npm.sh"
)

(
  printf "\n\n🧪 Running All Linters 🧪\n\n\n"

  "$REPO_ROOT/scripts/linting/linters/cargofmt.sh"
  "$REPO_ROOT/scripts/linting/linters/cspell.sh"
  "$REPO_ROOT/scripts/linting/linters/markdown-link-check.sh"
  "$REPO_ROOT/scripts/linting/linters/markdownlint.sh"
  "$REPO_ROOT/scripts/linting/linters/prettier.sh"
  "$REPO_ROOT/scripts/linting/linters/shellcheck.sh"

  printf "\n\n✅ Linting Success ✅\n\n\n"
)
