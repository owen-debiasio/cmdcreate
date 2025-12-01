import os
from lib.sys import command


def test():
    os.system("clear")

    # Intro
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

    # Creating the commands
    print("Creating commands...\n")
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Search for command
    print('Searching for command that contains "2"...\n')
    command("cargo run search 2")
    input("\nPress enter to continue...")

    os.system("clear")

    # Search for command that doesn't exists
    print('Searching for no matches using "none"...\n')
    command("cargo run search none")
    input("\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Removing/cleaning up...\n")
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
