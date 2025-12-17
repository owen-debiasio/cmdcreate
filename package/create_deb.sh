#!/usr/bin/env bash
# package_deb.sh - Build a Debian (.deb) package for cmdcreate
#
# This script takes a pre-built cmdcreate binary, creates the necessary
# DEBIAN control files, and packages it as a .deb for installation.
# It also organizes the binary and package files in ~/Downloads for easy access.
#
# Usage:
#   ./package_deb.sh <version>
# Example:
#   ./package_deb.sh 0.7.1
#
# Note:
#   - The version argument MUST NOT start with 'v'.
#   - Make sure the binary exists in ~/Downloads before running.

set -e # Exit immediately on any error

# Check for version argument
if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"
PKGDIR="cmdcreate-${VERSION}-linux-x86_64-deb"
BINARY_SRC="$HOME/Downloads/cmdcreate-${VERSION}-linux-x86_64-bin"

# Ensure the binary exists
if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary $BINARY_SRC not found. Build it first."
    exit 1
fi

# Create package directory structure
mkdir -p "$PKGDIR/DEBIAN" "$PKGDIR/usr/bin"

# Generate the DEBIAN/control file with metadata
cat >"$PKGDIR/DEBIAN/control" <<EOF
Package: cmdcreate
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Owen Debiasio <owen.debiasio@gmail.com>
Depends: bash, curl, nano
Description: Allows you to create custom commands for your custom scripts
EOF

# Optional: run helper script to process the binary (e.g., formatting, validation)
./create_bin.sh "$VERSION"

# Copy the binary into the package structure and set permissions
cp "$BINARY_SRC" "$PKGDIR/usr/bin/cmdcreate"
chmod 755 "$PKGDIR/usr/bin/cmdcreate"

# Build the .deb package
dpkg-deb --build --root-owner-group "$PKGDIR"

# Final package filename
FINAL_DEB="cmdcreate-v${VERSION}-linux-x86_64.deb"

# Move .deb to Downloads
mv "${PKGDIR}.deb" "$FINAL_DEB"
cp "$FINAL_DEB" "$HOME/Downloads"

# Cleanup temporary packaging directory
rm -r "$PKGDIR"

# Move original binary to Downloads with versioned name
mv "$BINARY_SRC" "$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64-bin"

# Clean old package files in local package folder
rm -r cmdcreate*

# Final notification
echo -e "\nBuilt and moved $FINAL_DEB to ~/Downloads"
