#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  # Setup the workspace first:
  _group_output "$REPO_ROOT/scripts/setup/tools/workspace.sh"
)

(
  # Then setup all toolchains in order:
  _group_output "$REPO_ROOT/scripts/setup/tools/cargo.sh"
  _group_output "$REPO_ROOT/scripts/setup/tools/npm.sh"
  _group_output "$REPO_ROOT/scripts/setup/tools/pipenv.sh"
)
