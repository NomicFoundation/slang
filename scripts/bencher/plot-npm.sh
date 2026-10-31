#!/bin/bash

# plot-npm.sh: creates one plot per project/file in projects.json, in the given bencher project, for comparing with
# npm libraries.
#
# In essence, it helps to pick the right benchmarks, and solves the internal uuids use by bencher from the slug name
# of every part of it. Plots use the `main` branch and the `ci` testbed.
#
# usage: plot-npm.sh project
#
# Example:
# $  ./scripts/bencher/plot-npm.sh slang-dashboard-npm

set -e
shopt -s inherit_errexit

if [[ $# -ne 1 ]]; then
  echo "Usage: plot-npm.sh project"
  exit 1
fi

project=$1

source "$(dirname "$0")/_common.sh"

function get_benchmarks() {
  local search=$1
  details=$(bencher benchmark list --search "${search}" --per-page 255 "${project}")
  # filter to let only the relevant benchmarks, leaving slang's parse_all_files and total
  echo "${details}" | jq -r '. - map(select(.name | contains("resolve_all_bindings"))) - map(select(.name | contains("init_bindings"))) | map("--benchmarks " + .uuid) | join(" ")'
}

branch_uuid=$(get_uuid branch main)
testbed_uuid=$(get_uuid testbed ci)
measure_uuid=$(get_uuid measure Duration)

project_names=$(get_project_names)
mapfile -t names <<< "${project_names}"

for name in "${names[@]}"; do
  benchmarks_uuid=$(get_benchmarks "${name}")

  # `bencher plot create` requires at least one benchmark.
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
