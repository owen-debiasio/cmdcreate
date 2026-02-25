# -*- coding: utf-8 -*-

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
    create_multiple_commands,
    list_commands,
    delete_multiple_commands,
    multiple_command_desc,
    test_completed,
    enter_to_continue,
)


def test():
    command("clear")

    print("\nRunning tests: Command listing\n")
    for line in multiple_command_desc():
        print(line)

    enter_to_continue()

    print("Creating commands...")
    create_multiple_commands()

    enter_to_continue()

    print("Listing commands...")
    list_commands()

    print("Adding command as favorite...")
    command("cargo run favorite add test_command")

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    command("cargo run favorite remove test_command")

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    print("Removing/cleaning up...\n")
    delete_multiple_commands()

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    test_completed()


if __name__ == "__main__":
    test()
