#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

# If running in GitHub CI, mark the repository as a safe directory in git:
# See https://github.com/actions/checkout/issues/766
if [[ "${CI:-}" && "${GITHUB_WORKSPACE:-}" ]]; then
  (
    printf "\n\n ⚙️ Configuring GitHub Workspace ⚙️\n\n\n"

    cd "$REPO_ROOT"
    git config --global --add safe.directory "$GITHUB_WORKSPACE"

    printf "\n\n✅ GitHub Workspace Configured ✅\n\n\n"
  )
fi
