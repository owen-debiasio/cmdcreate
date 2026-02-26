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

if [[ $# -ne 1 ]]; then
    echo -e "\n> Usage: $0 <version> (no leading v)"
    exit 1
fi

ARCH="x86_64"

BINARY_NAME="cmdcreate-v$1-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"

cargo update
rustup update

cargo build --release

cp ../target/release/cmdcreate ~/Downloads/

mv "$HOME/Downloads/cmdcreate" "$BINARY_SRC"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary not found: $BINARY_SRC"
    exit 1
fi

echo -e "\n> Packaged cmdcreate to \"$BINARY_NAME\""
