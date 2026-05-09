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
    echo -e "\n${YELLOW}> Usage: $0 <version>${RESET}"
    exit 1
fi

VERSION="$1"
ARCHS=("x86_64" "i686")

cd "$(dirname "$0")/.."

for ARCH in "${ARCHS[@]}"; do
    BINARY_SRC="$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}-bin"

    if [[ ! -f "$BINARY_SRC" ]]; then
        echo -e "${RED}Error: Binary for $ARCH not found in Downloads.${RESET}"
        continue
    fi

    echo -e "${BLUE}> Packaging .rpm for ${ARCH}...${RESET}"

    RPMTOP="$(pwd)/rpmbuild_${ARCH}"
    mkdir -p "$RPMTOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    SPEC_FILE="$RPMTOP/SPECS/cmdcreate.spec"

    cat > "$SPEC_FILE" << EOF
%define _dbpath %{_topdir}/db
%define _binary_payload w9.gzdio
%define _build_id_links none
%define debug_package %{nil}
%define __strip /bin/true
%define _binaries_in_noarch_packages_terminate_build 0

Name:           cmdcreate
Version:        $VERSION
Release:        1
Summary:        Custom command creator
License:        GPL-3.0-or-later
BuildArch:      noarch

%description
Allows you to create custom commands for your custom scripts.

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 $BINARY_SRC %{buildroot}%{_bindir}/cmdcreate

%files
%{_bindir}/cmdcreate
EOF

    mkdir -p "$RPMTOP/db"
    rpmbuild -bb "$SPEC_FILE" --define "_topdir $RPMTOP" > /dev/null

    GENERATED_RPM=$(find "$RPMTOP/RPMS" -name "*.rpm" | head -n 1)

    if [[ -f "$GENERATED_RPM" ]]; then
        FINAL_NAME="$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}.rpm"
        cp "$GENERATED_RPM" "$FINAL_NAME"
        echo -e "${GREEN}> Successfully packaged ${ARCH} as ${FINAL_NAME}${RESET}"
    else
        echo -e "${RED}Error: RPM build failed for ${ARCH}${RESET}"
    fi

    rm -rf "$RPMTOP"
done

echo -e "\n${GREEN}> Built and moved .rpm packages to ~/Downloads${RESET}"
