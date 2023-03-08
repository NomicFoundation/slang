#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  cargo run \
    --manifest-path "$REPO_ROOT/Cargo.toml" \
    --bin "$(basename "$0")" \
    -- "$@"
)
