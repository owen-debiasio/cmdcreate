from testing.features.lib import command
from testing.features.shared import create_single_command, delete_single_command, test_completed, enter_to_continue, \
    single_command_desc


def test():
    command("clear")

    print("\nRunning tests: Display command contents\n")

    for line in single_command_desc():
        print(line)

    enter_to_continue()

    print("Creating command...")
    create_single_command()

    enter_to_continue()

    print("\nDisplaying command contents...\n\n")
    command("cargo run display test_command")

    enter_to_continue()

    print("Cleaning up...\n")
    delete_single_command()

    enter_to_continue()

    test_completed()


if __name__ == "__main__":
    test()
