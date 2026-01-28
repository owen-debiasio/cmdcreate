from features.lib import command
from features.shared import create_single_command, delete_single_command


def test(forced):
    command("clear")

    print("\nRunning tests: Command deletion\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    if forced:
        print("Forced mode enabled (-f)")

    input("\nPress enter to continue...")
    command("clear")

    print("\nCreating command...\n")
    create_single_command()
    input("\nPress enter to continue...")
    command("clear")

    print("\nRemoving created command...\n")
    if forced:
        delete_single_command()
    else:
        command("cargo run remove test_command")

    input("\nPress enter to continue...")
    command("clear")

    # End of test
    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test(forced=False)
