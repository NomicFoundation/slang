#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  printf "\n\n📦 Installing Pipenv 📦\n\n\n"

  # Use the top-level `Pipfile` to search for the version of `pipenv` to install.
  # This should match lines that have: `pipenv = "==YYYY.MM.DD"`
  PIPENV_VERSION="$(sed -n 's/^pipenv = "==\([^"]*\)"$/\1/p' "$REPO_ROOT/Pipfile")"
  echo "Using pipenv version: $PIPENV_VERSION"

  cd "$REPO_ROOT"
  pip3 install "pipenv==$PIPENV_VERSION"

  printf "\n\n✅ Pipenv Installed ✅\n\n\n"
)

PIP_FILES=$(_list_source_files '**/Pipfile')

(
  printf "\n\n📦 Installing Pipenv Packages 📦\n\n"

  # Loop over individual Pipfiles, and install their packages using `pipenv`.
  for file in $PIP_FILES; do
    printf "\n- Pipfile: %s\n" "$file"
    cd "$(dirname "$file")"

    if [[ "${CI:-}" ]]; then
      python3 -m pipenv install --deploy
    else
      python3 -m pipenv install
    fi
  done

  printf "\n\n✅ Pipenv Packages Installed ✅\n\n\n"
)
