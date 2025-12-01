import os
from lib.sys import command


def test():
    os.system("clear")

    # Intro
    print("\nRunning tests: Display command contents\n")
    desc = ['Command name: "test_command"', 'Command contents: "echo Test succeeded"']
    for line in desc:
        print(line)

    input("\nPress enter to continue...")

    os.system("clear")

    # Creating the command
    print("Creating command...")
    command('cargo run create test_command "echo Test succeeded"')
    input("\nPress enter to continue...")

    os.system("clear")

    # Display the created command
    print("\nDisplaying command contents...\n\n")
    command("cargo run display test_command")
    input("\n\nPress enter to continue...")

    os.system("clear")

    # Cleanup
    print("Cleaning up...\n")
    command("cargo run remove test_command -f")
    input("\nPress enter to continue...")

    os.system("clear")

    print("\nCommand test completed.\n")
    input("Press enter to continue...")


if __name__ == "__main__":
    test()
