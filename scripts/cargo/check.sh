#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  # Run setup first
  "$REPO_ROOT/scripts/setup/cargo.sh"
)

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"

  cd "$REPO_ROOT"
  cargo check --offline --all --all-targets

  printf "\n\n✅ Check Success ✅\n\n\n"
)
