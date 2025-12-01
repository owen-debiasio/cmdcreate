import os
from lib.sys import command


def test():
    os.system("clear")

    # Intro
    print("\nRunning tests: Command editing\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the command
    print("\nCreating command...\n")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Remove created command
    print("\nEditing created command...\n")
    command("cargo run edit test_command")
    input("\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
