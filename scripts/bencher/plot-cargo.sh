#!/bin/bash

# plot-cargo.sh: creates one plot per project/file in projects.json, in the given bencher project, for the comparison
# of libraries.
#
# In essence, it helps to pick the right benchmarks, and solves the internal uuids use by bencher from the slug name
# of every part of it. Plots use the `main` branch and the `ci` testbed.
#
# usage: plot-cargo.sh project
#
# Example:
# $  ./scripts/bencher/plot-cargo.sh slang-dashboard-cargo-cmp

set -e
shopt -s inherit_errexit

if [[ $# -ne 1 ]]; then
  echo "Usage: plot-cargo.sh project"
  exit 1
fi

project=$1

source "$(dirname "$0")/_common.sh"

function get_benchmarks() {
  local search=$1
  details=$(bencher benchmark list --search "${search}" --per-page 255 "${project}")
  echo "${details}" | jq -r 'map("--benchmarks " + .uuid) | join(" ")'
}

branch_uuid=$(get_uuid branch main)
testbed_uuid=$(get_uuid testbed ci)
measure_uuid=$(get_uuid measure Instructions)

project_names=$(get_project_names)
mapfile -t names <<< "${project_names}"

for name in "${names[@]}"; do
  benchmarks_uuid=$(get_benchmarks "${name}")

  # Not every project is benchmarked in every bencher project (e.g. slang-v2 only covers 0.8.x projects),
  # and `bencher plot create` requires at least one benchmark.
  if [[ -z "${benchmarks_uuid}" ]]; then
    echo "No benchmarks matching '${name}' in ${project}. Skipping."
    continue
  fi

  # benchmarks_uuid is a list of options, so it shouldn't be quoted.
  # shellcheck disable=SC2086,SC2250
  (
    set -x
    bencher plot create --branches "${branch_uuid}" --measures "${measure_uuid}" --testbeds "${testbed_uuid}" --x-axis date_time --window 2419200 $benchmarks_uuid --title "${name}" "${project}"
  )
done
