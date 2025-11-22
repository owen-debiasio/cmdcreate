# Cmdcreate changelog

## v0.2.0

- When running the `create` tool, it will now show some info of what it did
- Code cleanup

## v0.3.0

- Command usage has changed
- When removing a command, it now asks for confirmation
- Other overhauls

## v0.4.0

- `edit` arg: Allows you to edit your commands
- `--version` arg: Displays the version of cmdcreate
- `--supported_editors` arg: Displays current text editors
- Flag and arg descriptions
- Bug fixes
- Other misc changes

## v0.4.1

- Added `code-insiders` as a supported editor
- Overhauled the intro

## v0.4.2

- Added `vi` and `emacs` as supported text editors
- Added information about how updates work.

## v0.4.3

- Listing commands now only displays the command names
- Code has been cleaned up (at least a little bit)
- `--changelog` flag: Displays cmdcreate's changelog

## v0.4.4

- `search` command: Searches for matched command

## v0.4.5

- Cleaned up code
- `--license` arg: Displays license
- Tweaked the intro

## v0.4.6

- Code cleanup
- `reset` command: Removes ALL installed commands

## v0.4.7

- Bug fix
- Partially enhanced error handling
- `--get_offline_files` argument: Downloads files needed to use cmdcreate offline.
- Added `vscodium` as a supported text editor

## v0.4.8

- Bug fix: Retrieving offline files always fails due to missing directory
- `--remove_offline_files` argument: Removes offline files

## v0.4.9

- You can now install cmdcreate with a `.deb` file (Found in the `releases` tab)
- Added `bash`, `curl`, and `nano` as dependencies

## v0.5.0

- Bug fixes:
  - Fixed reporting of incorrect version
- Enhancements/Fixes for command `list`:
  - Now shows the number of installed commands
    - Fixed incorrect reporting of if commands are installed
- Enhancements/Fixes for command `search`:
  - Now shows the number of commands that contain your input
  - Cleaned up code
- Misc enhancements:
  - When you don't enter enough arguments for a command, it will display its usage.

## v0.5.1

- New arguments:
  - `--credits`: Displays credits for cmdcreate
  - `--debugging`: Displays flags used for debugging
    - Flags for debugging:
          - `--arg_count`: Displays number of arguments supplied
          - `--force_system_shell`: Forces system shell to be used when running commands
- Cleaned up code a bit

## v0.5.2

- Fixed bug where cmdcreate would allow you to delete a command that doesn't exist
- Cleaned up code a bit

## v0.5.3

- Fixed bug where cmdcreate would allow you to delete a command that didn't exist
- Fixed bug where you would have insufficient write access to commands.

## v0.5.4

- New commands:
  - `check`: Allows you to check for cmdcreate updates.
  - `update`: Allows you to update cmdcreate easily.

## v0.5.5

- Updated AUR update method by cloning the `cmdcreate` branch from the AUR directly.
- Code optimizations and cleanup
- Other small changes

## v0.5.6

- `update` command is now disabled if you run the latest version.
- Added `vscodium-insiders` as a supported editor
- Removed `credits` flag, no purpose.

## v0.5.7

- When resetting cmdcreate, it will now have a double check just in case
- cmdcreate no longer checks for updates when running `cmdcreate` with no arguments/flags

## v0.5.8

- Code optimizations
- Added `kwrite` as a supported editor
- `display` command: Displays the contents of a command
- Overhaul to the intro

## v0.5.9

- `rename` command: Allows you to rename commands
  - Usage: `cmdcreate display <command> <new name>`
- Code optimizations

## v0.6.0

- Improved error handling
- Heavy code optimizations
- Fixed bug in command `display`
  - Fixed the output of `%` at the end of the output
- Debugging flag: `--offline`: Allows you to run commands without an internet connection

## v0.6.1

- Code optimizations
- Tweaks to command `rename`
  - Removed the line showing how many results there are

## v0.6.2

- Code optimizations

## v0.6.3

- Output fixes
- Organized file structure
- Colored text output
- Help output overhaul

## v0.6.4

- Bug fix
  - Fixed when upgrader retrieves wrong file name string.
- Code optimizations

## v0.6.5

- Code optimizations
- `--force`/`-f` flag: Skips confirmation for an action
- Typos and bug fixes
- Improved debugging menu

## v0.6.6

- Typo fix
- Bug fixes
- When upgrading through the AUR, you now have to options:
  - Use AUR manager (Yay and Paru only)
  - Clone repository

## v0.6.7

- Heavy code optimizations

## V0.6.8

- Code optimizations
- File structure overhaul

## v0.6.9

- Fixed the broken update checking
- Code optimizations
- You can now install/update cmdcreate through a `.rpm` file
- Fedora system support
- Removed `reset` command
- Removed `--arg_count`/`a` and `--offline`/`-o` flags
- Bug fixes

## v0.7.0

- Fixed bugs
  - Slow startup
  - Fixed commands not deleting
- Code optimizations
- You can now `import`/`export` commands
  - Added commands
    - `import <file input>`
    - `export <output directory>`

## v0.7.1

- Fixed commands not deleting again
- Code optimizations
- Fixed typos
- Added proper documentation/code comments to files

## v0.7.2

- Optimized changelog
- Code optimizations
- Added `favorite` command
  - Usage
    - `add` Adds a command to favorites
    - `remove` Removes a command from favorites
  - Displays listed commands with an additional `★`

## v0.7.3

- Updated information about updates
- Code optimizations

## v0.7.4

- Exporting/importing commands should now work properly
- Code optimizations
- Other tweaks

## v0.7.4-2

- AUR users: compile error

## v0.7.5

- Code optimizations
- Attempt to fix the compile error for AUR users again

## v0.7.6

- Fixed odd listing when updating with a `.rpm` or `.deb` file
- Misc tweaks
