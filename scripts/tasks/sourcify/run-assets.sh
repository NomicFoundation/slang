#!/bin/bash

set -euo pipefail

#
# Run every corpus asset listed on stdin (`solidity_testing_sourcify lock ...` output:
# name, sha256, bytes, contracts per line) through `run-corpus`, one at a time: download
# from the pinned release, verify, unpack, run, delete. One JSONL per asset lands in the
# results directory; extra arguments go to `run-corpus`.
#
# Usage: solidity_testing_sourcify lock pr | run-assets.sh <binary> <results-dir> [run-corpus flags...]
#

if [[ $# -lt 2 ]]; then
  echo >&2 "Usage: $(basename "${BASH_SOURCE[0]}") <binary> <results-dir> [run-corpus flags...]"
  exit 1
fi

binary="$1"
results="$2"
shift 2

release="$("${binary}" lock release)"
repo="${release%%$'\t'*}"
tag="${release##*$'\t'}"
mkdir -p "${results}"
work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT

status=0
while IFS=$'\t' read -r name sha256 _ _; do
  [[ -n "${name}" ]] || continue
  echo "== ${name}"
  gh release download "${tag}" --repo "${repo}" --pattern "${name}" --output "${work}/${name}"
  echo "${sha256}  ${work}/${name}" | sha256sum --check --quiet
  mkdir -p "${work}/corpus"
  tar -xJf "${work}/${name}" -C "${work}/corpus"
  rm "${work}/${name}"
  # Every asset gets its verdict; a failing one does not stop the others.
  "${binary}" run-corpus "${work}/corpus" --out "${results}/${name%.tar.xz}.jsonl" "$@" || status=1
  rm -rf "${work}/corpus"
done

exit "${status}"
