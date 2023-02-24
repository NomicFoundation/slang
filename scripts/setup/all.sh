#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# This is used to prepare the devcontainer after it has been created.
# It can also be invoked locally to setup all toolchains at once.

(
  "$REPO_ROOT/scripts/setup/cargo.sh"
  "$REPO_ROOT/scripts/setup/npm.sh"
  "$REPO_ROOT/scripts/setup/pipenv.sh"
)
