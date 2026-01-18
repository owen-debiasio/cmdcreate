#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <version> (no leading v)"
    exit 1
fi

VERSION="$1"
ARCH="x86_64"

BINARY_NAME="cmdcreate-v${VERSION}-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"

RPMTOP="$HOME/rpmbuild"
SPEC_FILE="$RPMTOP/SPECS/cmdcreate.spec"
SOURCE_FILE="cmdcreate-${VERSION}-linux-${ARCH}-bin"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary not found: $BINARY_SRC"
    exit 1
fi

mkdir -p "$RPMTOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

cp "$BINARY_SRC" "$RPMTOP/SOURCES/$SOURCE_FILE"

cat >"$SPEC_FILE" <<EOF
Name:           cmdcreate
Version:        $VERSION
Release:        1%{?dist}
Summary:        Allows you to create custom commands for your custom scripts

License:        MIT
URL:            https://github.com/owen-debiasio/cmdcreate
Source0:        $SOURCE_FILE

BuildArch:      x86_64
Requires:       curl, nano, git, openssl-libs, openssl-devel

%description
Allows you to create custom commands for your custom scripts.

%prep
# nothing to prep

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 %{SOURCE0} %{buildroot}%{_bindir}/cmdcreate

%files
%{_bindir}/cmdcreate

%changelog
* $(date +"%a %b %d %Y") Owen Debiasio <owen.debiasio@gmail.com> - $VERSION-1
- Initial RPM release
EOF

rpmbuild -bb "$SPEC_FILE"

RPM_FILE=$(find "$RPMTOP/RPMS/x86_64" -name "cmdcreate-${VERSION}-*.rpm" | head -n1)
FINAL_RPM="$HOME/Downloads/cmdcreate-v${VERSION}-linux-${ARCH}.rpm"

cp "$RPM_FILE" "$FINAL_RPM"

echo -e "\nBuilt and moved $FINAL_RPM to ~/Downloads"
