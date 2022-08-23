#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  # Run setup first
  "$DOCUMENTATION_DIR/scripts/setup.sh"
)

(
  # Merge Config Files
  mkdir -p "$DOCUMENTATION_TARGET_DIR"

  # shellcheck disable=SC2016
  yq eval-all '. as $file ireduce ({}; . *+ $file )' \
    "$DOCUMENTATION_DIR/mkdocs.config.yml" \
    "$DOCUMENTATION_DIR/mkdocs.specification.yml" \
    "$DOCUMENTATION_DIR/mkdocs.theme.yml" \
    > "$DOCUMENTATION_TARGET_DIR/mkdocs.yml"
)

(
  printf "\n\n📚 Building Static Assets 📚\n\n\n"
  cd "$DOCUMENTATION_TARGET_DIR"
  python3 -m pipenv run mkdocs build --clean --strict
)

printf "\n\n✅ Build Success ✅\n\n\n"
