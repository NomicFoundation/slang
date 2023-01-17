#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  cargo run \
    --manifest-path "$REPO_ROOT/Cargo.toml" \
    --bin "$(basename "$0")" \
    -- "$@"
)
