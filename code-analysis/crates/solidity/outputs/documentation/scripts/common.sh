#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
[[ -z "${HERMIT_ENV:-}" ]] && source "$THIS_DIR/../../../../../../bin/activate-hermit"

export DOCUMENTATION_DIR="$THIS_DIR/.."
export DOCUMENTATION_SOURCE_FILES="$DOCUMENTATION_DIR/docs"
export DOCUMENTATION_TARGET_DIR="$DOCUMENTATION_DIR/target"
export DOCUMENTATION_SITE_DIR="$DOCUMENTATION_DIR/target/site"
