#!/usr/bin/env bash

set -euo pipefail

echo -e "\n> Cleaning up Cargo..."

cargo clean

echo -e "\n> Cleaning up Python cache files..."

rm -rf ./testing/features/__pycache__/
rm -rf ./testing/features/scripts/__pycache__/

./dev/format.sh

echo -e "\n> Cleaned up environment!"
