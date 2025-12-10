import os
from lib.sys import command
from features.shared import create_multiple_commands, delete_multiple_commands


# This test covers the search system.
# It checks:
# - Creating multiple commands
# - Searching for commands using partial matches
# - Handling searches with zero results
# - Cleaning up all created test commands afterward
def test():
    os.system("clear")

    # Intro section
    print("\nRunning tests: Searching for command\n")
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

    # Create sample commands
    print("Creating commands...\n")
    create_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    # Search for commands using a valid term
    print('Searching for command that contains "2"...\n')
    command("cargo run search 2")
    input("\nPress enter to continue...")
    os.system("clear")

    # Search using a term with no matches
    print('Searching for no matches using "none"...\n')
    command("cargo run search none")
    input("\nPress enter to continue...")
    os.system("clear")

    # Cleanup all test commands
    print("Removing/cleaning up...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
