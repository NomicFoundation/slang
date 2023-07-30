#!/bin/bash
set -euo pipefail

#
# Activate the Hermit environment:
#

{
  _repo_root="$(realpath "$(dirname "${BASH_SOURCE[0]}")/..")"

  # Check if another environment is already active:
  if [[ "${HERMIT_ENV:-}" ]]; then
    if [[ "$HERMIT_ENV" == "$_repo_root" ]]; then
      # Our repository. Do nothing.
      true
    fi

    # Not our repository. Deactivate it first:
    eval "$("$HERMIT_ENV/bin/hermit" env --deactivate)"
  fi

  # Activate this repository's environment:
  eval "$("$_repo_root/bin/hermit" env --activate)"
}

#
# Set up _CARGO_CLI_ENV_VARS_ (keep In Sync)
#

{
  export CARGO="${REPO_ROOT}/bin/cargo"
  export RUSTC="${REPO_ROOT}/bin/rustc"
  export RUSTFMT="${REPO_ROOT}/bin/rustfmt"
  export RUSTUP="${REPO_ROOT}/bin/rustup"
}

#
# Use the repository's Rust version:
#

{
  rustup default "$RUST_VERSION"
}
