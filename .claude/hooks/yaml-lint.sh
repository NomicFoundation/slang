#!/bin/bash

# Pre-commit yamllint check.
# Runs as a PreToolUse hook on Bash — only triggers on git commit commands.

INPUT=$(cat)

# Only run on git commit commands
if ! echo "$INPUT" | grep -q '"git commit'; then
  exit 0
fi

# Ensure we're at the repo root for reliable paths
repo_root=$(git rev-parse --show-toplevel 2> /dev/null) || exit 0
cd "$repo_root" || exit 0

changed=$(git diff --cached --name-only 2> /dev/null | grep -E '\.(ya?ml)$')
if [[ -n "$changed" ]]; then
  output=$(./scripts/bin/infra lint yamllint 2>&1)
  if [[ $? -ne 0 ]]; then
    echo "yamllint failed — fix before committing:" >&2
    echo "$output" | grep -B1 '^\s\+[0-9]\+:[0-9]\+\s\+\(error\|warning\)' | grep -v '^--$' >&2
    exit 2
  fi
fi
