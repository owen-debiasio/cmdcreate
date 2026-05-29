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

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
RESET='\033[0m'

[[ $# -eq 1 ]] || die -e "${RED}> Provide cmdcreate version (MUST NOT START WITH v)${RESET}"
[[ "$1" != v* ]] || die -e "${RED}> Version must NOT start with 'v'${RESET}"

git tag "v$1"
git push origin "v$1"

echo -e "\n${GREEN}> Update complete for cmdcreate.${RESET}"
