#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

(
  printf "\n\n🧪 cspell 🧪\n\n\n"

  cspell lint --show-context --show-suggestions --dot --gitignore --root "$REPO_ROOT"
)
