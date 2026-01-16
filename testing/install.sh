#!/usr/bin/env bash

set -euo pipefail

BIN_NAME="cmdcreate"
TARGET="target/release/$BIN_NAME"
INSTALL_DIR="/usr/local/bin"

echo "Updating Rust toolchain..."
rustup update stable
rustup default stable

echo "Updating Cargo..."
cargo update

./package/format.sh

echo "Running clippy..."
cargo clippy --fix --allow-no-vcs

echo "Building release..."
cargo build --release

echo "Installing binary..."
sudo install -Dm755 "$TARGET" "$INSTALL_DIR/$BIN_NAME"

echo "Done. $BIN_NAME installed to $INSTALL_DIR"
