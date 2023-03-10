#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  printf "\n\n🌐 Serving Documentation 🌐\n\n\n"

  cd "$REPO_ROOT/documentation"
  # VS Code devcontainer is configured to automatically render the website when this port is forwarded.
  python3 -m pipenv run mkdocs serve --watch-theme --dev-addr "127.0.0.1:5353"
)
