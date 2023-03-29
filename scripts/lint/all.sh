#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  printf "\n\n🧪 Running All Linters 🧪\n\n\n"

  _group_output "$REPO_ROOT/scripts/lint/linters/cargofmt.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/cspell.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/markdown-link-check.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/markdownlint.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/prettier.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/shellcheck.sh"
  _group_output "$REPO_ROOT/scripts/lint/linters/yamllint.sh"

  printf "\n\n✅ Linting Success ✅\n\n\n"
)
