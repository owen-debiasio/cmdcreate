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

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

if [[ "${1:-}" = "--help" && "${1:-}" != "-h" ]]; then
    echo -ne "\n${YELLOW}> Help:\n\nPass \"-o\" or \"--offline\" flags to use offline${RESET}\n"
    exit 1
fi

if [[ "${1:-}" != "--offline" && "${1:-}" != "-o" ]]; then
    echo -e "\n${BLUE}> Updating Rust toolchain...${RESET}"
    rustup update stable
    rustup default stable

    echo -e "\n${BLUE}> Updating Cargo...${RESET}"
    cargo update
fi

./dev/format.sh

echo -e "\n${BLUE}> Running clippy...${RESET}"
cargo clippy --fix --allow-no-vcs

echo -e "\n${BLUE}> Building release...${RESET}"
cargo build --release

echo -e "\n${BLUE}> Installing binary...${RESET}"
sudo install -Dm755 "$TARGET" "$INSTALL_DIR/$BIN_NAME"
sudo chmod +x $INSTALL_DIR/$BIN_NAME

echo -e "\n${GREEN}> Done. $BIN_NAME installed to $INSTALL_DIR${RESET}"
