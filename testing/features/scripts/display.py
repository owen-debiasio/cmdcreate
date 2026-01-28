from click import command

from features.shared import create_single_command, delete_single_command


def test():
    command("clear")

    # Intro section
    print("\nRunning tests: Display command contents\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    command("clear")

    print("Creating command...")
    create_single_command()
    input("\nPress enter to continue...")

    command("clear")

    print("\nDisplaying command contents...\n\n")
    command("cargo run display test_command")
    input("\n\nPress enter to continue...")

    command("clear")

    print("Cleaning up...\n")
    delete_single_command()
    input("\nPress enter to continue...")

    command("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
