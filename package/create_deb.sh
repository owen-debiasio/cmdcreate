#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <version> (no leading v)"
    exit 1
fi

VERSION="$1"
ARCH="x86_64"
PKGDIR="cmdcreate-${VERSION}-linux-${ARCH}-deb"
BINARY_NAME="cmdcreate-${VERSION}-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"

cleanup() {
    rm -rf "$PKGDIR"
}
trap cleanup EXIT

if [[ ! -x ./create_bin.sh ]]; then
    echo "create_bin.sh missing or not executable"
    exit 1
fi

./create_bin.sh "$VERSION"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary $BINARY_SRC not found. Build failed?"
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
Depends: curl, nano, git, libssl3 | libssl1.1
Description: Allows you to create custom commands for your custom scripts
EOF

install -m755 "$BINARY_SRC" "$PKGDIR/usr/bin/cmdcreate"

dpkg-deb --build --root-owner-group "$PKGDIR"

FINAL_DEB="cmdcreate-v${VERSION}-linux-${ARCH}.deb"

mv "${PKGDIR}.deb" "$FINAL_DEB"
mv "$FINAL_DEB" "$HOME/Downloads/"
mv "$BINARY_SRC" "$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64-bin"

echo -e "\nBuilt and moved $FINAL_DEB to ~/Downloads"
