#!/bin/bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Owen Debiasio
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 \"commit message\""
    exit 1
fi

echo "Enter the AUR package name to update:"
echo "1) cmdcreate"
echo "2) cmdcreate-git"
read -r -p "Selection [1/2]: " choice

case "$choice" in
    1) PKG_NAME="cmdcreate" ;;
    2) PKG_NAME="cmdcreate-git" ;;
    *)
        echo "Invalid selection. Exiting."
        exit 1
        ;;
esac

if ! command -v ssh-keyscan &> /dev/null; then
    echo "Error: openssh is not installed."
    exit 1
fi

if ! ssh-keygen -F aur.archlinux.org > /dev/null; then
    echo "Adding aur.archlinux.org to known_hosts..."
    mkdir -p ~/.ssh
    ssh-keyscan aur.archlinux.org >> ~/.ssh/known_hosts 2> /dev/null
fi

if [ -z "$(git config user.email)" ] || [ -z "$(git config user.name)" ]; then
    echo "Error: Git identity not found. Run 'git config --global user.email ...'"
    exit 1
fi

if [ ! -f "$HOME/.ssh/id_ed25519" ] && [ ! -f "$HOME/.ssh/id_rsa" ]; then
    echo "Error: No SSH key found in ~/.ssh/"
    exit 1
fi

echo "Checking connection to AUR for $PKG_NAME..."
SSH_TEST=$(ssh -o ConnectTimeout=5 -T aur@aur.archlinux.org 2>&1 || true)

if echo "$SSH_TEST" | grep -qiE "Welcome|Interactive shell"; then
    echo "Authentication successful."
elif echo "$SSH_TEST" | grep -q "Permission denied"; then
    echo "Error: Permission denied. Is your public key uploaded to the AUR website?"
    exit 1
else
    echo "Error: Could not connect to AUR."
    echo "Details: $SSH_TEST"
    exit 1
fi

if [ -f "./dev/uninstall.sh" ]; then
    ./dev/uninstall.sh
fi

WORK_DIR=$(mktemp -d /tmp/aur-update-XXXXXX)
trap 'rm -rf "$WORK_DIR"' EXIT

echo "Cloning $PKG_NAME..."
git clone "ssh://aur@aur.archlinux.org/$PKG_NAME.git" "$WORK_DIR"
cd "$WORK_DIR"

OLD_VERSION=$(grep -P '^pkgver=' PKGBUILD | cut -d= -f2 || echo "0")

${EDITOR:-nano} PKGBUILD

NEW_VERSION=$(grep -P '^pkgver=' PKGBUILD | cut -d= -f2 || echo "0")

if [ "$OLD_VERSION" == "$NEW_VERSION" ]; then
    read -r -p "Version hasn't changed ($OLD_VERSION). Continue anyway? [y/N]: " conf
    if [[ ! "$conf" =~ ^[yY]$ ]]; then
        exit 0
    fi
fi

echo "Updating .SRCINFO..."
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO

if git diff-index --quiet HEAD --; then
    echo "No changes detected."
else
    git commit -m "$1"
    echo "Pushing changes..."
    git push origin master
fi

echo -e "\nUpdate complete for $PKG_NAME."
