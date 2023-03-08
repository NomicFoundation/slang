#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

if [[ "${CI:-}" ]]; then
  # Strict checks for CI
  RUSTFLAGS="${RUSTFLAGS:-} --deny warnings"
fi
