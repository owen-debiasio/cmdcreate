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

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
RESET='\033[0m'

if [[ $# -ne 1 ]]; then
    echo -e "\n${YELLOW}> Usage: $0 <version> (no leading v)${RESET}"
    exit 1
fi

VERSION="$1"
ARCHS=("x86_64" "i686" "aarch64" "armv7")

cd "$(dirname "$0")/.."

for ARCH in "${ARCHS[@]}"; do
    echo -e "${BLUE}> Building static binary for ${ARCH}...${RESET}"

    if [[ "$ARCH" == "armv7" ]]; then
        TARGET="armv7-unknown-linux-musleabihf"
    elif [[ "$ARCH" == "aarch64" ]]; then
        TARGET="aarch64-unknown-linux-musl"
    else
        TARGET="${ARCH}-unknown-linux-musl"
    fi

    BINARY_NAME="cmdcreate-v${VERSION}-linux-${ARCH}-bin"

    export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-fno-sanitize=all"
    export CFLAGS="-O3 -pipe"
    export CRATE_CC_NO_DEFAULTS=true

    cargo zigbuild --release --target "$TARGET"

    BUILD_PATH="target/${TARGET}/release/cmdcreate"

    if [[ -f "$BUILD_PATH" ]]; then
        cp "$BUILD_PATH" "$HOME/Downloads/${BINARY_NAME}"
        echo -e "${GREEN}> Successfully packaged ${ARCH}${RESET}"
    else
        echo -e "${RED}Error: Could not find binary at $BUILD_PATH${RESET}"
        exit 1
    fi
done

echo -e "\n${GREEN}> All binaries built and moved to ~/Downloads${RESET}"
