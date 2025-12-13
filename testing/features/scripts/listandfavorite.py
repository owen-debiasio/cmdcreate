import os
from lib.sys import command
from shared import (
    create_multiple_commands,
    delete_multiple_commands,
    list_commands,
)


# This test script verifies the full command listing & favoriting workflow.
# It performs the following steps:
# 1. Shows a preview of the commands that will be created
# 2. Creates three commands using your Rust CLI
# 3. Lists them
# 4. Marks one of them as a favorite
# 5. Lists them again to verify the change
# 6. Removes the favorite status
# 7. Lists again
# 8. Removes all test commands
# 9. Lists again to confirm cleanup
def test():
    os.system("clear")

    # Intro section
    print("\nRunning tests: Command listing\n")
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

    # Step 1: Create commands
    print("Creating commands...")
    create_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 2: List commands
    print("Listing commands...")
    list_commands()

    # Step 3: Add a command as favorite
    print("Adding command as favorite...")
    command("cargo run favorite add test_command")
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 4: List again to verify favorite flag
    print("\nListing commands again...\n")
    list_commands()

    # Step 5: Remove favorite status
    command("cargo run favorite remove test_command")
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 6: List again to verify removal
    print("\nListing commands again...\n")
    list_commands()

    # Step 7: Cleanup — remove all test commands
    print("Removing/cleaning up...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    # Step 8: Final list to confirm everything is gone
    print("\nListing commands again...\n")
    list_commands()

    os.system("clear")
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
