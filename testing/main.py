#!/usr/bin/env python3
from lib import command
from scripts import (
    remove,
    create,
    listandfavorite,
    search,
    edit,
    display,
    rename,
    backup,
)

VERSION = "v0.1.8"


def main():
    command("clear")
    print(f"cmdcreate feature testing {VERSION}\n\nPick a feature to test:\n")

    options = [
        "1] Creating command",
        "2] Removing command",
        "3] Removing command (forced)",
        "4] Editing command",
        "5] Listing/favoriting command(s)",
        "6] Searching for commands",
        "7] Displaying command contents",
        "8] Rename command",
        "9] Importing/exporting command(s)\n",
        "10] All tests\n",
        "0]  Abort\n",
    ]

    for line in options:
        print(line)

    run_test(input("\nChoose: "))


def run_test(test):
    match test:
        case "0":
            print("Aborted.")
            return
        case "1":
            create.test()
        case "2":
            remove.test(False)
        case "3":
            remove.test(True)
        case "4":
            edit.test()
        case "5":
            listandfavorite.test()
        case "6":
            search.test()
        case "7":
            display.test()
        case "8":
            rename.test()
        case "9":
            backup.test()
        case "10":
            for i in range(1, 10):
                run_test(str(i))
        case _:
            main()


if __name__ == "__main__":
    main()
