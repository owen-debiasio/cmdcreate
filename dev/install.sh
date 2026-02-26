#!/bin/bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Owen Debiasio
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

set -euo pipefail

BIN_NAME="cmdcreate"
TARGET="target/release/$BIN_NAME"
INSTALL_DIR="/usr/bin/"

if [[ "${1:-}" = "--help" && "${1:-}" != "-h" ]]; then
    echo -ne "\n> Help:\n\nPass \"-o\" or \"--offline\" flags to use offline\n"
    exit 1
fi

if [[ "${1:-}" != "--offline" && "${1:-}" != "-o" ]]; then
    echo -e "\n> Updating Rust toolchain..."
    rustup update stable
    rustup default stable

    echo -e "\n> Updating Cargo..."
    cargo update
fi

./dev/format.sh

echo -e "\n> Running clippy..."
cargo clippy --fix --allow-no-vcs

echo -e "\n> Building release..."
cargo build --release

echo -e "\n> Installing binary..."
sudo install -Dm755 "$TARGET" "$INSTALL_DIR/$BIN_NAME"
sudo chmod +x $INSTALL_DIR/$BIN_NAME

echo -e "\n> Done. $BIN_NAME installed to $INSTALL_DIR"
#!/usr/bin/env bash

set -euo pipefail

BIN_NAME="cmdcreate"
TARGET="target/release/$BIN_NAME"
INSTALL_DIR="/usr/bin/"
