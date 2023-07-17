#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

function _napi_publish() {
  package="$1"

  package_name=$(jq --raw-output '.name' "$package")
  printf "\n\n🚀 Publishing Package: %s 🚀\n\n\n" "$package_name"

  repo_version=$(_slang_repo_version)
  echo "Repository version: $repo_version"

  published_version=$(npm view "$package_name" version)
  echo "Published version: $published_version"

  if [[ "$repo_version" == "$published_version" ]]; then
    echo "Skipping publish, since the repository version is already published."
    return 0
  fi

  cd "$REPO_ROOT"

  command=(
    npm publish
    "$(dirname "$package")"
    --access public
  )

  if [[ "${SLANG_PUBLISH:-}" != "true" ]]; then
    command+=(--dry-run)
  fi

  _group_output "${command[@]}"
}

(
  # Publish platform-specific packages first:

  platform_packages=$(_list_source_files "$PACKAGE_DIR" "platforms/**/package.json")

  echo "$platform_packages" | while read -r platform_package; do
    _napi_publish "$platform_package"
  done

  # Then publish the root package:

  _napi_publish "$PACKAGE_DIR/target/dist/package.json"
)
