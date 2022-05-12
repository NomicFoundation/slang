#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  cd "$PROJECT_DIR"

  cargo run --bin ebnf_to_grammar -- \
    --ebnf-path "$PROJECT_DIR/syntax/ebnf/original_grammar.ebnf" \
    --grammar-output "$PROJECT_DIR/syntax/ebnf/original_grammar.yml"
)

(
  cd "$PROJECT_DIR"

  cargo run --bin manifest_to_ebnf -- \
    --manifest-path "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
    --ebnf-output "$PROJECT_DIR/syntax/ebnf/grammar.ebnf"
)

(
  cd "$PROJECT_DIR"

  cargo run --bin manifest_to_parser -- \
    --manifest-path "$PROJECT_DIR/syntax/ebnf/manifest.yml" \
    --parser-output "$PROJECT_DIR/syntax/ebnf/parser.rs"
)

(
  cd "$PROJECT_DIR"

  cargo run --bin ebnf_to_grammar -- \
    --ebnf-path "$PROJECT_DIR/syntax/solidity/original_grammar.ebnf" \
    --grammar-output "$PROJECT_DIR/syntax/solidity/original_grammar.yml"
)

(
  cd "$PROJECT_DIR"

  cargo run --bin manifest_to_ebnf -- \
    --manifest-path "$PROJECT_DIR/syntax/solidity/manifest.yml" \
    --ebnf-output "$PROJECT_DIR/syntax/solidity/grammar.ebnf"
)

(
  cd "$PROJECT_DIR"

  cargo run --bin manifest_to_parser -- \
    --manifest-path "$PROJECT_DIR/syntax/solidity/manifest.yml" \
    --parser-output "$PROJECT_DIR/syntax/solidity/parser.rs"
)
