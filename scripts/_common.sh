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
  rustup default "${RUST_VERSION:?}"
}
