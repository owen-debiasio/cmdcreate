from testing.features.lib import command, home
from testing.features.shared import multiple_command_desc, create_multiple_commands, delete_multiple_commands


def test():
    command("clear")

    print("\nRunning tests: Command importing/exporting\n")
    desc = multiple_command_desc(), "\nOutput path: ~/.cache/export.cmdcreate"

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
    import_fn()

    print("Cleaning up...\n")
    delete_multiple_commands()
    command(f"rm {home}/.cache/export.cmdcreate")
    input("\nPress enter to continue...")

    command("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")

def import_fn():
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

if __name__ == "__main__":
    test()
