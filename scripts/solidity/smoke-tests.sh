#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

(
  cargo run --offline --bin "solidity_smoke_testing" -- "$@"
)
