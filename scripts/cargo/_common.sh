#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# Enable stack traces for any errors
export RUST_BACKTRACE="full"

RUSTFLAGS="${RUSTFLAGS:-}"
RUSTFLAGS="$(_try_append_arg "$RUSTFLAGS" "--warn unused_crate_dependencies")"

if [[ "${CI:-}" ]]; then
  # Strict checks for CI
  RUSTFLAGS="$(_try_append_arg "$RUSTFLAGS" "--deny warnings")"
else
  # Optimizations for local development
  RUSTFLAGS="$(_try_append_arg "$RUSTFLAGS" "--codegen target-cpu=native")"
fi

export RUSTFLAGS
