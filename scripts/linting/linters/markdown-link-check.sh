#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

MARKDOWN_FILES=$(_list_source_files '**/*.md')

(
  printf "\n\n🧪 markdown-link-check 🧪\n\n\n"
  echo "$MARKDOWN_FILES" | xargs markdown-link-check --quiet --verbose
)
