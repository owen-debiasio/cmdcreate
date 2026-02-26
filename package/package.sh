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

set -Eeuo pipefail

die() {
    echo -e "\n> error: $*" >&2
    exit 1
}

[[ $# -eq 1 ]] || die "Provide package version (MUST NOT START WITH v)"
[[ "$1" != v* ]] || die "Version must NOT start with 'v'"

VERSION="$1"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

./dev/format.sh

echo -e "\n> Creating packages for version $VERSION..."

cd "$SCRIPT_DIR"

echo -e "\n> Creating binary..."
./create_bin.sh "$VERSION"

echo -e "\n> Creating Debian package..."
./create_deb.sh "$VERSION"

echo -e "\n> Creating RPM package..."
./create_rpm.sh "$VERSION"

cd ..

echo -e "\n> Cleaning up..."
./dev/clean.sh

echo -e "\n> release artifacts created successfully"
