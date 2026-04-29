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
from colors import Colors
from tests.init_tests import init_tests

VERSION = "v0.2.0"


def list_available_commands():
    available_options = [
        "list",
        "run",
    ]

    output("Available Commands:\n", enable_arrow=True)

    for option in available_options:
        output(option, enable_arrow=False)

    print()


def main():
    output(f"cmdcreate Testing Suite {VERSION}\n", enable_arrow=True)

    output(
        f"{Colors.yellow}UNFINISHED, STILL CURRENTLY BEING WORKED ON! PLEASE BE MINDFUL!",
        enable_arrow=True,
    )

    retrieved_args = return_args()

    if len(retrieved_args) == 0:
        list_available_commands()

        exit(1)

    root_command = retrieved_args[0]

    match root_command:
        case "test":
            init_tests()

        case _:
            error(f"Invalid option: {root_command}")


if __name__ == "__main__":
    main()
