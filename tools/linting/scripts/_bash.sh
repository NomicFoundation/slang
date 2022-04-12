#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")
REPO_ROOT=$(realpath "$PROJECT_DIR/../..")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

BASH_FILES=$(
  cd "$REPO_ROOT"
  git ls-files --cached --modified --others --exclude-standard | grep ".sh$" | xargs realpath --canonicalize-existing 2> /dev/null
)

(
  printf "\n\n🧪 shellcheck 🧪\n\n\n"
  echo "$BASH_FILES" | xargs -t shellcheck
)
