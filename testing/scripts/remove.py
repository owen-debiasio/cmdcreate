import os
from lib.sys import command


def test(forced):
    os.system("clear")

    # Intro
    print("\nRunning tests: Command deletion\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    if forced:
        print("Forced")

    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the command
    print("\nCreating command...\n")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Remove created command
    print("\nRemoving created command...\n")
    if forced:
        command("cargo run remove test_command -f")
    else:
        command("cargo run remove test_command")

    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test(forced=False)
