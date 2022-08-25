#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/../common.sh"

BASH_FILES=$(
  cd "$HERMIT_ENV"
  git ls-files --cached --modified --others --exclude-standard | grep ".sh$" | xargs realpath --canonicalize-existing 2> /dev/null
)

(
  printf "\n\n🧪 shellcheck 🧪\n\n\n"
  echo "$BASH_FILES" | xargs -t shellcheck
)
