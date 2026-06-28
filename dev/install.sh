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
YELLOW='\033[0;33m'
RED='\033[0;31m'
RESET='\033[0m'

BIN_NAME="cmdcreate-dev"
INSTALL_DIR=""
USE_SUDO=true
OFFLINE_MODE=false

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

check_conflicting_binaries() {
    local conflicts=()
    local target_file="${INSTALL_DIR%/}/$BIN_NAME"
    local paths=(
        "$HOME/.local/bin/cmdcreate"
        "/usr/bin/cmdcreate"
        "$HOME/.local/bin/cmdcreate-dev"
        "/usr/bin/cmdcreate-dev"
    )

    for p in "${paths[@]}"; do
        if [[ "$p" == "$target_file" ]]; then
            continue
        fi
        if [[ "$INSTALL_DIR" == "/usr/bin/" && "$p" == "$HOME/.local/bin/"* ]]; then
            continue
        fi
        if [[ "$INSTALL_DIR" == "$HOME/.local/bin" && "$p" == /usr/bin/* ]]; then
            continue
        fi

        if [[ -f "$p" ]]; then
            conflicts+=("$p")
        fi
    done

    if [[ ${#conflicts[@]} -gt 0 ]]; then
        echo -e "\n${RED}> Conflict Detected: The following binaries cannot coexist with this installation:${RESET}"
        for c in "${conflicts[@]}"; do
            echo -e "  - $c"
        done

        echo -ne "\n${YELLOW}Would you like to delete these conflicting binaries to proceed? (y/N): ${RESET}"
        read -r response
        if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            for c in "${conflicts[@]}"; do
                if [[ "$c" == /usr/bin/* ]]; then
                    echo -e "${BLUE}> Removing system binary (requires sudo):${RESET} $c"
                    sudo rm -f "$c"
                else
                    echo -e "${BLUE}> Removing user binary:${RESET} $c"
                    rm -f "$c"
                fi
            done
            echo -e "${GREEN}> Conflicts resolved.${RESET}"
        else
            echo -e "${RED}Installation aborted due to conflicting binaries.${RESET}"
            exit 1
        fi
    fi
}

add_to_shell_paths() {
    local target_path="\$HOME/.local/bin"

    if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
        return 0
    fi

    echo -e "\n${YELLOW}> Notice: $HOME/.local/bin is not in your PATH. Updating shell configs...${RESET}"

    if [[ -f "$HOME/.bashrc" ]]; then
        if ! grep -q "export PATH=.*\.local/bin" "$HOME/.bashrc"; then
            echo -e "\n# Added by $BIN_NAME installer\nexport PATH=\"$target_path:\$PATH\"" >> "$HOME/.bashrc"
            echo -e "  -> Added to ~/.bashrc"
        fi
    fi

    if [[ -f "$HOME/.zshrc" ]]; then
        if ! grep -q "export PATH=.*\.local/bin" "$HOME/.zshrc"; then
            echo -e "\n# Added by $BIN_NAME installer\nexport PATH=\"$target_path:\$PATH\"" >> "$HOME/.zshrc"
            echo -e "  -> Added to ~/.zshrc"
        fi
    fi

    if which fish &> /dev/null; then
        mkdir -p "$HOME/.config/fish"
        local fish_config="$HOME/.config/fish/config.fish"
        touch "$fish_config"
        if ! grep -q "fish_add_path.*\.local/bin" "$fish_config"; then
            echo -e "\n# Added by $BIN_NAME installer\nfish_add_path \$HOME/.local/bin" >> "$fish_config"
            echo -e "  -> Added to $fish_config"
        fi
    fi

    echo -e "${YELLOW}! Please restart your shell or source your config file for changes to take effect.${RESET}"
}

show_help() {
    echo -e "\n${YELLOW}> Usage:${RESET} $0 [options]"
    echo -e "\n${YELLOW}Installation Type (Required):${RESET}"
    echo "  -u, --user      Install to \$HOME/.local/bin/"
    echo "  -s, --system    Install to /usr/bin/ (Requires root)"
    echo -e "\n${YELLOW}Flags:${RESET}"
    echo "  -o, --offline   Skip toolchain updates and use offline cargo build"
    echo "  -h, --help      Show this help menu"
    exit 0
}

if [[ $# -eq 0 ]]; then
    echo -e "${RED}Error: You must specify an installation scope (--user or --system).${RESET}"
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
            INSTALL_DIR="/usr/bin/"
            USE_SUDO=true
            shift
            ;;
        -o | --offline)
            OFFLINE_MODE=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            show_help
            ;;
    esac
done

if [[ -z "$INSTALL_DIR" ]]; then
    echo -e "${RED}Error: Missing installation scope (-u or -s).${RESET}"
    exit 1
fi

if is_immutable; then
    if [ "$INSTALL_DIR" = "/usr/bin/" ]; then
        echo -e "${YELLOW}> Warning: Immutable distribution detected. Forcing installation to local user directory instead.${RESET}"
    fi
    INSTALL_DIR="$HOME/.local/bin"
    USE_SUDO=false
fi

check_conflicting_binaries

if [ "$USE_SUDO" = false ]; then
    mkdir -p "$INSTALL_DIR"
fi

ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" ]]; then
    RUST_TARGET="x86_64-unknown-linux-musl"
    ZIG_TARGET="x86_64-linux-musl"
    CC_ENV_VAR="CC_x86_64_unknown_linux_musl"
elif [[ "$ARCH" == "i686" || "$ARCH" == "i386" ]]; then
    RUST_TARGET="i686-unknown-linux-musl"
    ZIG_TARGET="x86-linux-musl"
    CC_ENV_VAR="CC_i686_unknown_linux_musl"
elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
    RUST_TARGET="aarch64-unknown-linux-musl"
    ZIG_TARGET="aarch64-linux-musl"
    CC_ENV_VAR="CC_aarch64_unknown_linux_musl"
elif [[ "$ARCH" == "armv7l" || "$ARCH" == "armv7" ]]; then
    RUST_TARGET="armv7-unknown-linux-musleabihf"
    ZIG_TARGET="arm-linux-musleabihf"
    CC_ENV_VAR="CC_armv7_unknown_linux_musleabihf"
else
    echo "Error: Unsupported architecture $ARCH"
    exit 1
fi

TARGET_BIN="target/$RUST_TARGET/release/$BIN_NAME"

if [ "$OFFLINE_MODE" = false ]; then
    echo -e "\n${BLUE}> Updating Rust toolchain...${RESET}"
    rustup update stable
    rustup target add "$RUST_TARGET"

    echo -e "\n${BLUE}> Updating Cargo...${RESET}"
    cargo install cargo-zigbuild
    cargo update
fi

if [[ -f "./dev/format.sh" ]]; then
    ./dev/format.sh
fi

echo -e "\n${BLUE}> Running clippy ($ARCH)...${RESET}"
export CRATE_CC_NO_DEFAULTS=true
export "$CC_ENV_VAR"="zig cc -target $ZIG_TARGET -fno-sanitize=all"

cargo clippy --fix --allow-no-vcs --target "$RUST_TARGET"

echo -e "\n${BLUE}> Building release (Static Musl $ARCH)...${RESET}"
cargo zigbuild --release --target "$RUST_TARGET"

if [ -f "target/$RUST_TARGET/release/cmdcreate" ]; then
    cp "target/$RUST_TARGET/release/cmdcreate" "$TARGET_BIN"
fi

echo -e "\n${BLUE}> Installing binary...${RESET}"
if [ "$USE_SUDO" = true ]; then
    sudo install -Dm755 "$TARGET_BIN" "$INSTALL_DIR/$BIN_NAME"
else
    install -Dm755 "$TARGET_BIN" "$INSTALL_DIR/$BIN_NAME"
    add_to_shell_paths
fi

echo -e "\n${GREEN}> Done. $BIN_NAME (statically linked $ARCH) installed to \"${BLUE}$INSTALL_DIR/$BIN_NAME\".${RESET}"
echo -e "\n${GREEN}> Run ${BLUE}\"cmdcreate-dev\"${GREEN} to run this development build!${RESET}"
