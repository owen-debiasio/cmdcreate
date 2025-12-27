import os
from lib.sys import command
from shared import (
    create_multiple_commands,
    delete_multiple_commands,
    list_commands,
)


def test():
    os.system("clear")

    print("\nRunning tests: Command listing\n")
    desc = [
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
    for line in desc:
        print(line)

    input("\nPress enter to continue...")
    os.system("clear")

    print("Creating commands...")
    create_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    print("Listing commands...")
    list_commands()

    print("Adding command as favorite...")
    command("cargo run favorite add test_command")
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nListing commands again...\n")
    list_commands()

    command("cargo run favorite remove test_command")
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nListing commands again...\n")
    list_commands()

    print("Removing/cleaning up...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nListing commands again...\n")
    list_commands()

    os.system("clear")
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
