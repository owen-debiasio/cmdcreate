from testing.features.lib import command
from testing.features.shared import (
    multiple_command_desc,
    create_multiple_commands,
    enter_to_continue,
    bigger_cleanup,
)


def test():
    command("clear")

    print("\nRunning tests: Searching for command\n")
    for line in multiple_command_desc():
        print(line)

    input("\nPress enter to continue...")
    command("clear")

    print("Creating commands...\n")
    create_multiple_commands()

    enter_to_continue()

    print('Searching for command that contains "2"...\n')
    command("cargo run search 2")

    enter_to_continue()

    print('Searching for no matches using "none"...\n')
    command("cargo run search none")

    enter_to_continue()

    bigger_cleanup()


if __name__ == "__main__":
    test()
