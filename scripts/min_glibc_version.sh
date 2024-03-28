#!/bin/bash

# This scripts determines the minimum GLIBC version required for a dynamic library.

set -e -o pipefail

# List the dynamic symbols of the library
# Note: `ldd` does not work reliably when inspecting cross-compiled ARM binaries on x86_64
objdump -T "$1" \
  |
  # Select only the GLIBC symbols
  grep -o '(GLIBC_[0-9][0-9]*.*)' \
  |
  # Version-sort the unique symbols, descending
  sort -r -V | uniq \
  |
  # Print out just the version number
  sed -n 's/.*GLIBC_\([0-9.]*\)).*/\1/p' \
  |
  # Finally, print out the highest version
  head -n 1
