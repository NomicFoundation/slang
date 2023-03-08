#!/bin/bash
set -euo pipefail

# Import common utilities, after Hermit is activated:
# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/_utils.sh"

# Activate the Hermit environment:
eval "$(_print_hermit_env)"

# If running in GitHub CI, mark the repository as safe.directory in git:
# See https://github.com/actions/checkout/issues/766
if [[ "${CI:-}" && "${GITHUB_WORKSPACE:-}" ]]; then
  cd "$REPO_ROOT"
  git config --global --add safe.directory "$GITHUB_WORKSPACE"
fi
