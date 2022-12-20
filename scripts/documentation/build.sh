#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/documentation/setup.sh"
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

(
  # If deploying from CI, let the deployment job know which files to upload
  if [[ "${GITHUB_JOB:-}" == "github-pages" ]]; then
    # Use a relative path, as this script is running within the dev container,
    # but the CI job expects a path that will work on the runner host machine.
    site_dir="$(realpath "$DOCUMENTATION_SITE_DIR" --relative-to="$REPO_ROOT")"
    echo "site_dir=$site_dir" >> "$GITHUB_OUTPUT"
  fi
)
