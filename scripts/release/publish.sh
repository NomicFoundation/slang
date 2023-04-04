#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🚀 Publishing To Cargo 🚀\n\n\n"

  _group_output \
    cargo publish \
    --all-features \
    --package "slang_solidity"

  printf "\n\n✅ Published to Cargo Successfully ✅\n\n\n"
)

(
  printf "\n\n🚀 Publishing To NPM 🚀\n\n\n"

  # TODO: Make sure to pass `NPM_TOKEN` when you disable dry-run:
  _group_output \
    npm publish \
    --access "public" \
    --dry-run \
    --workspace "slang-solidity"

  printf "\n\n✅ Published to NPM Successfully ✅\n\n\n"
)
