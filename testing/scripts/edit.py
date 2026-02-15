from testing.lib import command
from testing.shared import (
    create_single_command,
    delete_single_command,
    test_completed,
    enter_to_continue,
)


def test():
    command("clear")

    print("\nRunning tests: Command editing\n")
    for line in [
        'Command name: "test_command"',
        'Command contents: "echo Test succeeded"',
    ]:
        print(line)

    enter_to_continue()

    print("\nCreating command...\n")
    create_single_command()

    enter_to_continue()

    print("\nEditing created command...\n")
    command("cargo run edit test_command")

    enter_to_continue()

    print("Cleaning up...\n")
    delete_single_command()

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test()
