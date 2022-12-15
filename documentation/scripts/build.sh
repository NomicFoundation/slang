#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Run setup first
  "$REPO_ROOT/documentation/scripts/setup.sh"
)

(
  # Merge Config Files
  mkdir -p "$REPO_ROOT/documentation/target"

  # shellcheck disable=SC2016
  yq eval-all '. as $file ireduce ({}; . *+ $file )' \
    "$REPO_ROOT/documentation/mkdocs.config.yml" \
    "$REPO_ROOT/documentation/mkdocs.theme.yml" \
    "$REPO_ROOT/documentation/docs/specification/generated/mkdocs.navigation.yml" \
    > "$REPO_ROOT/documentation/target/mkdocs.yml"
)

(
  printf "\n\n📚 Building Documentation Site 📚\n\n\n"

  cd "$REPO_ROOT/documentation/target"
  python3 -m pipenv run mkdocs build --clean --strict

  printf "\n\n✅ Build Success ✅\n\n\n"
)
