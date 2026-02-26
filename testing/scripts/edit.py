# -*- coding: utf-8 -*-

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

from testing.lib import command
from testing.shared import (
    create_single_command,
    delete_single_command,
    test_completed,
    enter_to_continue,
)


def test():
    command("clear")

    print("\nRunning tests: Command editing\n")
    for line in [
        'Command name: "test_command"',
        'Command contents: "echo Test succeeded"',
    ]:
        print(line)

    enter_to_continue()

    print("\nCreating command...\n")
    create_single_command()

    enter_to_continue()

    print("\nEditing created command...\n")
    command("cargo run edit test_command")

    enter_to_continue()

    print("Cleaning up...\n")
    delete_single_command()

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test()
