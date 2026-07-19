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

VERSION="$1"
ARCHS=("x86_64" "i686" "aarch64" "armv7")
LICENSE_FILE="../LICENSE"

cd "$(dirname "$0")"

if [[ ! -x ./create_bin.sh ]]; then
    echo -e "\n${RED}> create_bin.sh missing or not executable.${RESET}"
    exit 1
fi

./create_bin.sh "$VERSION"

echo -e "${BLUE}> Packaging .deb packages...${RESET}"

for ARCH in "${ARCHS[@]}"; do
    DEB_ARCH="amd64"
    if [[ "$ARCH" == "i686" ]]; then
        DEB_ARCH="i386"
    elif [[ "$ARCH" == "aarch64" ]]; then
        DEB_ARCH="arm64"
    elif [[ "$ARCH" == "armv7" ]]; then
        DEB_ARCH="armhf"
    fi

    PKGDIR="cmdcreate-${VERSION}-linux-${ARCH}-deb"
    BINARY_SRC="$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}-bin"

    if [[ ! -f "$BINARY_SRC" ]]; then
        echo -e "${RED}> Binary $BINARY_SRC not found. Skipping $ARCH.${RESET}"
        continue
    fi

    echo -e "${BLUE}> Creating .deb for ${ARCH}...${RESET}"

    mkdir -p "$PKGDIR/DEBIAN" "$PKGDIR/usr/bin" "$PKGDIR/usr/share/doc/cmdcreate"

    cat > "$PKGDIR/DEBIAN/control" << EOF
Package: cmdcreate
Version: $VERSION
Section: utils
Priority: optional
Architecture: $DEB_ARCH
Maintainer: Owen Debiasio <owen.debiasio@gmail.com>
Description: Create your own custom commands for your convenience
EOF

    cp "$BINARY_SRC" "$PKGDIR/usr/bin/cmdcreate"

    if [[ -f "$LICENSE_FILE" ]]; then
        cp "$LICENSE_FILE" "$PKGDIR/usr/share/doc/cmdcreate/copyright"
    fi

    chmod 755 "$PKGDIR/usr/bin/cmdcreate"

    dpkg-deb --build --root-owner-group "$PKGDIR" "$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}.deb" > /dev/null

    rm -rf "$PKGDIR"
done

echo -e "\n${GREEN}> Built and moved .deb packages to ~/Downloads${RESET}"
