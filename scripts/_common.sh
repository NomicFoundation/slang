#!/bin/bash
set -euo pipefail

# See https://github.com/actions/checkout/issues/766
[[ "${GITHUB_WORKSPACE:-}" ]] && git config --global --add safe.directory "$GITHUB_WORKSPACE"

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" ]] || source "$(dirname "${BASH_SOURCE[0]}")/../bin/activate-hermit"

#
# Searches the repository for all files matching the passed globs:
# - Globs should be relative to "$REPO_ROOT".
# - Results are canonicalized (converted to full paths).
# - It also excludes files hidden by ".gitignore".
#
function _list_source_files() {
  pattern="$1"

  cd "$REPO_ROOT"
  rg --files --hidden --glob '!.git/**' --glob "$pattern" | xargs realpath --canonicalize-existing
}
