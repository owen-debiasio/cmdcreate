#!/usr/bin/env bash

set -Eeuo pipefail

die() {
    echo "error: $*" >&2
    exit 1
}

[[ $# -eq 1 ]] || die "Provide cmdcreate's version (MUST NOT START WITH v)"
[[ "$1" != v* ]] || die "Version must NOT start with 'v'"

VERSION="$1"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

cd "$SCRIPT_DIR/package"

echo "Formatting code..."
./format.sh

echo "Creating binary ($VERSION)..."
./create_bin.sh "$VERSION"

echo "Creating Debian package..."
./create_deb.sh "$VERSION"

echo "Creating RPM package..."
./create_rpm.sh "$VERSION"

echo
echo "All release artifacts created successfully"