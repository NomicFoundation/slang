#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
[[ -z "${HERMIT_ENV:-}" ]] && source "$THIS_DIR/../../bin/activate-hermit"

PROJECT_DIR=$(dirname "$THIS_DIR")
export PROJECT_DIR

REPO_ROOT=$(dirname "$PROJECT_DIR")
export REPO_ROOT
