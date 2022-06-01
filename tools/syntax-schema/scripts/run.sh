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

cargo run --bin manifest_to_chumsky -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --chumsky-output "$PROJECT_DIR/src/ebnf/parser.rs"

cargo run --bin manifest_to_ebnf -- \
  --manifest-input "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
  --ebnf-output "$PROJECT_DIR/syntax/ebnf/derived.ebnf"
