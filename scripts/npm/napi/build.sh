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

  # Navigate to where files should be generated:
  cd "$PACKAGE_DIR/src/generated"

  command=(
    napi build
    --platform
    --config "../../package.json"
    --cargo-cwd "../../../crate"
    --target "$target"
    --no-const-enum
  )

  if [[ "${SLANG_CROSS_BUILD:-}" == "true" ]]; then
    command+=(--release)
  fi

  _group_output "${command[@]}"
}

function _process_generated_files() {
  printf "\n\n📚 Processing Generated Files: 📚\n\n\n"

  generated_files=(
    "$PACKAGE_DIR/src/generated/index.d.ts"
    "$PACKAGE_DIR/src/generated/index.js"
  )

  for file in "${generated_files[@]}"; do
    contents=$(
      echo "// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE"
      echo "// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE"
      echo ""
      echo "// @ts-nocheck"
      echo ""
      cat "$file"
    )

    echo "$contents" > "$file"
    prettier --write "$file"
  done
}

function _process_bindings() {
  printf "\n\n📚 Processing Bindings: 📚\n\n\n"

  # Populate artifacts folder with all binding files:

  mkdir -p "$PACKAGE_DIR/artifacts"
  artifacts="$(_list_source_files "$PACKAGE_DIR/src/generated" "*.node")"

  echo "$artifacts" | while read -r artifact; do
    cp "$artifact" "$PACKAGE_DIR/artifacts"
  done

  # Copy each binding file to its platform-specific sub-folder:

  cd "$PACKAGE_DIR"
  napi artifacts --dir "artifacts" --dist "platforms"
}

(
  _check_local_changes

  _list_active_targets | while read -r target; do
    _napi_build "$target"
  done

  _process_generated_files
  _process_bindings

  _check_local_changes
)
