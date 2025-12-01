#!/usr/bin/env bash
# build_install.sh - Build and install cmdcreate from source
#
# This script automates the process of updating Rust, formatting the code,
# building cmdcreate in release mode, running Clippy fixes, and installing
# the binary to /usr/bin.
#
# It is intended for development or testing purposes.

# Remove any previously installed cmdcreate package via pacman
# -Rns removes the package and dependencies that are not required by other packages
# --noconfirm skips confirmation prompts
sudo pacman -Rns cmdcreate --noconfirm || clear

# Notify user if release version was not installed
echo -e "\nRelease ver not installed, skipping\n"

# Update Rust toolchain to the latest version
rustup update

# Update Rust crate dependencies
cargo update

# Run formatting script (assumes you have ./format.sh)
./format.sh

# Build cmdcreate in release mode
cargo build --release

# Run Clippy to automatically apply fixable lints
# --allow-no-vcs allows running outside a git repository
cargo clippy --fix --allow-no-vcs

# Copy the release binary to /usr/bin so it is globally accessible
sudo cp ./target/release/cmdcreate /usr/bin
