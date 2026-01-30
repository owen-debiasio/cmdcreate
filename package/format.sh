#!/usr/bin/env bash

set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT_DIR"

echo "Formatting cmdcreate..."

command -v cargo >/dev/null || {
    echo "cargo not found"
    exit 1
}
command -v black >/dev/null || {
    echo "black not found"
    exit 1
}
command -v shfmt >/dev/null || {
    echo "shfmt not found"
    exit 1
}

echo "Formatting Rust source..."
cargo fmt

if [[ -d "$ROOT_DIR/testing/features" ]]; then
    echo "Formatting Python testing scripts..."
    black "$ROOT_DIR/testing/features"
else
    echo "Skipping Python formatting (testing/features not found)"
fi

echo "Linting shell scripts..."
find . -name "*.sh" -exec shellcheck {} +

echo "Formatting shell scripts..."
shfmt -w -i 4 "$ROOT_DIR"

echo "Formatting complete."
