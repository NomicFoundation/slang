#!/bin/bash

set -euo pipefail

#
# Print a NUL-separated list of the repository files matching the given 'git' pathspecs.
#
# Both tracked and untracked files are listed, while '.gitignore'-d paths, '.git'
# internals, and the contents of git submodules are not. Dotfiles participate like
# any other file, so pathspecs have to exclude them explicitly if unwanted.
#
# Only regular files are printed. Symbolic links are skipped, so that linters see
# each file once under its real path, rather than again through every alias.
#
# Usage:
#   list-files.sh ':(glob)**/*.md'
#   list-files.sh ':(glob)**/*.yml' ':(glob,exclude)pnpm-lock.yaml'
#

if [[ $# -eq 0 ]]; then
  echo >&2 "Usage: $(basename "${BASH_SOURCE[0]}") <pathspec>..."
  exit 1
fi

cd "${REPO_ROOT:?}"

git ls-files -z --cached --others --exclude-standard -- "$@" \
  | while IFS= read -r -d '' file; do
    if [[ ! -L "${file}" && -f "${file}" ]]; then
      printf '%s\0' "${file}"
    fi
  done
