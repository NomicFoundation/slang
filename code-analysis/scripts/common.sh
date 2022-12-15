#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../../infrastructure/scripts/common.sh"

# Enable stack traces for any errors
export RUST_BACKTRACE="full"

if [[ "${CI:-}" ]]; then
  # Strict checks for rustc
  declare -a rust_flags=(
    "${RUSTFLAGS:-}"
    "--warn unused_crate_dependencies"
    "--deny warnings"
  )

  export RUSTFLAGS="${rust_flags[*]}"
  export SLANG_CODEGEN_CHECK_ONLY="true"
fi
