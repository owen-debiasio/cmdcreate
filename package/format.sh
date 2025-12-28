#!/usr/bin/env bash

set -e

cd ./

echo "Formatting source code..."
cargo fmt

# Format testing scripts
echo "Formatting testing scripts..."
black testing/features/*

echo "Formatting shell scripts..."
shfmt -w -i 4 ./
echo "Done formatting all source files."
