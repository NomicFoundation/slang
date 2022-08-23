#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
[[ -z "${HERMIT_ENV:-}" ]] && source "$THIS_DIR/../../../bin/activate-hermit"

export LINTING_DIR="$THIS_DIR/.."
export NPM_BIN_DIR="$LINTING_DIR/node_modules/.bin"
