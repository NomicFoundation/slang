#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

BASH_FILES=$(_list_source_files "$REPO_ROOT" "**/*.sh")

(
  printf "\n\n🧪 shellcheck 🧪\n\n\n"

  echo "$BASH_FILES"

  echo "$BASH_FILES" | xargs shellcheck
)
