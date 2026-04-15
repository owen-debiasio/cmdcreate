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

if [[ -f /etc/os-release ]]; then
    source /etc/os-release
else
    exit 1
fi

ID_LIKE="${ID_LIKE:-}"

ask_yn() {
    read -r -p "$1 [y/N]: " answer
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
                shfmt shellcheck \
                python-black python-pylint \
                nodejs npm markdownlint-cli2 prettier \
                rpm-tools dpkg
            ;;

        *fedora* | *rhel* | *centos*)
            sudo dnf install -y \
                curl openssl-devel git gcc gcc-c++ make \
                shfmt ShellCheck \
                python3-black python3-pylint \
                nodejs npm rpm-build dpkg-dev

            sudo npm install -g prettier markdownlint-cli2@0.13.0

            if ! command -v rustup &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            fi
            ;;

        *debian* | *ubuntu* | *pop*)
            sudo apt-get update
            sudo apt-get install -y \
                curl libssl-dev build-essential pkg-config git \
                shfmt shellcheck \
                black pylint \
                nodejs npm rpm dpkg-dev

            sudo npm install -g prettier markdownlint-cli2@0.13.0

            if ! command -v rustup &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            fi
            ;;

        *)
            exit 1
            ;;
    esac
}

ask_yn "> Do you want to set up cmdcreate's dev environment?"

echo "> Installing dependencies..."

install_dependencies

echo "> Configuring Rust..."

if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

rustup default stable

read -r -p "> Enter directory for cmdcreate dev environment: " dev_dir

dev_dir="${dev_dir/#\~/$HOME}"

echo "> Setting up enviornment at: $dev_dir..."

git clone https://github.com/owen-debiasio/cmdcreate.git "$dev_dir"
cd "$dev_dir"

echo "> Activating shell scripts..."

find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +

echo -e "\nSetup complete!"
