#!/usr/bin/env bash

set -e

if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"
PKGDIR="cmdcreate-${VERSION}-linux-x86_64-deb"
BINARY_SRC="$HOME/Downloads/cmdcreate-${VERSION}-linux-x86_64-bin"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary $BINARY_SRC not found. Build it first."
    exit 1
fi

mkdir -p "$PKGDIR/DEBIAN" "$PKGDIR/usr/bin"

cat >"$PKGDIR/DEBIAN/control" <<EOF
Package: cmdcreate
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Owen Debiasio <owen.debiasio@gmail.com>
Depends: curl, nano, libssl-dev
Description: Allows you to create custom commands for your custom scripts
EOF

./create_bin.sh "$VERSION"

cp "$BINARY_SRC" "$PKGDIR/usr/bin/cmdcreate"
chmod 755 "$PKGDIR/usr/bin/cmdcreate"

dpkg-deb --build --root-owner-group "$PKGDIR"

FINAL_DEB="cmdcreate-v${VERSION}-linux-x86_64.deb"

mv "${PKGDIR}.deb" "$FINAL_DEB"
cp "$FINAL_DEB" "$HOME/Downloads"

rm -r "$PKGDIR"

mv "$BINARY_SRC" "$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64-bin"

rm -r cmdcreate*

echo -e "\nBuilt and moved $FINAL_DEB to ~/Downloads"
