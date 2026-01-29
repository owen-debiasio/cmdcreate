from testing.features.lib import command
from testing.features.shared import enter_to_continue, \
     simple_cleanup, simple_init


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
