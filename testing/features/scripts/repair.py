import os
from lib.sys import command
from shared import (
    create_multiple_commands,
    delete_multiple_commands,
    delete_single_command,
)


# This test covers command repairing.
# It verifies:
# - Detecting missing binaries
# - Automatically rebuilding commands via `repair`
# - Running repaired commands to ensure functionality
# - Running multi-command repair scenarios
# - Cleaning up after each test stage
def test():
    os.system("clear")

    # Intro section
    print("\nRunning tests: Command repairing\n")
    desc = [
        "Command 1:",
        "    name: test_command",
        '    contents: "echo Test succeeded"',
        "    run in test(s): 1, 2",
        "Command 2:",
        "    name: test_command2",
        '    contents: "echo Test succeeded 2"',
        "    run in test(s): 2",
        "Command 3:",
        "    name: test_command3",
        '    contents: "echo Test succeeded 3"',
        "    run in test(s): 2",
    ]
    for line in desc:
        print(line)

    input("\nPress enter to continue...")
    os.system("clear")

    # TEST 1 — Single command repair
    print("Creating command for first test...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")
    os.system("clear")

    # Simulate missing binary
    print("\nRemoving created command binary...\n\n")
    command("sudo rm /usr/bin/test_command")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Repair attempt
    print("\nRepairing command...\n\n")
    command("cargo run repair")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Verify repair
    print("\nRunning repaired command...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Cleanup
    print("Cleaning up for next test...\n")
    delete_single_command()
    input("\nPress enter to continue...")
    os.system("clear")

    # TEST 2 — Multi-command repair
    print("Creating commands for test 2...")
    create_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    # Run to confirm working pre-repair
    print("\nRunning repaired commands...\n\n")
    command("test_command2")
    command("test_command3")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Remove two binaries to simulate corruption
    print("\nRemoving created command binaries of test commands 2, 3...\n\n")
    command("sudo rm /usr/bin/test_command2")
    command("sudo rm /usr/bin/test_command3")
    input("\nPress enter to continue...")
    os.system("clear")

    # Repair attempt
    print("\nRepairing commands...\n\n")
    command("cargo run repair")
    input("\n\nPress enter to continue...")
    os.system("clear")

    # Cleanup all commands
    print("Cleaning up...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")
    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
