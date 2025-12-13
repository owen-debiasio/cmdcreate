#!/usr/bin/env python3

"""
cmdcreate feature testing script

This script provides an interactive CLI interface for testing individual
features of cmdcreate during development. Each feature test is implemented
in its own module inside `scripts/`, and this script simply routes input
to the correct test function.

Features supported:
1. Creating commands
2. Removing commands (normal)
3. Removing commands (forced)
4. Editing commands
5. Listing and managing favorites
6. Searching for commands
7. Displaying command contents
8. Renaming commands
9. Repairing missing installations
10. Importing/exporting command bundles
11. Run every test
"""

from scripts import (
    create,
    remove,
    edit,
    listandfavorite,
    search,
    display,
    rename,
    repair,
    backup,
)
import os

VERSION = "v0.1.1"


def main():
    """
    Main UI entrypoint.

    Clears the screen, prints an option menu, and waits for user input
    to select which cmdcreate feature to test.
    """
    os.system("clear")
    print(f"cmdcreate feature testing {VERSION}\n\nPick a feature to test:\n")

    # Menu choices shown to the user
    options = [
        "1] Creating command",
        "2] Removing command",
        "3] Removing command (forced)",
        "4] Editing command",
        "5] Listing/favoriting command(s)",
        "6] Searching for commands",
        "7] Displaying command contents",
        "8] Rename command",
        "9] Repair command(s)",
        "10] Importing/exporting command(s)\n",
        "11] All tests",
    ]

    for line in options:
        print(line)

    run_test(input("\nChoose: "))


def run_test(test):
    """
    Route the user's numeric selection to the appropriate feature module.

    Each module exposes a `test()` function used specifically for
    development/testing, independent of normal cmdcreate runtime behavior.
    """
    match test:
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
            repair.test()
        case "10":
            backup.test()
        case "11":
            for i in range(1, 10):
                run_test(str(i))
        case _:
            # If invalid input, restart the menu
            main()


if __name__ == "__main__":
    main()
