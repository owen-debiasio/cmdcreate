from testing.features.lib import command
from testing.features.shared import create_single_command, delete_single_command


def test():
    command("clear")

    print("\nRunning tests: Command editing\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    command("clear")

    print("\nCreating command...\n")
    create_single_command()
    input("\nPress enter to continue...")

    command("clear")

    print("\nEditing created command...\n")
    command("cargo run edit test_command")
    input("\nPress enter to continue...")

    command("clear")

    print("Cleaning up...\n")
    delete_single_command()
    input("\nPress enter to continue...")

    command("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
