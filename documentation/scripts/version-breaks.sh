#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")
REPO_ROOT=$(dirname "$PROJECT_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n🚀 Running Version Break Tests 🚀\n\n\n"
  unset ACTIVE_HERMIT # Run the other tool in its own environment
  "$REPO_ROOT/tools/version-breaks/scripts/run.sh" --tests-dir "$PROJECT_DIR/docs"
)
