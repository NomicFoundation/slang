#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🧪 Running NPM Tests 🧪\n\n\n"

  cd "$REPO_ROOT"
  npm run "slang-test" --workspaces --if-present

  printf "\n\n✅ Tests Success ✅\n\n\n"
)
