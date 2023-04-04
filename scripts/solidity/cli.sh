#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  cargo run --offline \
    --manifest-path "$REPO_ROOT/crates/solidity/outputs/rust/slang_solidity/Cargo.toml" \
    -- "$@"
)
