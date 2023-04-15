#!/bin/bash
set -euo pipefail

# Import common utilities, before Hermit is activated:
source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

# NPM scripts always run from the package directory:
PACKAGE_DIR="$(pwd)"

if [[ $(jq 'has("napi")' "$PACKAGE_DIR/package.json") != "true" ]]; then
  printf "\n\n❌ Error: Script must be invoked by an NPM script from the NAPI package directory ❌\n\n\n"
  exit 1
fi
