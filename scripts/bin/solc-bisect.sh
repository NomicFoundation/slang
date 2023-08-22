#!/bin/bash
# Executes the provided command with each version of solc (installed by solc-select[1])
# until the command fails The first version that fails is reported to the user.
# [1]: https://github.com/crytic/solc-select

METHOD="bisect"
if [ "$1" = "scan" ] || [ "$1" = "bisect" ]; then
  METHOD="$1"
  shift # Remove the first argument
fi

# Ensure at least one command argument is provided after the optional method argument
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 [scan|bisect] <command> [args...]"
  exit 1
fi

bisect() {
  local low=0
  local high=$((${#VERSIONS[@]} - 1))
  local fail_version=""

  while [[ $low -le $high ]]; do
    local mid=$(((low + high) / 2))
    local version=${VERSIONS[mid]}
    echo ">>> Trying $mid solc $version:" >&2
    solc-select install "$version" 1> /dev/null

    if env SOLC_VERSION="$version" "$@"; then
      low=$((mid + 1))
    else
      fail_version="$version"
      high=$((mid - 1))
    fi
  done

  if [[ -n $fail_version ]]; then
    echo "Command first failed for version: $fail_version"
    return 1
  else
    echo "All versions completed without errors."
    return 0
  fi
}

linear_scan() {
  local index=$((${#VERSIONS[@]} - 1))
  local success_version=""

  while [[ $index -gt 0 ]]; do
    local version=${VERSIONS[index]}
    echo ">>> Trying solc $version:" >&2
    solc-select install "$version" 1> /dev/null

    if env SOLC_VERSION="$version" "$@"; then
      success_version="$version"
    fi

    index=$((index - 1))
  done

  if [[ -n $success_version ]]; then
    echo "Command first succeeded for version: $success_version"
    return 1
  else
    echo "All versions completed without errors."
    return 0
  fi
}

# Merge the "installable" and "installed" versions, as they're disjoint by solc-select
VERSIONS=(
  $(
    {
      solc-select install | tail -n +2
      solc-select versions | awk '/^[0-9]+\.[0-9]+\.[0-9]+$/ {print $0}'
    } \
      | sort -V | uniq
  )
)

if [ "$METHOD" = "scan" ]; then
  linear_scan "$@"
else
  bisect "$@"
fi
