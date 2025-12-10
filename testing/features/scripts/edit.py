import os
from lib.sys import command
from features.shared import create_single_command, delete_single_command

# This test script verifies the "edit" functionality of your custom command system.
# It performs the following steps:
# 1. Clears the screen and shows test info
# 2. Creates a test command using your Rust CLI
# 3. Opens the editor to modify that command
# 4. Cleans up by deleting the command afterward
# 5. Uses pauses so users can follow each stage while running interactively


def test():
    os.system("clear")

    # Intro section
    print("\nRunning tests: Command editing\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # Step 1: Create the command using your tool
    print("\nCreating command...\n")
    create_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # Step 2: Edit the command (this will open the editor configured in your CLI)
    print("\nEditing created command...\n")
    command("cargo run edit test_command")
    input("\nPress enter to continue...")

    os.system("clear")

    # Step 3: Cleanup — remove the test command
    print("Cleaning up...\n")
    delete_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # Final message
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
