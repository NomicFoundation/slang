#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

###################################################
# EBNF from the intended source manifest
###################################################

# Do this twice to auto-dogfood the ebnf -> chumsky process
# Should be a fixed point

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "validate_manifest" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --chumsky-output "$PROJECT_DIR/src/ebnf/generated/"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_ebnf" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --ebnf-output "$PROJECT_DIR/src/ebnf/generated/derived.ebnf"
