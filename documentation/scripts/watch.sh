#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

(
  # Perform a full build first
  "$THIS_DIR/build.sh"
)

(
  printf "\n\n🌐 Serving Documentation 🌐\n\n\n"
  cd "$PROJECT_DIR/target"
  # VS Code devcontainer is configured to automatically render the website when this port is forwarded.
  python3 -m pipenv run mkdocs serve --watch-theme --dev-addr "127.0.0.1:5353"
)
