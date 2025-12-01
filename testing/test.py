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


def main():
    os.system("clear")
    print("cmdcreate feature testing v1.0\n\nPick a feature to test:\n")

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
        "10] Importing/exporting command(s)",
    ]

    for line in options:
        print(line)

    run_test(input("\nChoose: "))


def run_test(test):
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
        case _:
            main()


if __name__ == "__main__":
    main()
