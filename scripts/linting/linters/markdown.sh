#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

MARKDOWN_FILES=$(
  cd "$REPO_ROOT"
  git ls-files --cached --modified --others --exclude-standard | grep ".md$" | xargs realpath --canonicalize-existing 2> /dev/null
)

(
  printf "\n\n🧪 markdownlint 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs -t markdownlint --dot --config "$REPO_ROOT/.markdownlint.json"
)

(
  printf "\n\n🧪 markdown-link-check 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs -t markdown-link-check --quiet --verbose
)
