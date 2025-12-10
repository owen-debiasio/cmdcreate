import os
from lib.sys import command
from features.shared import create_single_command, delete_single_command

# This test script verifies the "display" functionality of your command system.
# It does the following:
# 1. Clears the screen and shows test info
# 2. Creates a test command using your Rust CLI
# 3. Displays the contents of the created command
# 4. Cleans up by removing the test command
# 5. Pauses between steps for easier visibility while running interactively


def test():
    os.system("clear")

    # Intro section
    print("\nRunning tests: Display command contents\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # Step 1: Create the command using your CLI
    print("Creating command...")
    create_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # Step 2: Display the created command's contents
    print("\nDisplaying command contents...\n\n")
    command("cargo run display test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Step 3: Cleanup
    print("Cleaning up...\n")
    delete_single_command()
    input("\nPress enter to continue...")

    os.system("clear")

    # Final message
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
