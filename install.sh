#!/bin/bash

# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Owen Debiasio

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
RESET='\033[0m'

REPO="owen-debiasio/cmdcreate"
OS_TYPE=""
LICENSE_PATH=""

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu | debian | kali | pop | linuxmint)
                OS_TYPE="debian"
                LICENSE_PATH="/usr/share/doc/cmdcreate/copyright"
                ;;
            fedora | rhel | centos | amzn)
                OS_TYPE="fedora"
                LICENSE_PATH="/usr/share/doc/cmdcreate/LICENSE"
                ;;
            arch | manjaro | endeavouros)
                OS_TYPE="arch"
                LICENSE_PATH="/usr/share/licenses/cmdcreate/LICENSE"
                ;;
            *)
                OS_TYPE="other"
                LICENSE_PATH="/usr/local/share/doc/cmdcreate/LICENSE"
                ;;
        esac
    else
        OS_TYPE="other"
        LICENSE_PATH="/usr/local/share/doc/cmdcreate/LICENSE"
    fi
}

get_latest_version() {
    LATEST_TAG=$(curl -sSfL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$LATEST_TAG" ]; then
        echo -e "${RED}> Error: Could not detect version.${RESET}" >&2
        exit 1
    fi
    VERSION="$LATEST_TAG"
}

install_license() {
    sudo mkdir -p "$(dirname "$LICENSE_PATH")"
    if [ -f "$1" ]; then
        sudo cp "$1" "$LICENSE_PATH"
    else
        sudo curl -sSfL -o "$LICENSE_PATH" "https://raw.githubusercontent.com/$REPO/main/LICENSE"
    fi
    sudo chmod 644 "$LICENSE_PATH"
}

install_dependencies() {
    echo -e "${BLUE}> Installing dependencies...${RESET}"
    case "$OS_TYPE" in
        debian)
            sudo apt-get update -qq
            sudo apt-get install -y -qq curl libssl-dev build-essential pkg-config git
            ;;
        fedora)
            sudo dnf install -y -q curl openssl-devel git
            ;;
        arch)
            sudo pacman -Syq > /dev/null 2>&1
            sudo pacman -S --needed --noconfirm --noprogressbar base-devel git rustup curl > /dev/null 2>&1
            ;;
    esac

    if ! command -v rustup &> /dev/null; then
        echo -e "${BLUE}> Rustup not found. Installing... ${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -q
        source "$HOME/.cargo/env"
    fi

    rustup default stable > /dev/null 2>&1
}

build_from_source() {
    install_dependencies
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    echo -e "${BLUE}> Building... (Please wait as this will take a while)${RESET}"
    git clone --depth=1 --quiet "https://github.com/$REPO.git" .
    cargo build --release -q

    echo -e "${BLUE}> Installing...${RESET}"

    sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate
    install_license "LICENSE"

    cd "$HOME"
    rm -rf "$TEMP_DIR"
}

install_bin() {
    get_latest_version
    echo -e "${BLUE}> Downloading standalone binary...${RESET}"
    URL="https://github.com/$REPO/releases/download/${VERSION}/cmdcreate-${VERSION}-linux-x86_64-bin"
    sudo curl -sSfL -o /usr/bin/cmdcreate "$URL"
    sudo chmod +x /usr/bin/cmdcreate
    install_license ""
}

install_pkg() {
    get_latest_version
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    if [ "$OS_TYPE" == "debian" ]; then
        echo -e "${BLUE}> Installing...${RESET}"
        URL="https://github.com/$REPO/releases/download/${VERSION}/cmdcreate-${VERSION}-linux-x86_64.deb"
        curl -sSfL -o cmdcreate.deb "$URL"
        sudo dpkg -i cmdcreate.deb > /dev/null 2>&1 || sudo apt-get install -f -y -qq
    else
        echo -e "${BLUE}> Installing...${RESET}"
        URL="https://github.com/$REPO/releases/download/${VERSION}/cmdcreate-${VERSION}-linux-x86_64.rpm"
        curl -sSfL -o cmdcreate.rpm "$URL"
        sudo rpm -Uvh --quiet cmdcreate.rpm
    fi

    install_license ""
    cd "$HOME"
    rm -rf "$TEMP_DIR"
}

install_aur() {
    echo -e "${BLUE}AUR Installation Options:${RESET}"
    read -rp "1) Stable, 2) Git: " aur_choice
    PKG_NAME="cmdcreate"
    [ "$aur_choice" == "2" ] && PKG_NAME="cmdcreate-git"

    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    echo -e "${BLUE}> Installing... (Please wait as this may take a while)${RESET}"
    git clone --quiet "https://aur.archlinux.org/$PKG_NAME.git" .
    makepkg -si --noconfirm > /dev/null 2>&1

    cd "$HOME"
    rm -rf "$TEMP_DIR"
}

detect_distro
echo -e "${BLUE}Welcome to the cmdcreate installer! Please choose an option:${RESET}\n"

case "$OS_TYPE" in
    debian)
        echo -e "${BLUE}1)${RESET} Build from source"
        echo -e "${BLUE}2)${RESET} Debian (.deb) package (x86 only)"
        echo -e "${BLUE}3)${RESET} Raw binary (x86 only)"
        echo ""
        read -rp "Select: " choice
        case $choice in
            1) build_from_source ;;
            2) install_pkg ;;
            3) install_bin ;;
        esac
        ;;
    fedora)
        echo -e "${BLUE}1)${RESET} Build from source"
        echo -e "${BLUE}2)${RESET} RPM (.rpm) package (x86 only)"
        echo -e "${BLUE}3)${RESET} Raw binary (x86 only)"
        echo ""
        read -rp "Select: " choice
        case $choice in
            1) build_from_source ;;
            2) install_pkg ;;
            3) install_bin ;;
        esac
        ;;
    arch)
        echo -e "${BLUE}1)${RESET} Build from source"
        echo -e "${BLUE}2)${RESET} AUR (stable and git)"
        echo -e "${BLUE}3)${RESET} Raw binary (x86 only)"
        echo ""
        read -rp "Select: " choice
        case $choice in
            1) build_from_source ;;
            2) install_aur ;;
            3) install_bin ;;
        esac
        ;;
    *)
        echo -e "${BLUE}1)${RESET} Build from source"
        echo -e "${BLUE}2)${RESET} Binary (x86 only)"
        echo ""
        read -rp "Select: " choice
        case $choice in
            1) build_from_source ;;
            2) install_bin ;;
        esac
        ;;
esac

if [ -f "/usr/bin/cmdcreate" ] && [ -f "$LICENSE_PATH" ]; then
    echo -e "\n${GREEN}> Done. cmdcreate has been installed successfully!${RESET}"
    exit 0
else
    echo -e "${RED}> Error: One or more files are missing from installation.${RESET}" >&2
    [ ! -f "/usr/bin/cmdcreate" ] && echo -e "${RED}> Missing: /usr/bin/cmdcreate${RESET}" >&2
    [ ! -f "$LICENSE_PATH" ] && echo -e "${RED}> Missing: $LICENSE_PATH${RESET}" >&2
    exit 1
fi
