import os
from lib.sys import command


def list_commands():
    command("cargo run list")
    input("\nPress enter to continue...")

    os.system("clear")


def test():
    os.system("clear")

    # Intro
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

    # Creating the commands
    print("Creating commands...")
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Listing the commands
    print("Listing commands...")
    list_commands()

    # Add command as favorite
    print("Adding command as favorite...")
    command("cargo run favorite add test_command")
    input("\nPress enter to continue...")

    os.system("clear")

    # List commands again
    print("\nListing commands again...\n")
    list_commands()

    # Remove command as favorite
    command("cargo run favorite remove test_command")
    input("\nPress enter to continue...")

    os.system("clear")

    # List commands again
    print("\nListing commands again...\n")
    list_commands()

    # Cleanup
    print("Removing/cleaning up...\n")
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # List commands again
    print("\nListing commands again...\n")
    list_commands()

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
