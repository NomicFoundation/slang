#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

function _napi_publish() {
  package="$1"

  printf "\n\n🚀 Publishing Package: %s 🚀\n\n\n" "$package"

  package_name=$(jq --raw-output '.name' "$package")

  local_version=$(jq --raw-output '.version' "$package")

  echo "Local version: $local_version"

  published_version=$(
    if [[ ! "$(npm view "$package_name" version &> /dev/null)" ]]; then
      echo "not published yet"
    else
      npm view "$package_name" version
    fi
  )

  echo "Published version: $published_version"

  if [[ "$local_version" == "$published_version" ]]; then
    echo "Skipping publish, since the local version is already published."
    exit 0
  fi

  if [[ "${SLANG_PUBLISH:-}" != "true" ]]; then
    extra_args="--dry-run"
  fi

  cd "$(dirname "$package")"
  _group_output \
    npm publish \
    --access "public" \
    "${extra_args:-}"
}

(
  # Publish platform-specific packages first:

  platforms=$(_list_source_files "$PACKAGE_DIR" "platforms/**/package.json")

  echo "$platforms" | while read -r platform; do
    _group_output _napi_publish "$platform"
  done

  # Then publish the root package:

  _group_output _napi_publish "$PACKAGE_DIR/package.json"
)
