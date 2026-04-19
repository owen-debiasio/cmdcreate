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
ARCH="x86_64"
PKGDIR="cmdcreate-${VERSION}-linux-${ARCH}-deb"
BINARY_NAME="cmdcreate-v${VERSION}-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"
LICENSE_FILE="../LICENSE"

cleanup() {
    rm -rf "$PKGDIR"
}
trap cleanup EXIT

if [[ ! -x ./create_bin.sh ]]; then
    echo -e "\n${RED}> create_bin.sh missing or not executable.${RESET}"
    exit 1
fi

if [[ ! -f "$LICENSE_FILE" ]]; then
    echo -e "\n${RED}> License file not found: ${RESET}$LICENSE_FILE"
    exit 1
fi

./create_bin.sh "$VERSION"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "\n${RED}> Binary $BINARY_SRC not found. Build failed?${RESET}"
    exit 1
fi

echo -e "${BLUE}> Packaging .deb package...${RESET}"

mkdir -p "$PKGDIR/DEBIAN" "$PKGDIR/usr/bin" "$PKGDIR/usr/share/doc/cmdcreate"

cat > "$PKGDIR/DEBIAN/control" << EOF
Package: cmdcreate
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Owen Debiasio <owen.debiasio@gmail.com>
Depends: curl, nano, git, build-essential, pkg-config, libssl-dev, libssl3 | libssl1.1
Description: Allows you to create custom commands for your custom scripts
EOF

install -m755 "$BINARY_SRC" "$PKGDIR/usr/bin/cmdcreate"
install -m644 "$LICENSE_FILE" "$PKGDIR/usr/share/doc/cmdcreate/copyright"

dpkg-deb --build --root-owner-group "$PKGDIR"

FINAL_DEB="cmdcreate-v${VERSION}-linux-${ARCH}.deb"

mv "${PKGDIR}.deb" "$FINAL_DEB"
mv "$FINAL_DEB" "$HOME/Downloads/"

echo -e "\n${GREEN}> Built and moved $FINAL_DEB to ~/Downloads${RESET}"
