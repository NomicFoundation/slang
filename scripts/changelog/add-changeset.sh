#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n📜 Adding a new changeset 📜\n\n\n"

  cd "$REPO_ROOT"
  changeset add

  printf "\n\n✅ Added changeset ✅\n\n\n"
)
