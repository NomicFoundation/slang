#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

function _list_active_targets() {
  if [[ "${SLANG_CROSS_BUILD:-}" == "true" ]]; then
    # Build all targets:
    jq --raw-output ".napi.triples.additional[]" "$PACKAGE_DIR/package.json"
  else
    # Just build the default target for local development:
    # "rustc -vV" will produce many key-value pairs, so we filter for "host".
    # Then "cut" will split by delimeter ' ', and take the second field, which is the target.
    #
    # host: aarch64-apple-darwin
    # └ 1 ┘ └─────── 2 ────────┘

    rustc -vV | grep '^host:' | cut -d ' ' -f 2
  fi
}

function _napi_build() {
  target="$1"

  printf "\n\n📚 Building for target: %s 📚\n\n\n" "$target"

  _group_output rustup target add "$target"

  if [[ "$target" == *"-pc-windows-msvc" ]]; then
    # TODO: move "cargo-xwin" to workspace dependencies, so that it is installed/upgraded with the rest of them.
    # https://github.com/rust-lang/cargo/issues/2267
    _group_output cargo install "cargo-xwin" --version "0.14.2"
  fi

  if [[ "$target" == *"-unknown-linux-musl" ]]; then
    # https://github.com/rust-lang/rust/pull/40113#issuecomment-323193341
    RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=-crt-static"
  fi

  export RUSTFLAGS

  # Navigate to where files should be generated:
  cd "$PACKAGE_DIR/src/generated"

  if [[ "${SLANG_CROSS_BUILD:-}" == "true" ]]; then
    extra_args="--release"
  fi

  _group_output \
    napi build --platform \
    --config "../../package.json" \
    --cargo-cwd "../../../crate" \
    --target "$target" \
    "${extra_args:-}"
}

function _process_source_files() {
  source_files=(
    "$PACKAGE_DIR/src/generated/index.d.ts"
    "$PACKAGE_DIR/src/generated/index.js"
  )

  # Add license headers to generated files, and format them using prettier:

  for file in "${source_files[@]}"; do
    contents=$(
      printf "%s\n%s\n\n%s" \
        "/* Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE */" \
        "/* NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE */" \
        "$(cat "$file")"
    )

    echo "$contents" > "$file"
    prettier --write "$file"
  done
}

function _process_bindings() {
  # Create and populate artifacts folder:

  mkdir -p "$PACKAGE_DIR/artifacts"
  artifacts="$(_list_source_files "$PACKAGE_DIR/src/generated" "*.node")"

  echo "$artifacts" | while read -r artifact; do
    cp "$artifact" "$PACKAGE_DIR/artifacts"
  done

  # Copy artifacts to platform-specific folders:

  cd "$PACKAGE_DIR"
  napi artifacts --dir "artifacts" --dist "platforms"
}

(
  _check_local_changes

  targets="$(_list_active_targets)"

  # Then build each target in turn:
  echo "$targets" | while read -r target; do
    _napi_build "$target"
  done

  printf "\n\n📚 Processing Generated Files: 📚\n\n\n"

  _process_source_files
  _process_bindings

  _check_local_changes
)
