#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
[[ -z "${HERMIT_ENV:-}" ]] && source "$THIS_DIR/../../bin/activate-hermit"

export PROJECT_DIR="$THIS_DIR/.."
