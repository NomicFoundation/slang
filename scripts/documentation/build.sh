#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/setup/pipenv.sh"
)

(
  printf "\n\n📚 Building Documentation Site 📚\n\n\n"

  cd "$REPO_ROOT/documentation"
  python3 -m pipenv run mkdocs build --clean --strict

  printf "\n\n✅ Build Success ✅\n\n\n"
)
