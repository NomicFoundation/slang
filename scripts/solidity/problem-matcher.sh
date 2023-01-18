#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../cargo/_common.sh"

export VSCODE_PROBLEM_MATCHER="true"

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$REPO_ROOT"

  cargo check --offline --lib \
    --manifest-path "$REPO_ROOT/crates/solidity/inputs/schema/Cargo.toml"

  printf "\n\n✅ Check Success ✅\n\n\n"
)
