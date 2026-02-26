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

cd "$ROOT_DIR"

echo -e "\n> Formatting cmdcreate..."

command -v cargo > /dev/null || {
    echo -e "\n> cargo not found"
    exit 1
}
command -v black > /dev/null || {
    echo -e "\n> black not found"
    exit 1
}
command -v shfmt > /dev/null || {
    echo -e "\n> shfmt not found"
    exit 1
}
command -v shellcheck > /dev/null || {
    echo -e "\n> shellcheck not found"
    exit 1
}
command -v pylint > /dev/null || {
    echo -e "\n> pylint not found"
    exit 1
}
command -v markdownlint-cli2 > /dev/null || {
    echo -e "\n> markdownlint-cli2 not found"
    exit 1
}

echo -e "\n> Formatting Rust source..."
cargo fmt --all --

if [[ -d "$ROOT_DIR/testing/" ]]; then
    echo -e "\n> Formatting Python testing scripts..."
    black "$ROOT_DIR/testing/"

    echo -e "\n> Linting Python testing scripts..."
    find . -name "*.py" -exec pylint {} +
else
    echo -e "\n> Skipping Python formatting (testing/features not found)"
fi

echo -e "\n> Linting shell scripts..."
find . -name "*.sh" -exec shellcheck {} +

echo -e "\n> Formatting shell scripts..."
shfmt -w -i 4 -ci -sr "$ROOT_DIR"

echo -e "\n> Linting and formatting markdown files..."
markdownlint-cli2 "**/*.md" --config .markdownlint.json
prettier -w "**/*.md"

echo -e "\n> Formatting complete."
