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
  parent_dir="$1"
  pattern="$2"

  cd "$parent_dir"
  rg \
    --files --sort "path" \
    --hidden --glob '!.git/**' --glob '!.hermit/**' \
    --glob "$pattern" \
    | xargs realpath --canonicalize-existing
}

#
# Unlike our Cargo codegen utils, other build scripts don't have the
# ability to check whether files are up to date or not when running in CI.
# So, we make sure there are no local changes before or after the build.
#
function _check_local_changes() {
  if [[ "${CI:-}" && -n "$(git status --short)" ]]; then
    git diff
    git diff --cached
    printf "\n\n❌ Found local changes. Aborting. ❌\n\n\n"
    exit 1
  fi
}

function _slang_repo_version() {
  jq --raw-output '.version' "$REPO_ROOT/scripts/changelog/package.json"
}
