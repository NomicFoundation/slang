#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🚀 Publishing To Cargo 🚀\n\n\n"

  # TODO: Make sure to pass `CARGO_REGISTRY_TOKEN` when you disable dry-run:
  cargo publish \
    --all-features \
    --dry-run \
    --package "slang_solidity"

  printf "\n\n✅ Published to Cargo Successfully ✅\n\n\n"
)

(
  printf "\n\n🚀 Publishing To NPM 🚀\n\n\n"

  # TODO: Make sure to pass `NPM_TOKEN` when you disable dry-run:
  npm publish \
    --access "public" \
    --dry-run \
    --workspace "slang-solidity"

  printf "\n\n✅ Published to NPM Successfully ✅\n\n\n"
)
