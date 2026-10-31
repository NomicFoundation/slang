#!/bin/bash

# Common functions to obtain uuids from names in Bencher
# They expect the $project variable to be set with the name of the bencher project

# Returns the uuid of the given branch/testbed/measure: get_uuid kind name
function get_uuid() {
  local kind=$1
  local name=$2
  # shellcheck disable=SC2154
  details=$(bencher "${kind}" list "${project}" --name "${name}")
  echo "${details}" | jq -r ".[].uuid"
}

# Returns the names of all projects and files benchmarked in projects.json
function get_project_names() {
  jq -r "[.projects, .files] | add | map(.name) | .[]" "$(dirname "${BASH_SOURCE[0]}")/../../crates/solidity/testing/perf/projects.json"
}
