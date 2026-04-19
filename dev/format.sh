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

set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
RESET='\033[0m'

cd "$ROOT_DIR"

echo -e "\n${BLUE}> Formatting cmdcreate...${RESET}"

command -v cargo > /dev/null || {
    echo -e "\n${RED}> cargo not found${RESET}"
    exit 1
}
command -v black > /dev/null || {
    echo -e "\n${RED}> black not found${RESET}"
    exit 1
}
command -v shfmt > /dev/null || {
    echo -e "\n${RED}> shfmt not found${RESET}"
    exit 1
}
command -v shellcheck > /dev/null || {
    echo -e "\n${RED}> shellcheck not found${RESET}"
    exit 1
}
command -v pylint > /dev/null || {
    echo -e "\n${RED}> pylint not found${RESET}"
    exit 1
}
command -v markdownlint-cli2 > /dev/null || {
    echo -e "\n${RED}> markdownlint-cli2 not found${RESET}"
    exit 1
}

echo -e "\n${BLUE}> Formatting Rust source...${RESET}"
cargo fmt --all --

if [[ -d "$ROOT_DIR/testing/" ]]; then
    echo -e "\n${BLUE}> Formatting Python testing scripts...${RESET}"
    black "$ROOT_DIR/testing/"

    echo -e "\n${BLUE}> Linting Python testing scripts...${RESET}"
    find . -name "*.py" -exec pylint {} +
else
    echo -e "\n${YELLOW}> Skipping Python formatting (testing/features not found)${RESET}"
fi

echo -e "\n${BLUE}> Linting shell scripts...${RESET}"
find . -name "*.sh" -exec shellcheck {} +

echo -e "\n${BLUE}> Formatting shell scripts...${RESET}"
shfmt -w -i 4 -ci -sr "$ROOT_DIR"

echo -e "\n${BLUE}> Linting and formatting markdown files...${RESET}"
markdownlint-cli2 "**/*.md" --config .markdownlint.json
prettier -w "**/*.md"

echo -e "\n${GREEN}> Formatting complete!${RESET}"
