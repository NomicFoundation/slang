#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  # Setup the workspace first:
  _group_output "$REPO_ROOT/scripts/setup/workspace.sh"
)

(
  # Then setup all toolchains in order:
  _group_output "$REPO_ROOT/scripts/setup/cargo.sh"
  _group_output "$REPO_ROOT/scripts/setup/npm.sh"
  _group_output "$REPO_ROOT/scripts/setup/pipenv.sh"
)
