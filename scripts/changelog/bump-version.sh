#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

#
# NOTE: This script runs on the GitHub action runner, not inside the repository devcontainer.
#

#
# This repository versions and releases all its artifacts together, generating the same changelog.
# Unfortunately, changesets does not support combining changelogs from multiple packages into one.
#
# So, our workaround is to use a single "changelog" package, and exclude everything else in "$REPO_ROOT/.changeset/config.json".
# We let changesets bump the version of this package, and generate its changelog.
# Then our build process copies the new version and the single changelog to other packages and crates.
#
# Additionally, changesets can only bump versions of packages in the root workspace.
# However, NAPI platform-specific packages cannot be added to the workspace, because other platforms will fail "npm install".
# So we have to bump the versions over ourselves anyways.
#

(
  printf "\n\n📜 Consuming Changesets 📜\n\n\n"

  # This command will:
  # 1) Consume/delete any changeset files currently in "$REPO_ROOT/.changeset"
  # 2) Update the CHANGELOG.md file for the "changelog" package.
  # 3) Bump the version in its package.json accourdingly.
  cd "$REPO_ROOT"
  _group_output changeset version
)

CHANGELOG_DIR="$REPO_ROOT/scripts/changelog"
SLANG_NEW_VERSION=$(jq --raw-output '.version' "$CHANGELOG_DIR/package.json")

(
  printf "\n\n📜 Updating Cargo Crates 📜\n\n\n"

  # TODO: move "cargo-edit" to workspace dependencies, so that it is installed/upgraded with the rest of them.
  # https://github.com/rust-lang/cargo/issues/2267
  _group_output cargo install "cargo-edit" --version "0.11.9"
  _group_output cargo set-version "$SLANG_NEW_VERSION" --workspace
)

(
  printf "\n\n📜 Updating NPM Packages 📜\n\n\n"

  # Update each package in the workspace:
  SLANG_NEW_VERSION="$SLANG_NEW_VERSION" \
    npm run "slang-bump-version" --workspaces --if-present

  # Update lock file:
  cd "$REPO_ROOT"
  _group_output npm install --package-lock-only
)

CHANGELOG_FILES=$(_list_source_files "$REPO_ROOT" "**/CHANGELOG.md")

(
  printf "\n\n📜 Updating Changelogs 📜\n\n\n"
  source="$CHANGELOG_DIR/CHANGELOG.md"

  for destination in $CHANGELOG_FILES; do
    if [[ "$source" != "$destination" ]]; then
      echo "$destination"
      cp "$source" "$destination"
    fi
  done
)

(
  printf "\n\n✅ Updated to version %s ✅\n\n\n" "$SLANG_NEW_VERSION"
)
