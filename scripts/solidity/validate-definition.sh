#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🧪 Checking Solidity Definition 🧪\n\n\n"

  cargo check --offline --lib \
    --manifest-path "$REPO_ROOT/crates/solidity/inputs/language/Cargo.toml"

  printf "\n\n✅ Check Success ✅\n\n\n"
)
