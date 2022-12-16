#!/bin/bash
set -euo pipefail

# See https://github.com/actions/checkout/issues/766
[[ "${GITHUB_WORKSPACE:-}" ]] && git config --global --add safe.directory "$GITHUB_WORKSPACE"

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" ]] || source "$(dirname "${BASH_SOURCE[0]}")/../bin/activate-hermit"
