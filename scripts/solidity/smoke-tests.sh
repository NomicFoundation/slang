#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  cargo run --offline \
    --manifest-path "$REPO_ROOT/crates/solidity/testing/smoke/Cargo.toml" \
    -- "$@"
)
