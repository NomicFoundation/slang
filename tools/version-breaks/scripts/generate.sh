#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  printf "\n\n🚀 Generating Version Breaks Diffs 🚀\n\n\n"
  cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" --bin "version_breaks_diffs" -- \
    --tests-dir "$HERMIT_ENV/documentation/docs"
)
