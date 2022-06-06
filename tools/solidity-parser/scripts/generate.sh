#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

###################################################
# Solidity from the original antlr grammar
###################################################

cargo run --manifest-path "../syntax-schema/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$PROJECT_DIR/syntax/solidity/original/manifest.yml" \
  --no-default-map --box-non-tokens \
  --chumsky-output "$PROJECT_DIR/src/parser.rs"
