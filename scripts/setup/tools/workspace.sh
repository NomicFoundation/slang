#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

# If running in GitHub CI, mark the repository as a safe directory in git:
# See https://github.com/actions/checkout/issues/766
if [[ "${CI:-}" && "${GITHUB_WORKSPACE:-}" ]]; then
  (
    printf "\n\n ⚙️ Configuring GitHub Workspace ⚙️\n\n\n"

    cd "$REPO_ROOT"
    git config --global --add safe.directory "$GITHUB_WORKSPACE"

    printf "\n\n✅ GitHub Workspace Configured ✅\n\n\n"
  )
fi

# Warm up language server binaries and fetch any remote dependencies:
if [[ ! "${CI:-}" ]]; then
  (
    printf "\n\n📦 Installing Rust Analyzer 📦\n\n\n"

    cd "$REPO_ROOT"
    "$REPO_ROOT/bin/rust-analyzer" --version
    "$REPO_ROOT/bin/rust-src" --version

    printf "\n\n✅ Rust Analyzer Installed ✅\n\n\n"

  )
fi
