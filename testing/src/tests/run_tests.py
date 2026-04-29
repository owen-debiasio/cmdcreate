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

from operator import contains

from colors import Colors
from io_utils import error, output
from sys_utils import run_shell_command, running_as_root

available_tests: list = [
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
    "check",
    "import",
    "export",
]


def run_test(test_to_run: str):
    if not running_as_root:
        error("Please run the testing script as root to run tests!")

    if not contains(available_tests, test_to_run):
        error(f"Not a valid test name: {test_to_run}")

    match test_to_run:
        case "create":
            output(f'Running test {Colors.magenta}"create"', enable_arrow=True)
            if run_shell_command('cmdcreate create test -o "echo hi"'):
                output("Success", enable_arrow=True)
            else:
                error("Test failed!")
