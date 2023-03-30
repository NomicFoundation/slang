#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n🧪 tsc 🧪\n\n\n"

  tsc --project "$REPO_ROOT/tsconfig.json"
)
