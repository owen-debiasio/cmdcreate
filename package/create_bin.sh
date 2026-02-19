#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo -e "\n> Usage: $0 <version> (no leading v)"
    exit 1
fi

ARCH="x86_64"

BINARY_NAME="cmdcreate-v$1-linux-${ARCH}-bin"
BINARY_SRC="$HOME/Downloads/$BINARY_NAME"

cargo update
rustup update

cargo build --release

cp ../target/release/cmdcreate ~/Downloads/

mv "$HOME/Downloads/cmdcreate" "$BINARY_SRC"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary not found: $BINARY_SRC"
    exit 1
fi

echo -e "\n> Packaged cmdcreate to \"$BINARY_NAME\""
