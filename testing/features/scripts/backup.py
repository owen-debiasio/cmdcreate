from features.lib import home, command
from features.shared import delete_multiple_commands, create_multiple_commands


def test():
    command("clear")

    print("\nRunning tests: Command importing/exporting\n")
    desc = [
        "Command 1:",
        "    name: test_command",
        '    contents: "echo Test succeeded"',
        "    run in test(s): 1, 2",
        "Command 2:",
        "    name: test_command2",
        '    contents: "echo Test succeeded 2"',
        "    run in test(s): 2",
        "Command 3:",
        "    name: test_command3",
        '    contents: "echo Test succeeded 3"',
        "    run in test(s): 2",
        "\nOutput path: ~/.cache/export.cmdcreate",
    ]
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    command("clear")

    print("Creating command for test 1...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    command("clear")

    print("\nExporting command to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    command("clear")

    print("Cleaning up for a clean slate...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    command("clear")

    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    command("clear")

    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    command("clear")

    print("Cleaning up for test 2...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    command("clear")

    print("Creating test 2 commands...")
    create_multiple_commands()
    input("\nPress enter to continue...")

    command("clear")

    print("\nExporting commands to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    command("clear")

    print("Cleaning up for a clean slate...\n")
    delete_multiple_commands()
    input("\nPress enter to continue...")

    command("clear")

    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")
    input("\n\nPress enter to continue...")

    command("clear")

    print("\nListing commands...\n\n")
    command("test_command")
    input("\n\nPress enter to continue...")

    command("clear")

    print("Cleaning up...\n")
    delete_multiple_commands()
    command(f"rm {home}/.cache/export.cmdcreate")
    input("\nPress enter to continue...")

    command("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
