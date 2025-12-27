import os
from lib.sys import command


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
    os.system("clear")
