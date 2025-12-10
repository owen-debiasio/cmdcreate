import os
from lib.sys import command
from features.shared import create_single_command, delete_single_command


# This test covers command deletion behavior.
# It verifies:
# - Creating a command
# - Removing it either normally or with the -f (forced) flag
def test(forced):
    os.system("clear")

    # Intro section
    print("\nRunning tests: Command deletion\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    if forced:
        print("Forced mode enabled (-f)")

    input("\nPress enter to continue...")
    os.system("clear")

    # Step 1: Create the command
    print("\nCreating command...\n")
    create_single_command()
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 2: Remove the command (forced or normal)
    print("\nRemoving created command...\n")
    if forced:
        delete_single_command()
    else:
        command("cargo run remove test_command")

    input("\nPress enter to continue...")
    os.system("clear")

    # End of test
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test(forced=False)
