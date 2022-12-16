#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  # Perform a full build first
  "$REPO_ROOT/scripts/documentation/build.sh"
)

(
  printf "\n\n🌐 Serving Documentation 🌐\n\n\n"

  cd "$REPO_ROOT/documentation/target"
  # VS Code devcontainer is configured to automatically render the website when this port is forwarded.
  python3 -m pipenv run mkdocs serve --watch-theme --dev-addr "127.0.0.1:5353"
)
