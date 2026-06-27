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
RESET='\033[0m'
YELLOW='\033[0;33m'

cd "$ROOT_DIR"

echo -e "\n${BLUE}> Formatting cmdcreate...${RESET}"

command -v cargo > /dev/null || {
    echo -e "\n${RED}> cargo not found${RESET}"
    exit 1
}

echo -e "\n${BLUE}> Linting and formatting cmdcreate...${RESET}"
cargo fmt --all --
cargo clippy

echo -e "\n${BLUE}> Linting shell scripts...${RESET}"
if command -v shellcheck &> /dev/null; then
    find . -name "*.sh" -exec shellcheck {} +
else
    echo -e "\n${YELLOW}> shellcheck not found... ignore if not in a development environment!${RESET}"
fi

echo -e "\n${BLUE}> Formatting shell scripts...${RESET}"
if command -v shfmt &> /dev/null; then
    shfmt -w -i 4 -ci -sr "$ROOT_DIR"
else
    echo -e "\n${YELLOW}> shfmt not found... ignore if not in a development environment!${RESET}"
fi

echo -e "\n${BLUE}> Linting and formatting markdown and yaml files...${RESET}"

echo -e "\n${BLUE}> Running Markdownlint...${RESET}"
if command -v markdownlint-cli2 &> /dev/null; then
    markdownlint-cli2 "**/*.md" "**/*.yml" --config .markdownlint.json --fix
else
    echo -e "\n${YELLOW}> markdownlint-cli2 not found... ignore if not in a development environment!${RESET}"
fi

echo -e "\n${BLUE}> Running Prettier...${RESET}"
if command -v prettier &> /dev/null; then
    prettier --config .prettierrc -w .
else
    echo -e "\n${YELLOW}> prettier not found... ignore if not in a development environment!${RESET}"
fi

echo -e "\n${GREEN}> Formatting complete!${RESET}"
