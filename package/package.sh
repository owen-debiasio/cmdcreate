#!/usr/bin/env bash

set -Eeuo pipefail

die() {
    echo -e "\n> error: $*" >&2
    exit 1
}

[[ $# -eq 1 ]] || die "Provide package version (MUST NOT START WITH v)"
[[ "$1" != v* ]] || die "Version must NOT start with 'v'"

VERSION="$1"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

./dev/format.sh

echo -e "\n> Creating packages for version $VERSION..."

cd "$SCRIPT_DIR"

echo -e "\n> Creating binary..."
./create_bin.sh "$VERSION"

echo -e "\n> Creating Debian package..."
./create_deb.sh "$VERSION"

echo -e "\n> Creating RPM package..."
./create_rpm.sh "$VERSION"

cd ..

echo -e "\n> Cleaning up..."
./dev/clean.sh

echo -e "\n> release artifacts created successfully"
