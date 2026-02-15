from testing.lib import command
from testing.shared import enter_to_continue, simple_init, simple_cleanup


def test():
    command("clear")

    print("\nRunning tests: Command creation\n")

    simple_init()

    print("\nRunning created command...\n\n")
    command("test_command")

    enter_to_continue()

    simple_cleanup()


if __name__ == "__main__":
    test()
