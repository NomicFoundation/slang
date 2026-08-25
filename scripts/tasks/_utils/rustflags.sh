#!/bin/bash

set -euo pipefail

#
# Using `$RUSTFLAGS' or '--' overrides any rustflags from `.cargo/config.toml'.
# Using this syntax instead, as it is concatenated with the existing flags:
# __RUST_CI_FLAGS_OVERRIDES__ (keep in sync)
#

if [[ -z "${CI:-}" ]]; then
  exit 0
fi

# Deny any warnings, and lint against leftover 'dbg/todo!' macros:
readonly rustflags='["-Dwarnings", "-Wclippy::dbg_macro", "-Wclippy::todo"]'

# Rustdoc requires specifying rustdocflags, instead:
# See <https://github.com/rust-lang/cargo/issues/8424#issuecomment-1070988443>.
readonly rustdocflags='["-Dwarnings"]'

echo "--release --config 'build.rustflags = ${rustflags}' --config 'build.rustdocflags = ${rustdocflags}'"
