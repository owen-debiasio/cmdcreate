#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <version> (no leading v)"
    exit 1
fi

VERSION="$1"
ARCH="x86_64"

BINARY_NAME="cmdcreate-v${VERSION}-linux-${ARCH}-bin"
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

echo -e "\nPackaged cmdcreate to \"$BINARY_NAME\""
