import os
from lib.sys import command


# Create a single test command for testing
def create_single_command():
    command('cargo run create test_command "echo Test succeeded"')


# Delete the single test command
def delete_single_command():
    command("cargo run remove test_command -f")


# Create multiple test commands at once for testing
def create_multiple_commands():
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')


# Delete multiple test commands at once for cleanup
def delete_multiple_commands():
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")


# List currently installed commands
# Waits for user input before clearing the screen
def list_commands():
    command("cargo run list")
    input("\nPress enter to continue...")
    os.system("clear")
