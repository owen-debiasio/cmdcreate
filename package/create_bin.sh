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
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

if [[ $# -ne 1 ]]; then
    echo -e "\n${YELLOW}> Usage: $0 <version> (no leading v)${RESET}"
    exit 1
fi

ARCH="x86_64"

BINARY_NAME="cmdcreate-v$1-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"

echo -e "${BLUE}> Building binary...${RESET}"

cargo update
rustup update

cargo build --release

echo -e "${BLUE}> Packaging binary...${RESET}"

cp ../target/release/cmdcreate ~/Downloads/

mv "$HOME/Downloads/cmdcreate" "$BINARY_SRC"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "${RED}Binary not found:${RESET} $BINARY_SRC"
    exit 1
fi

echo -e "\n${GREEN}> Packaged cmdcreate to:${RESET} $BINARY_NAME"
