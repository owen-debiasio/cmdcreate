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

from io_utils import output, error
from sys_utils import return_args


def list_tests() -> None:
    available_options = [
        "create",
        "remove",
        "edit",
        "list",
        "search",
        "display",
        "rename",
        "favorite",
        "config",
        "doc",
    ]

    output("Available tests:\n", enable_arrow=True)

    for option in available_options:
        output(option, enable_arrow=False)


def list_subcommands() -> None:
    available_options = [
        "list",
        "run",
    ]

    output("Available Subcommands:\n", enable_arrow=True)

    for option in available_options:
        output(option, enable_arrow=False)


def init_tests() -> None:
    given_arguments: list = return_args()

    if len(given_arguments) <= 1:
        list_subcommands()

        exit(0)

    root_command: str = given_arguments[1]

    match root_command:
        case "list":
            list_tests()
        case _:
            error(f"Invalid command: {root_command}")
