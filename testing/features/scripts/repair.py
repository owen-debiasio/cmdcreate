from testing.features.lib import command
from testing.features.shared import delete_single_command, create_multiple_commands, delete_multiple_commands, \
    enter_to_continue, test_completed


def test():
    command("clear")

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

    enter_to_continue()

    print("Creating command for first test...")
    command('cargo run create test_command "echo Test succeeded"')

    enter_to_continue()

    print("\nRemoving created command binary...\n\n")
    command("sudo rm /usr/bin/test_command")

    enter_to_continue()

    print("\nRepairing command...\n\n")
    command("cargo run repair")

    enter_to_continue()

    print("\nRunning repaired command...\n\n")
    command("test_command")

    enter_to_continue()

    print("Cleaning up for next test...\n")
    delete_single_command()

    enter_to_continue()

    print("Creating commands for test 2...")
    create_multiple_commands()

    enter_to_continue()

    print("\nRunning repaired commands...\n\n")
    command("test_command2")
    command("test_command3")

    enter_to_continue()

    print("\nRemoving created command binaries of test commands 2, 3...\n\n")
    command("sudo rm /usr/bin/test_command2")
    command("sudo rm /usr/bin/test_command3")

    enter_to_continue()

    print("\nRepairing commands...\n\n")
    command("cargo run repair")

    enter_to_continue()

    print("Cleaning up...\n")
    delete_multiple_commands()

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test()
