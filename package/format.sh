#!/usr/bin/env bash

set -Eeuo pipefail

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

if [[ -d "../testing/features" ]]; then
    echo "Formatting Python testing scripts..."
    black "../testing/features"
else
    echo "Skipping Python formatting (testing/features not found)"
fi

echo "Formatting shell scripts..."
shfmt -w -i 4 ../

echo "Formatting complete."
