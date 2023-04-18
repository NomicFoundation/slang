#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🚀 Publishing To Cargo 🚀\n\n\n"
  cd "$REPO_ROOT"

  repo_version=$(_slang_repo_version)

  echo "Repository version: $repo_version"

  published_version=$(
    # "cargo search slang_solidity" will produce the following TOML output.
    # Then "cut" will split by delimeter '"', and take the second field, which is the version.
    #
    # slang_solidity = "1.2.3" # description
    # └────── 1 ──────┘ └ 2 ┘ └──── 3 ─────┘

    cargo search "slang_solidity" | grep '^slang_solidity = "' | cut -d '"' -f 2
  )

  echo "Published version: $published_version"

  if [[ "$repo_version" == "$published_version" ]]; then
    echo "Skipping publish, since the repository version is already published."
    exit 0
  fi

  command=(
    cargo publish
    --package "slang_solidity"
    --all-features
  )

  if [[ "${SLANG_PUBLISH:-}" != "true" ]]; then
    command+=(--dry-run)
  fi

  _group_output "${command[@]}"

  printf "\n\n✅ Cargo Publish Complete ✅\n\n\n"
)
