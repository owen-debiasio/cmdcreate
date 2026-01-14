from lib.sys import command
from shared import create_single_command


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

    input("\nPress enter to continue...")
    command("clear")

    print("Creating command...")
    create_single_command()
    input("\nPress enter to continue...")
    command("clear")

    print("\nRenaming created command...\n\n")
    command("cargo run rename test_command renamed_test_command")
    input("\n\nPress enter to continue...")

    print("\nRunning renamed command...\n\n")
    command("renamed_test_command")
    input("\n\nPress enter to continue...")
    command("clear")

    print("Cleaning up...\n")
    command("cargo run remove renamed_test_command -f")
    input("\nPress enter to continue...")
    command("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
