#!/bin/bash

set -euo pipefail

#
# Run the given command over the NUL-separated list of files read from stdin,
# split into batches that stay well under the maximum command line length, and
# fanned out over all available cores.
#
# Usage: list-files.sh ':(glob)**/*.md' | xargs-batch.sh markdownlint --dot
#

readonly BATCH_SIZE=50

if [[ $# -eq 0 ]]; then
  echo >&2 "Usage: $(basename "${BASH_SOURCE[0]}") <command> [args...]"
  exit 1
fi

parallelism="$(getconf _NPROCESSORS_ONLN)"
readonly parallelism

# '-r' is a no-op on BSD 'xargs', which already skips running on empty input:
exec xargs -0 -r -n "${BATCH_SIZE}" -P "${parallelism}" "$@"
