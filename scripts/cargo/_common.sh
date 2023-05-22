#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# _CARGO_CLI_ENV_VARS_ (keep In Sync)
export CARGO="${REPO_ROOT}/bin/cargo"
export RUSTC="${REPO_ROOT}/bin/rustc"
export RUSTFMT="${REPO_ROOT}/bin/rustfmt"
export RUSTUP="${REPO_ROOT}/bin/rustup"
