import os
from lib.sys import command


def test():
    os.system("clear")

    # Intro
    print("\nRunning tests: Rename command\n")
    desc = [
        'Starting command name: "test_command"',
        'Renamed command name: "renamed_test_command"\n',
        'Command contents: "echo Test succeeded"',
    ]
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the command
    print("Creating command...")
    command('cargo run create test_command "echo Rename succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Rename the created command
    print("\nRenaming created command...\n\n")
    command("cargo run rename test_command renamed_test_command")
    input("\n\nPress enter to continue...")

    # Run renamed command
    print("\nRunning renamed command...\n\n")
    command("renamed_test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up...\n")
    command("cargo run remove renamed_test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
