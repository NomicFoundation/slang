#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/../common.sh"

(
  printf "\n\n🧪 prettier 🧪\n\n\n"
  cd "$HERMIT_ENV"
  "$NPM_BIN_DIR/prettier" --check "$HERMIT_ENV"
)
