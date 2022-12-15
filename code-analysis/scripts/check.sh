#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  # Run setup first
  "$THIS_DIR/setup.sh"
)

declare -a rust_flags=(
  "${RUSTFLAGS:-}"
  "--warn unused_crate_dependencies"
  "--deny warnings"
)

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$PROJECT_DIR"

  export RUSTFLAGS="${rust_flags[*]}"
  export SLANG_CODEGEN_CHECK_ONLY="true"

  cargo check --offline --all --all-targets
  cargo test --no-fail-fast --offline --all --all-targets
  cargo fmt --check --all
)

printf "\n\n✅ Check Success ✅\n\n\n"
