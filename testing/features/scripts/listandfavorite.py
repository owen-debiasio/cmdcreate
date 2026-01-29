from testing.features.lib import command
from testing.features.shared import create_multiple_commands, list_commands, delete_multiple_commands, \
    multiple_command_desc


def test():
    command("clear")

    print("\nRunning tests: Command listing\n")
    for line in multiple_command_desc():
        print(line)

    input("\nPress enter to continue...")
    command("clear")

    print("Creating commands...")
    create_multiple_commands()
    input("\nPress enter to continue...")
    command("clear")

    print("Listing commands...")
    list_commands()

    print("Adding command as favorite...")
    command("cargo run favorite add test_command")
    input("\nPress enter to continue...")
    command("clear")

    print("\nListing commands again...\n")
    list_commands()

    command("cargo run favorite remove test_command")
    input("\nPress enter to continue...")
    command("clear")

    print("\nListing commands again...\n")
    list_commands()

    print("Removing/cleaning up...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")
    command("clear")

    print("\nListing commands again...\n")
    list_commands()

    command("clear")
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
