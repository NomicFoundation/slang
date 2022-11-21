#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  printf "\n\n📦 Installing Dependencies 📦\n\n\n"

  # run both in parallel and wait for them to finish
  (
    # install cargo crates
    cd "$PROJECT_DIR"
    cargo fetch --locked
  )
  (
    # install infra packages (need prettier for formatting generated files)
    "$REPO_ROOT/infrastructure/scripts/setup.sh"
  )
)

printf "\n\n✅ Setup Success ✅\n\n\n"
