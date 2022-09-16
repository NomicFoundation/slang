#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  # Perform a full build first
  "$DOCUMENTATION_DIR/scripts/build.sh"
)

if [ "$GITHUB_REF" != "refs/heads/main" ] || [ "$GITHUB_EVENT_NAME" != "push" ]; then
  printf "\n\n❌ Deployment environment not detected: Aborting deployment ❌\n\n"
  exit 1
fi

(
  printf "\n\n🚀 Deploy to GitHub Pages 🚀\n\n\n"
  cd "$DOCUMENTATION_DIR"
  python3 -m pipenv run ghp-import --no-jekyll --no-history --push "$DOCUMENTATION_SITE_DIR"
)

printf "\n\n✅ Workflow Success ✅\n\n\n"
