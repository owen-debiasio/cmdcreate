#!/usr/bin/env bash
# format.sh - Format all Rust source files in cmdcreate
#
# This script uses `cargo fmt` to format all Rust files.
# Run this before building or committing to ensure consistent code style.

set -e # Exit immediately on any error

cd ./

echo "Formatting source code..."
cargo fmt

# Format testing scripts
echo "Formatting testing scripts..."
black testing/features/*

echo "Formatting shell scripts..."
shfmt -w -i 4 ./
echo "Done formatting all source files."
