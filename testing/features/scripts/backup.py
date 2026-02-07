from testing.features.lib import command, home
from testing.features.shared import (
    multiple_command_desc,
    create_multiple_commands,
    delete_multiple_commands,
    test_completed,
    enter_to_continue,
)


def test():
    command("clear")

    print("\nRunning tests: Command importing/exporting\n")

    for line in [multiple_command_desc(), "\nOutput path: ~/.cache/export.cmdcreate"]:
        print(line)

    enter_to_continue()

    print("Creating command for test 1...")
    command('cargo run create test_command "echo Test succeeded"')

    enter_to_continue()

    print("\nExporting command to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")

    enter_to_continue()

    print("Cleaning up for a clean slate...\n")
    command("cargo run remove test_command -f")

    print("Cleaning up for test 2...\n")
    command("cargo run remove test_command -f")

    enter_to_continue()

    print("Creating test 2 commands...")
    create_multiple_commands()

    enter_to_continue()

    print("\nExporting commands to ~/.cache/export.cmdcreate...\n\n")
    command(f"cargo run export {home}/.cache/")
    input("\n\nPress enter to continue...")

    enter_to_continue()

    print("Cleaning up for a clean slate...\n")
    delete_multiple_commands()
    import_fn()

    print("Cleaning up...\n")
    delete_multiple_commands()
    command(f"rm {home}/.cache/export.cmdcreate")
    input("\nPress enter to continue...")

    enter_to_continue()

    test_completed()


def import_fn():
    enter_to_continue()

    print("\nImporting exported file...\n\n")
    command(f"cargo run import {home}/.cache/export.cmdcreate")

    enter_to_continue()

    print("\nListing commands...\n\n")
    command("test_command")

    enter_to_continue()


if __name__ == "__main__":
    test()
