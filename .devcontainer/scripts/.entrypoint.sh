#!/bin/bash
set -euo pipefail

# See https://github.com/actions/checkout/issues/766
git config --global --add safe.directory "$GITHUB_WORKSPACE"

"$SLANG_BUILD_SCRIPT"
