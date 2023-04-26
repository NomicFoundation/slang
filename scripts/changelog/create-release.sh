#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

if [[ ! "${GITHUB_ACTIONS:-}" ]]; then
  printf "\n\n❌ Error: Script must be invoked by GitHub Actions in CI ❌\n\n\n"
  exit 1
fi

(
  printf "\n\n📜 Creating GitHub Release 📜\n\n\n"

  export CHANGELOG_DIR="$REPO_ROOT/scripts/changelog"
  node "$REPO_ROOT/scripts/changelog/create-release.mjs"

  printf "\n\n✅ GitHub Release Success ✅\n\n\n"
)
