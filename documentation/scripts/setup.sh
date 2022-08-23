#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  printf "\n\n📦 Installing Dependencies 📦\n\n\n"
  cd "$DOCUMENTATION_DIR"
  pip3 install "pipenv==2022.3.28"
  python3 -m pipenv install --deploy
)

printf "\n\n✅ Setup Success ✅\n\n\n"
