#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# shellcheck source=/dev/null
source "$THIS_DIR/common.sh"

(
  # Perform a full build first
  "$DOCUMENTATION_DIR/scripts/build.sh"
)

(
  printf "\n\n🌐 Serving Documentation 🌐\n\n\n"
  cd "$DOCUMENTATION_TARGET_DIR"
  # VS Code devcontainer is configured to automatically render the website when this port is forwarded.
  python3 -m pipenv run mkdocs serve --watch-theme --dev-addr "127.0.0.1:5353"
)
