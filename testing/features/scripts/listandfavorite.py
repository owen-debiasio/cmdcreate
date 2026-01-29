from testing.features.lib import command
from testing.features.shared import (
    create_multiple_commands,
    list_commands,
    delete_multiple_commands,
    multiple_command_desc,
    test_completed,
    enter_to_continue,
)


def test():
    command("clear")

    print("\nRunning tests: Command listing\n")
    for line in multiple_command_desc():
        print(line)

    enter_to_continue()

    print("Creating commands...")
    create_multiple_commands()

    enter_to_continue()

    print("Listing commands...")
    list_commands()

    print("Adding command as favorite...")
    command("cargo run favorite add test_command")

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    command("cargo run favorite remove test_command")

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    print("Removing/cleaning up...\n")
    delete_multiple_commands()

    enter_to_continue()

    print("\nListing commands again...\n")
    list_commands()

    test_completed()


if __name__ == "__main__":
    test()
