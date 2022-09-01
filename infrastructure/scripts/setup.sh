#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  printf "\n\n📦 Installing Dependencies 📦\n\n\n"
  cd "$LINTING_DIR"
  npm install --ci
)

printf "\n\n✅ Setup Success ✅\n\n\n"
