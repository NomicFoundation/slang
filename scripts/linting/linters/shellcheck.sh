#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

BASH_FILES=$(_list_source_files '**/*.sh')

(
  printf "\n\n🧪 shellcheck 🧪\n\n\n"
  echo "$BASH_FILES" | xargs shellcheck
)
