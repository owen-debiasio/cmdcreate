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

if [ -z "$(git config user.email)" ] || [ -z "$(git config user.name)" ]; then
    echo "Error: Git identity not found."
    echo ""
    echo "Please run:"
    echo "  git config --global user.email \"you@example.com\""
    echo "  git config --global user.name \"Your Name\""
    exit 1
fi

if [ ! -f "$HOME/.ssh/id_ed25519" ] && [ ! -f "$HOME/.ssh/id_rsa" ]; then
    echo "Error: No SSH key found in ~/.ssh/"
    echo "Please generate one using: ssh-keygen -t ed25519"
    exit 1
fi

echo "Checking connection to AUR..."
if ! ssh -T aur@aur.archlinux.org -o BatchMode=yes 2>&1 | grep -q "Interactive shell is not allowed"; then
    echo "Error: Could not authenticate with aur.archlinux.org"
    exit 1
fi

if [ -f "./dev/uninstall.sh" ]; then
    echo "Running local uninstall script..."
    ./dev/uninstall.sh
fi

OLD_VERSION=$(grep -P '^pkgver=' PKGBUILD | cut -d= -f2 || echo "0")

WORK_DIR=$(mktemp -d /tmp/aur-update-XXXXXX)
trap 'rm -rf "$WORK_DIR"' EXIT

echo "Cloning to temporary directory: $WORK_DIR"
git clone ssh://aur@aur.archlinux.org/cmdcreate.git "$WORK_DIR"
cd "$WORK_DIR"

${EDITOR:-nano} PKGBUILD

NEW_VERSION=$(grep -P '^pkgver=' PKGBUILD | cut -d= -f2 || echo "0")

if [ "$OLD_VERSION" == "$NEW_VERSION" ]; then
    read -r -p "Version hasn't changed ($OLD_VERSION). Continue anyway? [y/N]: " conf
    if [[ ! "$conf" =~ ^[yY]$ ]]; then
        exit 0
    fi
fi

makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO

if git diff-index --quiet HEAD --; then
    echo "No changes to commit, skipping."
else
    git pull --rebase origin master
    git push origin master
fi

echo -e "\nUpdate complete."
