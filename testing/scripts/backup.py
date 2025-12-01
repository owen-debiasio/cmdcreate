import os
from lib.sys import command
from lib.path import home


def test():
    os.system("clear")

    # Intro
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

    # Creating the command
    print("Creating command for test 1...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Export command
    print("\nExporting command to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up for a clean slate...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # Import command
    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # List commands
    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up for test 2...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the commands
    print("Creating test 2 commands...")
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Export commands
    print("\nExporting commands to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up for a clean slate...\n")
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # Import command
    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # List commands
    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up...\n")
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")
    command(f"rm {home}/.cache/export.cmdcreate")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
