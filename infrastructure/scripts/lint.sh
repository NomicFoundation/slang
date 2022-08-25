#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  # Run setup first
  "$THIS_DIR/setup.sh"
)

(
  # Then run specific linters
  "$THIS_DIR/linters/bash.sh"
  "$THIS_DIR/linters/cspell.sh"
  "$THIS_DIR/linters/markdown.sh"
  "$THIS_DIR/linters/prettier.sh"
)

printf "\n\n✅ Lint Success ✅\n\n\n"
