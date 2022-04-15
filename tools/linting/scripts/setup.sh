#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n📦 Installing Dependencies 📦\n\n\n"
  cd "$PROJECT_DIR"
  npm install --ci
)

printf "\n\n✅ Setup Success ✅\n\n\n"
