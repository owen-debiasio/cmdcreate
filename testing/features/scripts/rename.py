import os
from lib.sys import command
from shared import create_single_command


# This test covers command renaming.
# It verifies:
# - Creating a command
# - Renaming it to a new identifier
# - Running the renamed version to ensure it works
# - Cleaning up afterward
def test():
    os.system("clear")

    # Intro section
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

    # Step 1: Create the command
    print("Creating command...")
    create_single_command()
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 2: Rename the command
    print("\nRenaming created command...\n\n")
    command("cargo run rename test_command renamed_test_command")
    input("\n\nPress enter to continue...")

    # Step 3: Run renamed command to verify it works
    print("\nRunning renamed command...\n\n")
    command("renamed_test_command")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Step 4: Cleanup
    print("Cleaning up...\n")
    command("cargo run remove renamed_test_command -f")
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
