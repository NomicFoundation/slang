#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

(
  printf "\n\n📦 Installing Pipenv 📦\n\n\n"

  # Search for the version string matching: `pipenv = "==YYYY.MM.DD"`
  PIPENV_VERSION="$(sed -n 's/^pipenv = "==\([^"]*\)"$/\1/p' "$REPO_ROOT/Pipfile")"
  echo "Using pipenv version: $PIPENV_VERSION"

  cd "$REPO_ROOT"
  pip3 install "pipenv==$PIPENV_VERSION"

  printf "\n\n✅ Pipenv Installed ✅\n\n\n"
)

(
  printf "\n\n📦 Installing Pipenv Packages 📦\n\n\n"

  cd "$REPO_ROOT"

  if [[ "${CI:-}" ]]; then
    python3 -m pipenv install --deploy
  else
    python3 -m pipenv install
  fi

  printf "\n\n✅ Pipenv Packages Installed ✅\n\n\n"
)
