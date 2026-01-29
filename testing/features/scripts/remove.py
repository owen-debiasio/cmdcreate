from testing.features.lib import command
from testing.features.shared import create_single_command, delete_single_command, test_completed, enter_to_continue


def test(forced):
    command("clear")

    print("\nRunning tests: Command deletion\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    if forced:
        print("Forced mode enabled (-f)")

    enter_to_continue()

    print("\nCreating command...\n")
    create_single_command()

    enter_to_continue()

    print("\nRemoving created command...\n")
    if forced:
        delete_single_command()
    else:
        command("cargo run remove test_command")

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test(forced=False)
