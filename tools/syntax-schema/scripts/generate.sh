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

EBNF_CRATE="$HERMIT_ENV/code-analysis/crates/parsers/ebnf"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "validate_manifest" -- \
  --manifest-input "$EBNF_CRATE/grammar/manifest.yml"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$EBNF_CRATE/grammar/manifest.yml" \
  --chumsky-output "$EBNF_CRATE/src/generated/"

cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "manifest_to_ebnf" -- \
  --manifest-input "$EBNF_CRATE/grammar/manifest.yml" \
  --ebnf-output "$EBNF_CRATE/src/generated/derived.ebnf"
