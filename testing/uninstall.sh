#!/usr/bin/env bash

set -euo pipefail

BIN="cmdcreate"
INSTALL_DIR="/usr/local/bin"

if [ -f "$INSTALL_DIR/$BIN" ]; then
    sudo rm "$INSTALL_DIR/$BIN"
    echo "cmdcreate uninstalled"
else
    echo "cmdcreate not installed, nothing to do"
fi
