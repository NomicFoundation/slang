#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

(
  # Run setup first
  "$REPO_ROOT/code-analysis/scripts/setup.sh"
)

(
  cargo run \
    --manifest-path "$REPO_ROOT/code-analysis/Cargo.toml" \
    --bin "$(basename "$0")" \
    -- "$@"
)
