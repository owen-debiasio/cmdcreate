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
command -v shfmt > /dev/null || {
    echo -e "\n${YELLOW}> shfmt not found... ignore if not in a development environment!${RESET}"
}
command -v shellcheck > /dev/null || {
    echo -e "\n${YELLOW}> shellcheck not found... ignore if not in a development environment!${RESET}"
}
command -v markdownlint-cli2 > /dev/null || {
    echo -e "\n${YELLOW}> markdownlint-cli2 not found... ignore if not in a development environment!${RESET}"
    exit 1
}
command -v prettier > /dev/null || {
    echo -e "\n${YELLOW}> prettier not found... ignore if not in a development environment!${RESET}"
    exit 1
}

echo -e "\n${BLUE}> Linting and formatting cmdcreate...${RESET}"
cargo fmt --all --
cargo clippy

echo -e "\n${BLUE}> Linting shell scripts...${RESET}"
find . -name "*.sh" -exec shellcheck {} +

echo -e "\n${BLUE}> Formatting shell scripts...${RESET}"
shfmt -w -i 4 -ci -sr "$ROOT_DIR"

echo -e "\n${BLUE}> Linting and formatting markdown and yaml files...${RESET}"
markdownlint-cli2 "**/*.md" "**/*.yml" --config .markdownlint.json --fix
prettier --config .prettierrc -w .

echo -e "\n${GREEN}> Formatting complete!${RESET}"
