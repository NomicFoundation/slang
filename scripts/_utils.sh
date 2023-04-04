#!/bin/bash
set -euo pipefail

#
# Prints necessary commands needed to activate the Hermit environment:
#
function _print_hermit_env() {
  _repo_root="$(realpath "$(dirname "${BASH_SOURCE[0]}")/..")"

  # Check if another environment is already active:
  if [[ "${HERMIT_ENV:-}" ]]; then
    if [[ "$HERMIT_ENV" == "$_repo_root" ]]; then
      # Our repository. Do nothing.
      return
    fi

    # Not our repository. Deactivate it first.
    "$HERMIT_ENV/bin/hermit" env --deactivate
  fi

  # Activate this repository's environment.
  "$_repo_root/bin/hermit" env --activate
}

#
# GitHub Actions Output Group:
#
function _group_output() {
  if [[ ! "${GITHUB_ACTIONS:-}" ]]; then
    "$@"
    return 0
  fi

  echo "::group::$*"
  echo ""
  "$@"
  echo ""
  echo "::endgroup::"
}

#
# Searches the repository for all files matching the passed globs:
# - Globs should be relative to "$REPO_ROOT".
# - Results are canonicalized (converted to full paths).
# - It also excludes files hidden by ".gitignore".
#
function _list_source_files() {
  pattern="$1"

  cd "$REPO_ROOT"
  rg \
    --files --sort "path" \
    --hidden --glob '!.git/**' --glob '!.hermit/**' \
    --glob "$pattern" \
    | xargs realpath --canonicalize-existing
}
