#!/usr/bin/env bash
# package.sh - Automates the full release workflow for cmdcreate
#
# This script formats the code, builds the binary, and packages cmdcreate
# for multiple distributions (.deb and .rpm). It ensures the release
# is ready for distribution.

set -e # Exit immediately on any error

# Check for version argument
if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"

# Step 1: Format the code using the formatter
./package/format.sh

cd package

# Step 2: Build the binary
echo "Creating binary..."
./create_bin.sh "$VERSION"

# Step 3: Build the Debian package
echo "Creating Debian package..."
./create_deb.sh "$VERSION"

# Step 4: Build the RPM package
echo "Creating RPM package..."
./create_rpm.sh "$VERSION"

echo -e "\nAll release artifacts created successfully."
