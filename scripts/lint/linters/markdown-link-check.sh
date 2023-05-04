#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

MARKDOWN_FILES=$(_list_source_files "$REPO_ROOT" "**/*.md")

(
  printf "\n\n🧪 markdown-link-check 🧪\n\n\n"

  echo "$MARKDOWN_FILES" \
    | xargs -n 8 -P 8 \
      markdown-link-check --verbose \
      --config "$REPO_ROOT/.markdown-link-check.json"
)
