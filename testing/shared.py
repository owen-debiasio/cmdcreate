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

from lib import command


def enter_to_continue():
    input("\n\nPress enter to continue...")
    command("clear")


def test_completed():
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


def create_single_command():
    command('cargo run create test_command "echo Test succeeded"')


def delete_single_command():
    command("cargo run remove test_command -f")


def create_multiple_commands():
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')


def delete_multiple_commands():
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")


def list_commands():
    command("cargo run list")
    input("\nPress enter to continue...")
    command("clear")


def multiple_command_desc() -> list:
    var = [
        "Command 1:",
        "    name: test_command",
        '    contents: "echo Test succeeded"',
        "Command 2:",
        "    name: test_command2",
        '    contents: "echo Test succeeded 2"',
        "Command 3:",
        "    name: test_command3",
        '    contents: "echo Test succeeded 3"',
    ]

    return var


def single_command_desc() -> list:
    var = [
        'Command name: "test_command"',
        'Command contents: "echo Test succeeded"',
    ]

    return var


def simple_cleanup():
    print("Cleaning up...\n")
    delete_single_command()

    enter_to_continue()

    test_completed()


def bigger_cleanup():
    print("Cleaning up...\n")
    delete_multiple_commands()

    enter_to_continue()

    test_completed()


def simple_init():
    for line in single_command_desc():
        print(line)

    enter_to_continue()

    print("Creating command...")
    create_single_command()

    enter_to_continue()
