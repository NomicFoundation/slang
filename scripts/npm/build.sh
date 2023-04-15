#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n📚 Building NPM Workspace 📚\n\n\n"

  cd "$REPO_ROOT"
  npm run "slang-build" --workspaces --if-present

  printf "\n\n✅ Build Success ✅\n\n\n"
)
