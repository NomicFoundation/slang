#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# If running in GitHub CI, mark the repository as safe.directory in git:
# See https://github.com/actions/checkout/issues/766

if [[ "${CI:-}" && "${GITHUB_WORKSPACE:-}" ]]; then
  cd "$REPO_ROOT"
  git config --global --add safe.directory "$GITHUB_WORKSPACE"
fi

# Setup all toolchains in order:

(
  "$REPO_ROOT/scripts/setup/cargo.sh"
  "$REPO_ROOT/scripts/setup/npm.sh"
  "$REPO_ROOT/scripts/setup/pipenv.sh"
)
