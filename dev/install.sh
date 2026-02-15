#!/usr/bin/env bash

set -euo pipefail

BIN_NAME="cmdcreate"
TARGET="target/release/$BIN_NAME"
INSTALL_DIR="/usr/bin/"

echo -e "\n> Updating Rust toolchain..."
rustup update stable
rustup default stable

echo -e "\n> Updating Cargo..."
cargo update

./dev/format.sh

echo -e "\n> Running clippy..."
cargo clippy --fix --allow-no-vcs

echo -e "\n> Building release..."
cargo build --release

echo -e "\n> Installing binary..."
sudo install -Dm755 "$TARGET" "$INSTALL_DIR/$BIN_NAME"
sudo chmod +x $INSTALL_DIR/$BIN_NAME

echo -e "\n> Done. $BIN_NAME installed to $INSTALL_DIR"
