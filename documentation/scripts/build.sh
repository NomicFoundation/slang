#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  # Run setup first
  "$THIS_DIR/setup.sh"
)

(
  # Merge Config Files
  mkdir -p "$PROJECT_DIR/target"

  # shellcheck disable=SC2016
  yq eval-all '. as $file ireduce ({}; . *+ $file )' \
    "$PROJECT_DIR/mkdocs.config.yml" \
    "$PROJECT_DIR/mkdocs.specification.yml" \
    "$PROJECT_DIR/mkdocs.theme.yml" \
    > "$PROJECT_DIR/target/mkdocs.yml"
)

(
  printf "\n\n📚 Building Static Assets 📚\n\n\n"
  cd "$PROJECT_DIR/target"
  python3 -m pipenv run mkdocs build --clean --strict
)

printf "\n\n✅ Build Success ✅\n\n\n"
