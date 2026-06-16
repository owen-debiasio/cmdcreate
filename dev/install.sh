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
INSTALL_DIR="/usr/bin/"

ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" ]]; then
    RUST_TARGET="x86_64-unknown-linux-musl"
    ZIG_TARGET="x86_64-linux-musl"
    CC_ENV_VAR="CC_x86_64_unknown_linux_musl"
elif [[ "$ARCH" == "i686" || "$ARCH" == "i386" ]]; then
    RUST_TARGET="i686-unknown-linux-musl"
    ZIG_TARGET="x86-linux-musl"
    CC_ENV_VAR="CC_i686_unknown_linux_musl"
elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
    RUST_TARGET="aarch64-unknown-linux-musl"
    ZIG_TARGET="aarch64-linux-musl"
    CC_ENV_VAR="CC_aarch64_unknown_linux_musl"
elif [[ "$ARCH" == "armv7l" || "$ARCH" == "armv7" ]]; then
    RUST_TARGET="armv7-unknown-linux-musleabihf"
    ZIG_TARGET="arm-linux-musleabihf"
    CC_ENV_VAR="CC_armv7_unknown_linux_musleabihf"
else
    echo "Error: Unsupported architecture $ARCH"
    exit 1
fi

TARGET_BIN="target/$RUST_TARGET/release/$BIN_NAME"

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

if [[ "${1:-}" = "--help" || "${1:-}" = "-h" ]]; then
    echo -ne "\n${YELLOW}> Help:\n\nPass \"-o\" or \"--offline\" flags to use offline${RESET}\n"
    exit 0
fi

if [[ "${1:-}" != "--offline" && "${1:-}" != "-o" ]]; then
    echo -e "\n${BLUE}> Updating Rust toolchain...${RESET}"
    rustup update stable
    rustup target add "$RUST_TARGET"

    echo -e "\n${BLUE}> Updating Cargo...${RESET}"
    cargo install cargo-zigbuild
    cargo update
fi

if [[ -f "./dev/format.sh" ]]; then
    ./dev/format.sh
fi

echo -e "\n${BLUE}> Running clippy ($ARCH)...${RESET}"
export CRATE_CC_NO_DEFAULTS=true
export "$CC_ENV_VAR"="zig cc -target $ZIG_TARGET -fno-sanitize=all"

cargo clippy --fix --allow-no-vcs --target "$RUST_TARGET"

echo -e "\n${BLUE}> Building release (Static Musl $ARCH)...${RESET}"
cargo zigbuild --release --target "$RUST_TARGET"

echo -e "\n${BLUE}> Installing binary...${RESET}"
sudo install -Dm755 "$TARGET_BIN" "$INSTALL_DIR/$BIN_NAME"

echo -e "\n${GREEN}> Done. $BIN_NAME (statically linked $ARCH) installed to $INSTALL_DIR${RESET}"
