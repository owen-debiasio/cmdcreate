from testing.features.lib import command
from testing.features.shared import create_single_command, test_completed, enter_to_continue


def test():
    command("clear")

    print("\nRunning tests: Rename command\n")
    desc = [
        'Starting command name: "test_command"',
        'Renamed command name: "renamed_test_command"\n',
        'Command contents: "echo Test succeeded"',
    ]
    for line in desc:
        print(line)

    enter_to_continue()

    print("Creating command...")
    create_single_command()

    enter_to_continue()

    print("\nRenaming created command...\n\n")
    command("cargo run rename test_command renamed_test_command")

    enter_to_continue()

    print("\nRunning renamed command...\n\n")
    command("renamed_test_command")

    enter_to_continue()

    print("Cleaning up...\n")
    command("cargo run remove renamed_test_command -f")

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test()
