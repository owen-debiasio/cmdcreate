import os
from lib.sys import command


def test():
    os.system("clear")

    # Intro
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

    # Creating the command
    print("Creating command for first test...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Removing created command binary
    print("\nRemoving created command binary...\n\n")
    command("sudo rm /usr/bin/test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Repairing command
    print("\nRepairing command...\n\n")
    command("cargo run repair")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Run repaired command
    print("\nRunning repaired command...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up for next test...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the commands for test 2
    print("Creating commands for test 2...")
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Run repaired command
    print("\nRunning repaired commands...\n\n")
    command("test_command2")
    command("test_command3")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Removing created command binaries of test commands 2, 3
    print("\nRemoving created command binaries of test commands 2, 3...\n\n")
    command("sudo rm /usr/bin/test_command2")
    command("sudo rm /usr/bin/test_command3")
    input("\nPress enter to continue...")

    os.system("clear")

    # Repairing commands
    print("\nRepairing commands...\n\n")
    command("cargo run repair")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up...\n")
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
