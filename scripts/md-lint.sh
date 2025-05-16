#!/bin/bash

# Script to run markdownlint-cli2 with project configuration
# Usage: ./scripts/md-lint.sh [--fix] [path/to/target]

set -e

MARKDOWNLINT="npx markdownlint-cli2"
DEFAULT_GLOB="**/*.md"
EXCLUDE_GLOB="!node_modules/**"
CONFIG="--config .markdownlint-cli2.yaml"
FIX_FLAG=""

# Check for --fix flag
if [[ "$1" == "--fix" ]]; then
  FIX_FLAG="--fix"
  TARGET="$2"
else
  TARGET="$1"
fi

# If no target is specified, use the default glob
if [[ -z "$TARGET" ]]; then
  TARGET="$DEFAULT_GLOB"
fi

# Run markdownlint
if [[ "$FIX_FLAG" == "--fix" ]]; then
  echo "Running markdownlint-cli2 in fix mode on $TARGET..."
  npx markdownlint-cli2 "$TARGET" "$EXCLUDE_GLOB" $CONFIG $FIX_FLAG
else
  echo "Checking markdown files with markdownlint-cli2..."
  npx markdownlint-cli2 "$TARGET" "$EXCLUDE_GLOB" $CONFIG
fi