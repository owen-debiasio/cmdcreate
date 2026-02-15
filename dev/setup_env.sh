#!/usr/bin/env bash
set -euo pipefail

# shellcheck source=/dev/null
if [[ -f /etc/os-release ]]; then
    source /etc/os-release
else
    echo "/etc/os-release not found. Cannot detect distro."
    exit 1
fi

ID_LIKE="${ID_LIKE:-}"

ask_yn() {
    read -r -p "$1 [y/N]: " answer
    answer="${answer,,}"
    if [[ "$answer" == "y" || "$answer" == "yes" ]]; then
        return 0
    else
        echo "Aborted."
        exit 1
    fi
}

ask_yn "> Do you want to set up cmdcreate's dev environment?"
echo -e "\n> Installing dependencies..."

install_dependencies() {
    case "$ID" in
        arch)
            sudo pacman -S --needed \
                rustup curl openssl git base-devel \
                shfmt shellcheck \
                python-black python-pylint \
                nodejs npm markdownlint-cli2
            ;;
        fedora)
            sudo dnf install -y \
                curl openssl-devel git \
                shfmt ShellCheck \
                python3-black python3-pylint \
                nodejs npm
            sudo npm install -g markdownlint-cli2 || true
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            ;;
        debian | ubuntu)
            sudo apt update
            sudo apt install -y \
                curl libssl-dev build-essential pkg-config git \
                shfmt shellcheck \
                black pylint \
                nodejs npm
            sudo npm install -g markdownlint-cli2 || true
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            ;;
        *)
            if [[ "$ID_LIKE" == *debian* ]]; then
                sudo apt update
                sudo apt install -y \
                    curl libssl-dev build-essential pkg-config git \
                    shfmt shellcheck \
                    black pylint \
                    nodejs npm
                sudo npm install -g markdownlint-cli2 || true
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            else
                echo -e "\n> Unsupported distro: $ID"
                exit 1
            fi
            ;;
    esac
}

install_dependencies

echo -e "\n> Setting up Rust..."

# shellcheck source=/dev/null
source "$HOME/.cargo/env"

rustup default stable

read -r -p "Enter directory for cmdcreate dev environment: " dev_dir
dev_dir="$(eval echo "$dev_dir")"
git clone https://github.com/owen-debiasio/cmdcreate.git "$dev_dir"
cd "$dev_dir" || exit 1

echo -e "\n> Activating shell scripts..."
find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +

echo -e "\n> Dev environment ready...\nOpen the environment in your chosen text editor or IDE..."
