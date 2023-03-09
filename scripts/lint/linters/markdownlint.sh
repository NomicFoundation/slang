#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

MARKDOWN_FILES=$(_list_source_files '**/*.md')

(
  printf "\n\n🧪 markdownlint 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs markdownlint --dot --config "$REPO_ROOT/.markdownlint.json"
)
