#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

###################################################
# EBNF from the intended source manifest
###################################################

# Do this twice to auto-dogfood the ebnf -> chumsky process
# Should be a fixed point

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "validate_manifest" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --chumsky-output "$PROJECT_DIR/src/ebnf/"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_ebnf" -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --ebnf-output "$PROJECT_DIR/syntax/ebnf/derived.ebnf"
