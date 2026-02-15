#!/usr/bin/env bash

set -euo pipefail

BIN="cmdcreate"
INSTALL_DIR="/usr/bin/"

if [ -f "$INSTALL_DIR/$BIN" ]; then
    sudo rm "$INSTALL_DIR/$BIN"
    echo -e "\n> cmdcreate uninstalled"
else
    echo -e "\n> cmdcreate not installed, nothing to do"
fi
