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

BIN_NAME="cmdcreate-dev"
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
RESET='\033[0m'

INSTALL_DIR=""
USE_SUDO=true
LICENSE_PATH=""

is_immutable() {
    if [[ -d "/ostree" ]]; then
        return 0
    fi

    if [[ -f "/etc/os-release" ]]; then
        if grep -qiE "silverblue|kinoite|microos|bazzite|aurae" /etc/os-release; then
            return 0
        fi
    fi

    if awk '$2 == "/" || $2 == "/usr" {print $4}' /proc/mounts 2> /dev/null | grep -q -E '(^|,)ro(,|$);'; then
        return 0
    fi

    return 1
}

show_help() {
    echo -e "\n${YELLOW}> Usage:${RESET} $0 [options]"
    echo -e "\n${YELLOW}Uninstallation Scope (Required):${RESET}"
    echo "  -u, --user      Uninstall from \$HOME/.local/bin/"
    echo "  -s, --system    Uninstall from /usr/bin/ (Requires root)"
    echo -e "\n${YELLOW}Flags:${RESET}"
    echo "  -h, --help      Show this help menu"
    exit 0
}

if [[ $# -eq 0 ]]; then
    echo -e "${RED}Error: You must specify an uninstallation scope (--user or --system).${RESET}"
    show_help
fi

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h | --help)
            show_help
            ;;
        -u | --user)
            INSTALL_DIR="$HOME/.local/bin"
            USE_SUDO=false
            shift
            ;;
        -s | --system)
            INSTALL_DIR="/usr/bin"
            USE_SUDO=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            show_help
            ;;
    esac
done

if [[ -z "$INSTALL_DIR" ]]; then
    echo -e "${RED}Error: Missing uninstallation scope (-u or -s).${RESET}"
    exit 1
fi

if is_immutable; then
    if [ "$INSTALL_DIR" = "/usr/bin" ]; then
        echo -e "${YELLOW}> Warning: Immutable distribution detected. Forcing user directory scope instead.${RESET}"
    fi
    INSTALL_DIR="$HOME/.local/bin"
    USE_SUDO=false
fi

if [ "$USE_SUDO" = false ]; then
    LICENSE_PATH="$HOME/.local/share/doc/$BIN_NAME/LICENSE"
else
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "${ID:-}" in
            ubuntu | debian | kali | pop | linuxmint)
                LICENSE_PATH="/usr/share/doc/$BIN_NAME/copyright"
                ;;
            fedora | rhel | centos | amzn)
                LICENSE_PATH="/usr/share/doc/$BIN_NAME/LICENSE"
                ;;
            arch | manjaro | endeavouros)
                LICENSE_PATH="/usr/share/licenses/$BIN_NAME/LICENSE"
                ;;
            *)
                LICENSE_PATH="/usr/local/share/doc/$BIN_NAME/LICENSE"
                ;;
        esac
    else
        LICENSE_PATH="/usr/local/share/doc/$BIN_NAME/LICENSE"
    fi
fi

if [ -f "$INSTALL_DIR/$BIN_NAME" ]; then
    if [ "$USE_SUDO" = true ]; then
        sudo rm "$INSTALL_DIR/$BIN_NAME"
    else
        rm "$INSTALL_DIR/$BIN_NAME"
    fi
    echo -e "${BLUE}> Removed binary:${RESET} $INSTALL_DIR/$BIN_NAME"
else
    echo -e "${YELLOW}> Binary not found in $INSTALL_DIR, skipping.${RESET}"
fi

if [ -n "$LICENSE_PATH" ] && [ -f "$LICENSE_PATH" ]; then
    LICENSE_DIR=$(dirname "$LICENSE_PATH")

    if [ "$USE_SUDO" = true ]; then
        sudo rm "$LICENSE_PATH"
        echo -e "${BLUE}> Removed license:${RESET} $LICENSE_PATH"
        if [ -d "$LICENSE_DIR" ] && [ ! "$(ls -A "$LICENSE_DIR")" ]; then
            sudo rm -rf "$LICENSE_DIR"
            echo -e "${BLUE}> Removed empty directory:${RESET} $LICENSE_DIR"
        fi
    else
        rm "$LICENSE_PATH"
        echo -e "${BLUE}> Removed license:${RESET} $LICENSE_PATH"

        if [ -d "$LICENSE_DIR" ] && [ ! "$(ls -A "$LICENSE_DIR")" ]; then
            rm -rf "$LICENSE_DIR"
            echo -e "${BLUE}> Removed empty directory:${RESET} $LICENSE_DIR"
        fi
    fi
else
    echo -e "${YELLOW}> License not found, skipping.${RESET}"
fi

echo -e "\n${GREEN}Done.${RESET}"
