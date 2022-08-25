#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/../common.sh"

MARKDOWN_FILES=$(
  cd "$HERMIT_ENV"
  git ls-files --cached --modified --others --exclude-standard | grep ".md$" | xargs realpath --canonicalize-existing 2> /dev/null
)

(
  printf "\n\n🧪 markdownlint 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs -t "$NPM_BIN_DIR/markdownlint" --dot --config "$HERMIT_ENV/.markdownlint.json"
)

(
  printf "\n\n🧪 markdown-link-check 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs -t "$NPM_BIN_DIR/markdown-link-check" --quiet --verbose
)
