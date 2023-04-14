#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n📚 Publishing NPM Workspace 📚\n\n\n"

  cd "$REPO_ROOT"
  npm run "slang-publish" --workspaces --if-present

  printf "\n\n✅ NPM Publish Success ✅\n\n\n"
)
