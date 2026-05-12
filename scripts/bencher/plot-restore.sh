#!/bin/bash

# plot-restore.sh: recreates plots in a bencher project from a backup
# JSON file produced by plot-clean.sh.
#
# The backup file is the JSON array returned by `bencher plot list` (one
# entry per plot). Each plot is recreated via `bencher plot create`, copying
# every plot field except the server-managed ones (uuid, created, modified).
# The destination project is read from the backup itself — bencher UUIDs
# (branches, testbeds, measures, benchmarks) are project-scoped, so a backup
# is only meaningful in the project it came from.
#
# Set DRY_RUN=1 to print the create commands instead of executing them.
#
# usage: plot-restore.sh backup-path
#
# Example:
# $  ./scripts/bencher/plot-restore.sh \
#       bencher-plots-slang-dashboard-cargo-slang-v2-20260508-093000.json

set -e
shopt -s inherit_errexit

if [[ $# -ne 1 ]]; then
  echo "Usage: plot-restore.sh backup-path"
  exit 1
fi

backup_path=$1

if [[ ! -f "${backup_path}" ]]; then
  echo "Backup file not found: ${backup_path}"
  exit 1
fi

project=$(jq -r '.[0].project // empty' "${backup_path}")
if [[ -z "${project}" ]]; then
  echo "Backup is empty or missing project metadata: ${backup_path}"
  exit 0
fi

total=$(jq 'length' "${backup_path}")
echo "Restoring ${total} plot(s) into ${project} from ${backup_path}"

# Build one `bencher plot create` invocation per plot. jq emits one CLI
# argument per line; mapfile loads them into a bash array so that values
# containing spaces (e.g. plot titles) are passed as single arguments.
for ((i = 0; i < total; i++)); do
  mapfile -t plot_args < <(
    # shellcheck disable=SC2312
    jq -r --argjson i "${i}" '
      .[$i] as $p
      | [
          (if $p.title != null   then "--title", $p.title  else empty end),
          (if $p.lower_value     then "--lower-value"      else empty end),
          (if $p.upper_value     then "--upper-value"      else empty end),
          (if $p.lower_boundary  then "--lower-boundary"   else empty end),
          (if $p.upper_boundary  then "--upper-boundary"   else empty end),
          "--x-axis", $p.x_axis,
          "--window", ($p.window | tostring),
          ($p.branches   | map("--branches",   .) | .[]),
          ($p.testbeds   | map("--testbeds",   .) | .[]),
          ($p.benchmarks | map("--benchmarks", .) | .[]),
          ($p.measures   | map("--measures",   .) | .[])
        ]
      | .[]
    ' "${backup_path}"
  )

  ${DRY_RUN:+echo} bencher plot create "${plot_args[@]}" "${project}"
done
