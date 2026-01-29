from testing.features.lib import command

def enter_to_continue():
    input("\n\nPress enter to continue...")
    command("clear")

def test_completed():
    print("\nCommand test completed.\n")
    input("Press enter to continue...")

def create_single_command():
    command('cargo run create test_command "echo Test succeeded"')


def delete_single_command():
    command("cargo run remove test_command -f")


def create_multiple_commands():
    command('cargo run create test_command "echo Test succeeded"')
    command('cargo run create test_command2 "echo Test succeeded 2"')
    command('cargo run create test_command3 "echo Test succeeded 3"')


def delete_multiple_commands():
    command("cargo run remove test_command -f")
    command("cargo run remove test_command2 -f")
    command("cargo run remove test_command3 -f")


def list_commands():
    command("cargo run list")
    input("\nPress enter to continue...")
    command("clear")

def multiple_command_desc() -> list:
    var = [
        "Command 1:",
        "    name: test_command",
        '    contents: "echo Test succeeded"',
        "Command 2:",
        "    name: test_command2",
        '    contents: "echo Test succeeded 2"',
        "Command 3:",
        "    name: test_command3",
        '    contents: "echo Test succeeded 3"',

    ]

    return var

def single_command_desc() -> list:
    var = [
        'Command name: "test_command"',
        'Command contents: "echo Test succeeded"',
    ]

    return var
