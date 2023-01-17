#!/bin/bash
set -euo pipefail

# shellcheck source=/dev/null
source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# passed to mkdocs.yml !ENV
export DOCUMENTATION_SOURCE_FILES="$REPO_ROOT/documentation/docs"
export DOCUMENTATION_SITE_DIR="$REPO_ROOT/documentation/target/site"
