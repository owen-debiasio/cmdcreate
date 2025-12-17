#!/usr/bin/env bash
# package_rpm.sh - Build an RPM package for cmdcreate
#
# This script takes a pre-built cmdcreate binary, creates the RPM spec file,
# organizes the necessary RPM build directories, and produces an installable
# RPM package.
#
# Usage:
#   ./package_rpm.sh <version>
# Example:
#   ./package_rpm.sh 0.7.1
#
# Note:
#   - The version argument MUST NOT start with 'v'.
#   - Ensure the binary exists in ~/Downloads before running.

set -e # Exit immediately if any command fails

# Check for version argument
if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version (MUST NOT START WITH v)"
    exit 1
fi

VERSION="$1"
BINARY_SRC="$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64-bin"
RPMBUILD_DIR="$HOME/rpmbuild"

# Ensure binary exists
if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary $BINARY_SRC not found. Build it first."
    exit 1
fi

# Create RPM build directory structure
mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Define the RPM spec file
SPEC_FILE="$RPMBUILD_DIR/SPECS/cmdcreate.spec"

# Generate the spec file with metadata, dependencies, and installation steps
cat >"$SPEC_FILE" <<EOF
Name:           cmdcreate
Version:        $VERSION
Release:        1%{?dist}
Summary:        Allows you to create custom commands for your custom scripts

License:        MIT
URL:            https://github.com/Meme-Supplier/cmdcreate
Source0:        %{name}-%{version}-linux-x86_64-bin

BuildArch:      x86_64
Requires:       bash, curl, nano

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

# Copy the binary into the SOURCES folder
cp "$BINARY_SRC" "$RPMBUILD_DIR/SOURCES/cmdcreate-${VERSION}-linux-x86_64-bin"

# Build the RPM package
rpmbuild -bb "$SPEC_FILE"

# Locate the built RPM and copy it to Downloads
RPM_FILE=$(find "$RPMBUILD_DIR/RPMS/x86_64" -name "cmdcreate-${VERSION}-*.rpm" | head -n1)
FINAL_RPM="$HOME/Downloads/cmdcreate-v${VERSION}-linux-x86_64.rpm"
cp "$RPM_FILE" "$FINAL_RPM"

# Clean up the RPM build directory
rm -r "$RPMBUILD_DIR"

# Notify user
echo -e "\nBuilt and moved $FINAL_RPM to ~/Downloads"
