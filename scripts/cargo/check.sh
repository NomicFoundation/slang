#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"

  cd "$REPO_ROOT"
  cargo check --offline --all --all-targets --all-features

  printf "\n\n✅ Check Success ✅\n\n\n"
)
