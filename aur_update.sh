#!/bin/bash
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

if [ "$(git config --global user.email)" != "owen.debiasio@gmail.com" ]; then
    git config --global user.email "owen.debiasio@gmail.com"
fi

if [ "$(git config --global user.name)" != "Owen Debiasio" ]; then
    git config --global user.name "Owen Debiasio"
fi

if [ ! -f "$HOME/.ssh/id_ed25519" ]; then
    ssh-keygen -t ed25519 -C "owen.debiasio@gmail.com" -f "$HOME/.ssh/id_ed25519" -N ""
fi

ssh -T aur@aur.archlinux.org -o BatchMode=yes || true

if [ -f "./dev/uninstall.sh" ]; then
    ./dev/uninstall.sh
fi

OLD_VERSION=$(grep -P '^pkgver=' PKGBUILD | cut -d= -f2 || echo "0")

WORK_DIR=$(mktemp -d /tmp/aur-update-XXXXXX)
trap 'rm -rf "$WORK_DIR"' EXIT

cd "$WORK_DIR"

git clone ssh://aur@aur.archlinux.org/cmdcreate.git .

nano PKGBUILD

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
    git commit -m "$1"
    git pull --rebase origin master
    git push origin master
fi

echo -e "\ndone"
