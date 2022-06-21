#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n🧪 Checking Project 🧪\n\n\n"
  cd "$PROJECT_DIR"
  cargo check --locked
  cargo fmt --check --all
)

(
  printf "\n\n🧪 Checking Generated Files 🧪\n\n\n"
  cd "$PROJECT_DIR"

  if [[ "$(git status --porcelain)" ]]; then
    echo "Found local changes before running. Aborting"
    exit 1
  fi

  "$PROJECT_DIR/scripts/generate.sh"

  if [[ "$(git status --porcelain)" ]]; then
    echo "Found local changes after running. Aborting"
    exit 1
  fi
)
