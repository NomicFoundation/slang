#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../../_common.sh"

YAML_FILES=$(_list_source_files '**/*.yml')

(
  printf "\n\n🧪 yamllint 🧪\n\n\n"

  cd "$REPO_ROOT/scripts/lint"

  echo "$YAML_FILES"

  echo "$YAML_FILES" | xargs \
    python3 -m pipenv run yamllint \
    --strict \
    --config-file "$REPO_ROOT/.yamllint.yml"
)
