#!/bin/bash
set -euo pipefail

#
# Wrap a command in a GitHub Actions log group when running in CI.
# Usage: _github_collapse_group "Group Title" <command> [args...]
#
_github_collapse_group() {
  local title="$1"
  shift

  [[ -n "${CI:-}" ]] && echo "::group::${title}"

  "$@"

  [[ -n "${CI:-}" ]] && echo "::endgroup::"
}

#
# Activate the Hermit environment:
#
_activate_hermit() {
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

_github_collapse_group "Initialize 'hermit' Workspace" \
  _activate_hermit

#
# This checks if the rust '$RUST_STABLE_VERSION' toolchain is already installed.
# If not, it will install the minimal profile of that toolchain.
# Any additional toolchains, or optional components, should be installed
# during 'infra setup cargo' step instead of here, as this is the hot path
# for every other command.
#
# See the comments here for more information:
# $REPO_ROOT/crates/infra/cli/src/commands/setup/cargo/mod.rs
#

if (cargo --version | grep -zq "${RUST_STABLE_VERSION:?}") > /dev/null 2>&1; then
  # Already installed. Do nothing.
  true
elif ! output=$(
  rustup install --no-self-update --profile "minimal" "${RUST_STABLE_VERSION:?}" \
    && rustup default "${RUST_STABLE_VERSION:?}" \
      2>&1
); then
  # Only print the output if the command failed:
  echo "Running 'rustup' failed:"
  echo >&2 "${output}"

  exit 1
fi
