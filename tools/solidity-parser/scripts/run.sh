#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

cargo install --path ../syntax-schema

###################################################
# Solidity from the intended source manifest
###################################################

manifest_to_ebnf \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml" \
  --ebnf-output "$PROJECT_DIR/syntax/solidity/derived.ebnf"

manifest_to_chumsky \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml" \
  --chumsky-output "$PROJECT_DIR/syntax/solidity/derived.rs"
