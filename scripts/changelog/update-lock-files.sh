#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

if [[ ! "${GITHUB_ACTIONS:-}" ]]; then
  printf "\n\n❌ Error: Script must be invoked by GitHub Actions in CI ❌\n\n\n"
  exit 1
fi

(
  printf "\n\n📜 Updating Lock Files 📜\n\n\n"

  _check_local_changes

  base_branch=$(git branch --show-current)

  _group_output npm install
  _group_output git diff

  cd "$REPO_ROOT"
  changed_files=$(git status --short)

  if [[ "$changed_files" == "" ]]; then
    echo "No files changed. Exiting early."
    exit 0
  fi

  if [[ "$changed_files" != " M package-lock.json" ]]; then
    echo "Unexpected changes to files:"
    echo "$changed_files"
    exit 1
  fi

  if [[ "${SLANG_PUBLISH:-}" != "true" ]]; then
    echo "Dry run mode. Exiting early."
    _group_output git checkout "package-lock.json"
    exit 0
  fi

  head_branch="release/update-lock-files"

  _group_output git checkout -B "$head_branch"
  _group_output git add "package-lock.json"
  _group_output git commit --message "update lock files after release"

  _group_output git push --force --set-upstream "origin" "$head_branch"
  _group_output gh pr create --fill --base "$base_branch" --head "$head_branch"

  _group_output git checkout "$base_branch"

  printf "\n\n✅ Lock Files Updated ✅\n\n\n"
)
