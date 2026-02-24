#!/usr/bin/env bash
set -euo pipefail

if [[ -f /etc/os-release ]]; then
    # shellcheck source=/dev/null
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
                nodejs npm markdownlint-cli2 \
                rpm-tools dpkg
            ;;

        *fedora* | *rhel* | *centos*)
            sudo dnf install -y \
                curl openssl-devel git gcc gcc-c++ make \
                shfmt ShellCheck \
                python3-black python3-pylint \
                nodejs npm rpm-build dpkg-dev

            sudo npm install -g markdownlint-cli2 || true

            if ! command -v rustup &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                # shellcheck source=/dev/null
                source "$HOME/.cargo/env"
            fi
            ;;

        *debian* | *ubuntu* | *pop*)
            sudo apt-get update
            sudo apt-get install -y \
                curl libssl-dev build-essential pkg-config git \
                shfmt shellcheck \
                black pylint \
                nodejs npm rpm dpkg-dev

            sudo npm install -g markdownlint-cli2 || true

            if ! command -v rustup &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                # shellcheck source=/dev/null
                source "$HOME/.cargo/env"
            fi
            ;;

        *)
            exit 1
            ;;
    esac
}

ask_yn "> Do you want to set up cmdcreate's dev environment?"

install_dependencies

if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

rustup default stable

read -r -p "Enter directory for cmdcreate dev environment: " dev_dir
dev_dir="$(eval echo "$dev_dir")"
git clone https://github.com/owen-debiasio/cmdcreate.git "$dev_dir"
cd "$dev_dir"

find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +

echo -e "\nDone."
