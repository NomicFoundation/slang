#!/bin/bash
set -euo pipefail

#
# Activate the Hermit environment:
#

{
  _repo_root="$(realpath "$(dirname "${BASH_SOURCE[0]}")/..")"

  # Check if another environment is already active:
  if [[ -n "${HERMIT_ENV:-}" ]]; then
    if [[ "${HERMIT_ENV}" == "${_repo_root}" ]]; then
      # Our repository. Do nothing.
      true
    fi

    # Not our repository. Deactivate it first:
    commands="$("${HERMIT_ENV}/bin/hermit" env --deactivate)"
    eval "${commands}"
  fi

  # Activate this repository's environment:
  commands="$("${_repo_root}/bin/hermit" env --activate)"
  eval "${commands}"
}

#
# Use the repository's Rust version:
#

{
  # Components included in the minimal profile:
  # - 'cargo'
  # - 'rust-std'
  # - 'rustc'
  #
  # Components only included in the default profile:
  # - 'clippy'
  # - 'rust-docs'
  # - 'rustfmt'
  #
  # No need to install 'rust-docs' (large offline copy). Add the rest manually:
  rustup install --no-self-update --profile "minimal" "${RUST_VERSION:?}"
  rustup component add "clippy" "rustfmt"

  # Make sure we chose the right version, if multiple toolchains are installed:
  rustup default "${RUST_VERSION:?}"
}
