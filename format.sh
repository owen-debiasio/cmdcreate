#!/usr/bin/env bash
# format.sh - Format all Rust source files in cmdcreate
#
# This script uses `rustfmt` to format all Rust files in the `src` directory,
# including main.rs, command modules, backup modules, and utility modules.
# Run this before building or committing to ensure consistent code style.

set -e  # Exit immediately on any error

# Enter the source directory
cd ./src || { echo "Failed to enter ./src directory"; exit 1; }

# Format the main file
echo "Formatting main..."
rustfmt main.rs

# Format core command modules
echo "Formatting commands..."
rustfmt ./cmds/*.rs

# Format backup submodules
rustfmt ./cmds/backup/*.rs

# Format utility modules
echo "Formatting utils..."
rustfmt ./utils/*.rs

# Format testing scripts
echo "Formatting tests..."
black ../testing/features/*

echo "Done formatting all source files."
