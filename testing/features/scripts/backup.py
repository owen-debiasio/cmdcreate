import os
from lib.sys import command
from lib.path import home
from shared import create_multiple_commands, delete_multiple_commands

"""
Testing suite for cmdcreate import/export functionality.

This script automates verification of:
1. Exporting a single command
2. Re-importing the exported file
3. Exporting multiple commands
4. Re-importing the multi-command export
5. Ensuring cleanup/restore flows work properly

Each step pauses for user interaction to allow inspection during testing.
"""


def test():
    """Run the full import/export test workflow."""
    os.system("clear")

    # Intro describing exactly what the tests will create and validate
    print("\nRunning tests: Command importing/exporting\n")
    desc = [
        "Command 1:",
        "    name: test_command",
        '    contents: "echo Test succeeded"',
        "    run in test(s): 1, 2",
        "Command 2:",
        "    name: test_command2",
        '    contents: "echo Test succeeded 2"',
        "    run in test(s): 2",
        "Command 3:",
        "    name: test_command3",
        '    contents: "echo Test succeeded 3"',
        "    run in test(s): 2",
        "\nOutput path: ~/.cache/export.cmdcreate",
    ]
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # ---------- Test 1: Export & import a single command ----------
    print("Creating command for test 1...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nExporting command to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup before import
    print("Cleaning up for a clean slate...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # Import back
    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Validate import
    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # ---------- Test 2: Multi-command export/import ----------
    print("Cleaning up for test 2...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("Creating test 2 commands...")
    create_multiple_commands()
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nExporting commands to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup for clean import
    print("Cleaning up for a clean slate...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")

    os.system("clear")

    # Import multi-command export
    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Validate import
    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Final cleanup
    print("Cleaning up...\n")
    delete_multiple_commands()
    command(f"rm {home}/.cache/export.cmdcreate")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
