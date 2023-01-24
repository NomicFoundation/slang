#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# Enable stack traces for any errors
export RUST_BACKTRACE="full"

if [[ "${CI:-}" ]]; then
  # Strict checks for CI
  declare -a rust_flags=(
    "${RUSTFLAGS:-}"
    "--warn unused_crate_dependencies"
    "--deny warnings"
  )
else
  # Optimizations for local development
  declare -a rust_flags=(
    "${RUSTFLAGS:-}"
    "--codegen target-cpu=native"
  )
fi

export RUSTFLAGS="${rust_flags[*]}"
