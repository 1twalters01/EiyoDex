#!/bin/bash

set -e  # Exit on error

VENV_DIR=".venv"
ACTIVATE_SCRIPT="$VENV_DIR/bin/activate"

if [ ! -d "$VENV_DIR" ]; then
  echo "No virtual environment found. Creating one..."
  uv venv
fi

if [ -f "$ACTIVATE_SCRIPT" ]; then
  echo "Activating virtual environment..."
  source "$ACTIVATE_SCRIPT"
else
  echo "Error: Failed to find activate script at $ACTIVATE_SCRIPT"
  exit 1
fi

echo "Installing packages from: ./python/..."

for dir in ./python/*/; do
  if [ -f "$dir/pyproject.toml" ]; then
    pkg_name=$(basename "$dir")
    echo "Installing $pkg_name..."
    uv pip install -e "$dir"
  else
    echo "Skipping: $(basename "$dir") (no pyproject.toml found)"
  fi
done

echo "All workspace packages have been installed."

