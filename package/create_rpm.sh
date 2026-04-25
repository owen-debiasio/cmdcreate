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

BINARY_NAME="cmdcreate-v${VERSION}-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"
LICENSE_FILE="../LICENSE"

RPMTOP="$HOME/rpmbuild"
SPEC_FILE="$RPMTOP/SPECS/cmdcreate.spec"
SOURCE_FILE="cmdcreate-${VERSION}-linux-${ARCH}-bin"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "\n${RED}> Binary not found:${RESET} $BINARY_SRC"
    exit 1
fi

if [[ ! -f "$LICENSE_FILE" ]]; then
    echo -e "\n${RED}> License file not found:${RESET} $LICENSE_FILE"
    exit 1
fi

echo -e "${BLUE}> Packaging .rpm package...${RESET}"

mkdir -p "$RPMTOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

cp "$BINARY_SRC" "$RPMTOP/SOURCES/$SOURCE_FILE"
cp "$LICENSE_FILE" "$RPMTOP/SOURCES/LICENSE"

cat > "$SPEC_FILE" << EOF
Name:           cmdcreate
Version:        $VERSION
Release:        1%{?dist}
Summary:        Allows you to create custom commands for your custom scripts

License:        GPL-3.0-or-later
URL:            https://github.com/owen-debiasio/cmdcreate
Source0:        $SOURCE_FILE
Source1:        LICENSE

BuildArch:      x86_64
Requires:       curl, nano, git, less, openssl-libs, openssl-devel

%description
Allows you to create custom commands for your custom scripts.

%prep
cp %{SOURCE0} .
cp %{SOURCE1} .

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 %{SOURCE0} %{buildroot}%{_bindir}/cmdcreate

%files
%license LICENSE
%{_bindir}/cmdcreate

%changelog
* $(date +"%a %b %d %Y") Owen Debiasio <owen.debiasio@gmail.com> - $VERSION-1
- Initial RPM release
EOF

rpmbuild -bb "$SPEC_FILE"

RPM_FILE=$(find "$RPMTOP/RPMS/x86_64" -name "cmdcreate-${VERSION}-*.rpm" | head -n1)
FINAL_RPM="$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}.rpm"

cp "$RPM_FILE" "$FINAL_RPM"

rm -rf ~/rpmbuild/

echo -e "\n${GREEN}> Built and moved $FINAL_RPM to ~/Downloads${RESET}"
