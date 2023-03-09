#!/bin/bash
set -euo pipefail

# Import common utilities, after Hermit is activated:
source "$(dirname "${BASH_SOURCE[0]}")/_utils.sh"

# Activate the Hermit environment:
eval "$(_print_hermit_env)"
