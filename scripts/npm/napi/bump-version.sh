#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Update main package:
  main_package=$(jq '.version = "'"$SLANG_NEW_VERSION"'"' "$PACKAGE_DIR/package.json")
  echo "$main_package" > "$PACKAGE_DIR/package.json"

  # Update platform-specific packages:
  cd "$PACKAGE_DIR"
  _group_output napi prepublish --skip-gh-release --prefix "platforms"

  # Format all packages:
  _list_source_files "$PACKAGE_DIR" "**/package.json" | xargs prettier --write
)
