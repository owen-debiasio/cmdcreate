import os
from lib.sys import command
from features.shared import create_single_command, delete_single_command

"""
Testing suite for cmdcreate's command creation feature.

This script walks through:
1. Creating a new command
2. Executing the created command to confirm success
3. Cleaning up afterwards

Each step pauses for user confirmation so the tester can inspect outputs.
"""


def test():
    """Run the full command creation test workflow."""
    os.system("clear")

    # Description of what will be tested
    print("\nRunning tests: Command creation\n")
    desc = [
        'Command name: "test_command"',
        'Command contents: "echo Test succeeded"',
    ]
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # --- Step 1: Create the command ---
    print("Creating command...")
    create_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # --- Step 2: Execute the command to verify functionality ---
    print("\nRunning created command...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # --- Step 3: Cleanup to restore a clean testing environment ---
    print("Cleaning up...\n")
    delete_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # Test complete
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
