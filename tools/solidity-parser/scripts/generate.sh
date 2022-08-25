#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

###################################################
# Solidity from the intended source manifest
###################################################

SOLIDITY_CRATE="$HERMIT_ENV/code-analysis/crates/parsers/solidity"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "validate_manifest" -- \
  --manifest-input "$SOLIDITY_CRATE/grammar/manifest.yml"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_ebnf" -- \
  --manifest-input "$SOLIDITY_CRATE/grammar/manifest.yml" \
  --ebnf-output "$SOLIDITY_CRATE/src/generated/derived.ebnf"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$SOLIDITY_CRATE/grammar/manifest.yml" \
  --chumsky-output "$SOLIDITY_CRATE/src/generated"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_spec" -- \
  --manifest-input "$SOLIDITY_CRATE/grammar/manifest.yml" \
  --documentation-folder "$HERMIT_ENV/documentation"
