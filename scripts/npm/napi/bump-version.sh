#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Update main package:
  repo_version=$(_slang_repo_version)
  main_package=$(jq '.version = "'"$repo_version"'"' "$PACKAGE_DIR/package.json")
  echo "$main_package" > "$PACKAGE_DIR/package.json"

  # Update platform-specific packages, without actually publishing:
  cd "$PACKAGE_DIR"
  export npm_config_dry_run=true
  _group_output napi prepublish --skip-gh-release --prefix "platforms"

  # Format all packages:
  _list_source_files "$PACKAGE_DIR" "**/package.json" | xargs prettier --write
)
