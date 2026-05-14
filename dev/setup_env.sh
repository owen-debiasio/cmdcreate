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

BLUE='\033[0;34m'
GREEN='\033[0;32m'
RESET='\033[0m'

if [[ -f /etc/os-release ]]; then
    source /etc/os-release
else
    exit 1
fi

ID_LIKE="${ID_LIKE:-}"

ask_yn() {
    echo -e "$1"
    read -r -p "[y/N]: " answer
    echo "${RESET}"
    answer="${answer,,}"
    if [[ "$answer" == "y" || "$answer" == "yes" ]]; then
        return 0
    else
        exit 1
    fi
}

install_dependencies() {
    local OS_FAMILY="${ID:-$ID_LIKE}"
    case "$OS_FAMILY" in
        *arch*)
            sudo pacman -S --needed --noconfirm \
                rustup curl openssl git base-devel \
                shfmt shellcheck bash-language-server \
                python-black python-pylint python-lsp-server \
                nodejs npm markdownlint-cli2 prettier marksman vscode-json-languageserver \
                rpm-tools dpkg
            ;;
        *fedora*)
            sudo dnf install -y \
                curl openssl-devel git gcc gcc-c++ make \
                shfmt ShellCheck nodejs-bash-language-server \
                python3-black python3-pylint \
                nodejs npm rpm-build dpkg-dev

            sudo npm install -g prettier markdownlint-cli2@0.13.0
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
            ;;
        *debian* | *ubuntu*)
            sudo apt-get update
            sudo apt-get install -y \
                curl libssl-dev build-essential pkg-config git \
                shfmt shellcheck \
                black pylint \
                nodejs npm rpm dpkg-dev
            sudo npm install -g prettier markdownlint-cli2@0.13.0
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
            ;;
    esac
}

ask_yn "${BLUE}> Do you want to set up cmdcreate's dev environment?"

echo -e "${BLUE}> Installing dependencies...${RESET}"

install_dependencies

echo -e "${BLUE}> Configuring Rust...${RESET}"

if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

rustup default stable
rustup component add rust-analyzer
rustup target add x86_64-unknown-linux-musl i686-unknown-linux-musl aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf
cargo install cargo-zigbuild

echo -e "${BLUE}> Enter directory for cmdcreate dev environment:${RESET}"
read -r -p "> " dev_dir
echo -e "${RESET}"

dev_dir="${dev_dir/#\~/$HOME}"

echo -e "${BLUE}> Setting up environment at: $dev_dir...${RESET}"

git clone https://github.com/owen-debiasio/cmdcreate.git "$dev_dir"
cd "$dev_dir"

echo -e "${BLUE}> Activating shell scripts...${RESET}"

find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +

echo -e "\n${GREEN}> Setup complete!${RESET}"
