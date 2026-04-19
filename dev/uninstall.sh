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

set -euo pipefail

BIN="cmdcreate"
INSTALL_DIR="/usr/bin"
LICENSE_PATH=""

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID" in
        ubuntu | debian | kali | pop | linuxmint)
            LICENSE_PATH="/usr/share/doc/cmdcreate/copyright"
            ;;
        fedora | rhel | centos | amzn)
            LICENSE_PATH="/usr/share/doc/cmdcreate/LICENSE"
            ;;
        arch | manjaro | endeavouros)
            LICENSE_PATH="/usr/share/licenses/cmdcreate/LICENSE"
            ;;
        *)
            LICENSE_PATH="/usr/local/share/doc/cmdcreate/LICENSE"
            ;;
    esac
else
    LICENSE_PATH="/usr/local/share/doc/cmdcreate/LICENSE"
fi

if [ -f "$INSTALL_DIR/$BIN" ]; then
    sudo rm "$INSTALL_DIR/$BIN"
    echo -e "${BLUE}> Removed binary: ${RESET}$INSTALL_DIR/$BIN"
else
    echo -e "${YELLOW}> Binary not found, skipping.${RESET}"
fi

if [ -n "$LICENSE_PATH" ] && [ -f "$LICENSE_PATH" ]; then
    LICENSE_DIR=$(dirname "$LICENSE_PATH")
    sudo rm "$LICENSE_PATH"
    echo -e "${BLUE}> Removed license: ${RESET}$LICENSE_PATH"

    if [ -d "$LICENSE_DIR" ] && [ ! "$(ls -A "$LICENSE_DIR")" ]; then
        sudo rm -rf "$LICENSE_DIR"
        echo -e "${BLUE}> Removed empty directory:${RESET} $LICENSE_DIR"
    fi
else
    echo -e "${YELLOW}> License not found, skipping.${RESET}"
fi

echo -e "\n${GREEN}Done.${RESET}"
