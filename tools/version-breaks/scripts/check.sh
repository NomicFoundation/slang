#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$PROJECT_DIR"
  cargo check --locked
  cargo fmt --check --all
)
