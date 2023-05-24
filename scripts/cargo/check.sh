#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$REPO_ROOT"

  command=(
    cargo check
    --offline
    --all
    --all-targets
    --all-features
  )

  # Strict checks for CI
  if [[ "${CI:-}" ]]; then
    command+=(
      --config
      'build.rustflags = ["--deny", "warnings"]'
    )
  fi

  "${command[@]}"

  printf "\n\n✅ Check Success ✅\n\n\n"
)
