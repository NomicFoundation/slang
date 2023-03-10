#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  printf "\n\n🧪 Running All Linters 🧪\n\n\n"

  "$REPO_ROOT/scripts/lint/linters/cargofmt.sh"
  "$REPO_ROOT/scripts/lint/linters/cspell.sh"
  "$REPO_ROOT/scripts/lint/linters/markdown-link-check.sh"
  "$REPO_ROOT/scripts/lint/linters/markdownlint.sh"
  "$REPO_ROOT/scripts/lint/linters/prettier.sh"
  "$REPO_ROOT/scripts/lint/linters/shellcheck.sh"
  "$REPO_ROOT/scripts/lint/linters/yamllint.sh"

  printf "\n\n✅ Linting Success ✅\n\n\n"
)
