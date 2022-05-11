#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  cd "$PROJECT_DIR"
  cargo run -- \
    --manifest-path "$PROJECT_DIR/syntax/manifest.yml" \
    --ebnf-output "$PROJECT_DIR/syntax/grammar.ebnf" \
    --parser-output "TODO/path/to/compiler/crate/that/has/parser"
)
