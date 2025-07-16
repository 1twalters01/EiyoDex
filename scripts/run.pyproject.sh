#!/usr/bin/env bash

# Usage: bash ./scripts/run.pyproject.sh utils
# This dynamically sets PYTHONPATH and runs the main module for the given uv project

PROJECT="$1"
SRC_DIR="python/$PROJECT/src"

if [ ! -d "$SRC_DIR" ]; then
  echo "Error: Source directory $SRC_DIR does not exist"
  exit 1
fi

PYTHONPATH="$SRC_DIR" uv run --package "$PROJECT" -- python -m "$PROJECT.main"

