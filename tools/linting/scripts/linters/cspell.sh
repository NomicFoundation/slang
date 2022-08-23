#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/../common.sh"

(
  printf "\n\n🧪 cspell 🧪\n\n\n"
  "$NPM_BIN_DIR/cspell" lint --show-context --show-suggestions --dot --gitignore --root "$HERMIT_ENV"
)
