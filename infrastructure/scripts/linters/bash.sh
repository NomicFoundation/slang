#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

BASH_FILES=$(
  cd "$REPO_ROOT"
  git ls-files --cached --modified --others --exclude-standard | grep ".sh$" | xargs realpath --canonicalize-existing 2> /dev/null
)

(
  printf "\n\n🧪 shellcheck 🧪\n\n\n"
  echo "$BASH_FILES" | xargs -t shellcheck
)
