#!/bin/bash

# check-base-report.sh: finds the base branch commit that a PR benchmark has to be compared against, and checks
# that bencher already has a report for it, so it can be passed to 'bencher run --start-point-hash'.
#
# Without it, bencher compares against the base branch's latest report at upload time, which is often not the
# commit the PR was merged with: any newer (or missing) base branch commits would be attributed to the PR. And
# bencher silently falls back to that latest report when the start point hash has none, hence this check.
#
# usage: check-base-report.sh infra-command [infra-suite]

set -euo pipefail

infra_command="$1"
infra_suite="${2:-}"

# The bencher project that 'infra perf <command> <suite>' uploads to.
# __SLANG_INFRA_BENCHER_PROJECTS__ (keep in sync)
case "${infra_command} ${infra_suite}" in
  "cargo comparison") project="slang-dashboard-cargo-cmp" ;;
  "cargo slang") project="slang-dashboard-cargo-slang" ;;
  "cargo slang-v2") project="slang-dashboard-cargo-slang-v2" ;;
  "npm ") project="slang-dashboard-npm" ;;
  *)
    echo "::error::No bencher project is known for 'infra perf ${infra_command} ${infra_suite}'."
    exit 1
    ;;
esac

testbed="ci"
api="https://api.bencher.dev/v0/projects/${project}"

base_ref="${GITHUB_BASE_REF:?}"
repository="${GITHUB_REPOSITORY:?}"
server_url="${GITHUB_SERVER_URL:?}"
output="${GITHUB_OUTPUT:?}"
pr_head_hash="${BENCHER_PR_HEAD_HASH:?}"

function fail() {
  echo "::error::$1"
  exit 1
}

# The test merge commit's first parent is the base branch commit, and its second one the PR head:
merged_hash="$(git rev-parse --verify --quiet "HEAD^2" || true)"
if [[ "${merged_hash}" != "${pr_head_hash}" ]]; then
  fail "Expected HEAD to be GitHub's test merge commit of the PR head ${pr_head_hash}, but its second parent is '${merged_hash}'."
fi

base_hash="$(git rev-parse "HEAD^1")"
echo "Comparing against '${base_ref}' at ${base_hash}."

# The base commit's checks show its benchmark run, whether it is still running, failed, or was never started (e.g. a
# base PR without the 'ci:perf' label).
missing="'${base_ref}' has no '${testbed}' benchmark report on bencher for ${base_hash}, which this PR was merged with. \
Wait for (or re-run) its benchmark at ${server_url}/${repository}/commit/${base_hash}, or rebase onto a benchmarked \
commit, then re-run this job."

# The projects are public, so their reports can be listed without an API key. They are listed oldest first starting
# from the base hash commit time, so the base commit's report is close to the top. 255 is the largest page.
# An unknown branch gives an empty list.
start_time=$(($(git show --no-patch --format=%ct "${base_hash}") - 24 * 60 * 60))
if ! reports="$(curl --silent --show-error --fail --retry 3 --get \
  --data-urlencode "branch=${base_ref}" \
  --data-urlencode "testbed=${testbed}" \
  --data "start_time=$((start_time * 1000))" \
  --data "per_page=255" \
  --data "sort=date_time" \
  --data "direction=asc" \
  "${api}/reports")"; then
  fail "Couldn't list the '${base_ref}' reports of '${project}' on bencher. Re-run this job later."
fi

# Every report listed is on the base branch, so any of them gives its uuid. None means it has no reports (or doesn't
# exist on bencher at all).
branch_uuid="$(jq --raw-output '.[0].branch.uuid // empty' <<< "${reports}")"
if [[ -z "${branch_uuid}" ]]; then
  fail "${missing}"
fi

# Bencher resolves the start point hash only among the versions of the base branch's current head. A Bencher head
# is a group of reports. `main` has a single head to which we add reports, but PR branches have a new head for each
# benchmark run ('--start-point-reset').
#
# There's a race condition here, the base branch can get a new head between this check and the upload, particularly
# on stacked PRs. We can't avoid this from our side, but the risk is small and is only problematic on non rebased
# stacked PRs.
if ! head_uuid="$(curl --silent --show-error --fail --retry 3 "${api}/branches/${branch_uuid}" | jq --raw-output '.head.uuid')"; then
  fail "Couldn't read the '${base_ref}' branch of '${project}' on bencher. Re-run this job later."
fi

# Look for a report within `reports` that belongs to the current head (`head_uuid`) and has the base commit hash.
if ! jq --exit-status --arg hash "${base_hash}" --arg head "${head_uuid}" \
  'any(.[]; .branch.head.version.hash == $hash and .branch.head.uuid == $head)' <<< "${reports}" > /dev/null; then
  fail "${missing}"
fi

echo "Found the '${base_ref}' report for ${base_hash}."
echo "hash=${base_hash}" >> "${output}"
