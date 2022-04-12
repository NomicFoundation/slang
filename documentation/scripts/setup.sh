#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  printf "\n\n📦 Installing Dependencies 📦\n\n\n"
  cd "$PROJECT_DIR"
  pip3 install "pipenv==2022.3.28"
  python3 -m pipenv install --deploy
)

printf "\n\n✅ Setup Success ✅\n\n\n"
