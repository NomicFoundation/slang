#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🚀 Publishing To Cargo 🚀\n\n\n"
  cd "$REPO_ROOT"

  local_version=$(
    cargo metadata --format-version 1 \
      | jq --raw-output '.packages[] | select(.name == "slang_solidity") | .version'
  )

  echo "Local version: $local_version"

  published_version=$(
    # "cargo search slang_solidity" will produce the following TOML output.
    # Then "cut" will split by delimeter '"', and take the second field, which is the version.
    #
    # slang_solidity = "1.2.3" # description
    # └────── 1 ──────┘ └ 2 ┘ └──── 3 ─────┘

    cargo search "slang_solidity" | grep '^slang_solidity = "' | cut -d '"' -f 2
  )

  echo "Published version: $published_version"

  if [[ "$local_version" == "$published_version" ]]; then
    echo "Skipping publish, since the local version is already published."
    exit 0
  fi

  if [[ "${SLANG_PUBLISH:-}" != "true" ]]; then
    extra_args="--dry-run"
  fi

  _group_output \
    cargo publish \
    --all-features \
    --package "slang_solidity" \
    "${extra_args:-}"

  printf "\n\n✅ Cargo Publish Complete ✅\n\n\n"
)
