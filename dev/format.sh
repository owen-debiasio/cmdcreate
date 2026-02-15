#!/usr/bin/env bash

set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT_DIR"

echo -e "\n> Formatting cmdcreate..."

command -v cargo > /dev/null || {
    echo -e "\n> cargo not found"
    exit 1
}
command -v black > /dev/null || {
    echo -e "\n> black not found"
    exit 1
}
command -v shfmt > /dev/null || {
    echo -e "\n> shfmt not found"
    exit 1
}
command -v shellcheck > /dev/null || {
    echo -e "\n> shellcheck not found"
    exit 1
}
command -v pylint > /dev/null || {
    echo -e "\n> pylint not found"
    exit 1
}
command -v markdownlint-cli2 > /dev/null || {
    echo -e "\n> markdownlint-cli2 not found"
    exit 1
}

echo -e "\n> Formatting Rust source..."
cargo fmt

if [[ -d "$ROOT_DIR/testing/" ]]; then
    echo -e "\n> Formatting Python testing scripts..."
    black "$ROOT_DIR/testing/"

    echo -e "\n> Linting Python testing scripts..."
    find . -name "*.py" -exec pylint {} +
else
    echo -e "\n> Skipping Python formatting (testing/features not found)"
fi

echo -e "\n> Linting shell scripts..."
find . -name "*.sh" -exec shellcheck {} +

echo -e "\n> Formatting shell scripts..."
shfmt -w -i 4 -ci -sr "$ROOT_DIR"

echo -e "\n> Linting markdown files..."
markdownlint-cli2 "**/*.md" --config .markdownlint.json

echo -e "\n> Formatting complete."
