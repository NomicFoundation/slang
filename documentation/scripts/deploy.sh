#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  # Perform a full build first
  "$THIS_DIR/build.sh"
)

if [ "$GITHUB_REF" != "refs/heads/main" ] || [ "$GITHUB_EVENT_NAME" != "push" ]; then
  printf "\n\n❌ Deployment environment not detected: Aborting deployment ❌\n\n"
  exit 1
fi

(
  printf "\n\n🚀 Deploy to GitHub Pages 🚀\n\n\n"
  cd "$PROJECT_DIR"
  python3 -m pipenv run ghp-import --no-jekyll --no-history --push "$SITE_DIR"
)

printf "\n\n✅ Workflow Success ✅\n\n\n"
