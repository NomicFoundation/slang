#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Perform a full build first
  "$REPO_ROOT/scripts/documentation/build.sh"
)

(
  printf "\n\n🚀 Deploy to GitHub Pages 🚀\n\n\n"
  cd "$REPO_ROOT/documentation"
  python3 -m pipenv run ghp-import --no-jekyll --no-history --push "$DOCUMENTATION_SITE_DIR"
)

printf "\n\n✅ Workflow Success ✅\n\n\n"
