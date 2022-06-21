#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")
REPO_ROOT=$(realpath "$PROJECT_DIR/../..")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n🚀 Generating Version Breaks Diffs 🚀\n\n\n"
  cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "version_breaks_diffs" -- \
    --tests-dir "$REPO_ROOT/documentation/docs"
)
