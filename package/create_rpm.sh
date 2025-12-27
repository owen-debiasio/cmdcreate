#!/usr/bin/env bash

set -e

if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"
BINARY_SRC="$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64-bin"
RPMBUILD_DIR="$HOME/rpmbuild"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary $BINARY_SRC not found. Build it first."
    exit 1
fi

mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

SPEC_FILE="$RPMBUILD_DIR/SPECS/cmdcreate.spec"

cat >"$SPEC_FILE" <<EOF
Name:           cmdcreate
Version:        $VERSION
Release:        1%{?dist}
Summary:        Allows you to create custom commands for your custom scripts

License:        MIT
URL:            https://github.com/owen-debiasio/cmdcreate
Source0:        %{name}-%{version}-linux-x86_64-bin

BuildArch:      x86_64
Requires:       curl, nano

%description
Allows you to create custom commands for your custom scripts

%prep

%install
mkdir -p %{buildroot}/usr/bin
cp %{SOURCE0} %{buildroot}/usr/bin/cmdcreate
chmod 755 %{buildroot}/usr/bin/cmdcreate

%files
/usr/bin/cmdcreate

%changelog
* $(date +"%a %b %d %Y") Owen Debiasio <owen.debiasio@gmail.com> - $VERSION-1
- Initial RPM release
EOF

cp "$BINARY_SRC" "$RPMBUILD_DIR/SOURCES/cmdcreate-${VERSION}-linux-x86_64-bin"

rpmbuild -bb "$SPEC_FILE"

RPM_FILE=$(find "$RPMBUILD_DIR/RPMS/x86_64" -name "cmdcreate-${VERSION}-*.rpm" | head -n1)
FINAL_RPM="$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64.rpm"
cp "$RPM_FILE" "$FINAL_RPM"

rm -r "$RPMBUILD_DIR"

echo -e "\nBuilt and moved $FINAL_RPM to ~/Downloads"
