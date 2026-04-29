# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Owen Debiasio

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

from sys import exit
from colors import Colors


def output(text: str, enable_arrow: bool) -> None:
    arrow_to_use: str = ""

    if len(text) != 0 and enable_arrow:
        arrow_to_use = "> "

    print(f"{Colors.blue}{arrow_to_use}{text}{Colors.reset}")


def error(text: str) -> None:
    output(f"{Colors.red}{text}", enable_arrow=True)
    exit(1)
