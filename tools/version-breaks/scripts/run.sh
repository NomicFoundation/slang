#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(realpath "$THIS_DIR/..")
REPO_ROOT=$(realpath "$PROJECT_DIR/../..")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n🚀 Testing Version Breaks 🚀\n\n\n"
  cd "$PROJECT_DIR"
  cargo run -- --tests-dir "$REPO_ROOT/documentation"
)
