#!/usr/bin/env bash

set -e

if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"

./package/format.sh

cd package

echo "Creating binary..."
./create_bin.sh "$VERSION"

echo "Creating Debian package..."
./create_deb.sh "$VERSION"

echo "Creating RPM package..."
./create_rpm.sh "$VERSION"

echo -e "\nAll release artifacts created successfully."
