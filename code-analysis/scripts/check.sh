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
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$PROJECT_DIR"

  export RUST_BACKTRACE="full"
  export SLANG_VALIDATE_GENERATED_FILES="true"

  cargo check --locked
  cargo fmt --check --all
)
