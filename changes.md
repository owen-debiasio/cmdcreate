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

## v0.7.7

- Tons of code optimizations
- Tons of bug fixes
- Added command: `repair`
  - Fixes commands by recreating missing binaries in `/usr/bin`
  - Usage: `cmdcreate repair`

## v0.7.8

- Quick fix

## v0.8.0

### Skipped v0.7.9 because of major changes

- Typo fix
- Code optimizations
- Added testing library
  - Run tests:
    - Backup (importing/exporting)
    - Command creation
    - Displaying of command contents
    - Editing of command contents
    - Listing commands
    - Favoriting commands
    - Removing commands
    - Renaming commands
    - repairing commands
    - Searching for commands

## v0.8.1

- Code optimizations
- File restructure
- Added better comments/documentation to feature testing scripts

## v0.8.2

- Fix (Update: Did not work, was fixed in v0.8.3)

## v0.8.3

- Major code optimizations and fixes
- Updated/fixed Python testing scripts for version 3.13.11
- Other random fixes

## v0.8.4

- Fixed command not installed error when running `cmdcreate edit`
- Reorganized the file structure a bit
- Added `mousepad`, `zed`, and `zed-beta` as supported editors
- Changed some statics to consts for consistency
- Added copyright and license information to version display
- Code optimizations
- You can now run all tests by picking option 11 in the Python script
- Other random fixes

## v0.8.5

- Code optimizations
- Formatted shell files to make them look better
- Overhauled upgrading command
  - It can now detect your distro (hopefully)
    - PLEASE and I mean **PLEASE** report issues
- You can no longer use the CTRL-C shortcut to exit
  - Meant to prevent accidental exits and break things

## v0.8.6

- Fixed AUR packages not installing (my mistake)
- Removed `--supported_editors`/`-s` flag: Why would you need this?
- Code optimizations
- Fixed bug where the CTRL-C disabled caused crashes

## v0.8.7

- Added more information in `Cargo.toml`
- Code optimizations
- The upgrading process should work a bit better now
- Optimized the sh!t out of the code, reorganized tons of stuff

## v0.8.8

- Ctrl-C is no longer disabled
- `bash` is no longer a dependency
- Code optimizations

## 0.8.9

- Fixed bug: (Thank you `Dominiquini` for bringing this to my attention!)
  - File not found error for creating, removing, listing commands
- Code optimizations
- Other small tweaks

## v0.9.0

- Code optimizations
- Added support for configurations
  - For all available configurations, visit:
    - <https://github.com/owen-debiasio/cmdcreate/blob/main/config_example.toml>
- Changed license to `GPL-2.0-only`

## v0.9.1

- Code optimizations
- Fixed `-f` and `--force` flags not working
- Made `search` command output look nicer

## v0.9.2

- Updated dependency `reqwest` to version `0.13`
- Overhauled a couple of the shell scripts
- Code optimizations

## v0.9.3

- Code optimizations
- cmdcreate will now detect your CPU and will let you know if it is supported
- New configuration: `fake_arch`
  - Spoofs your architecture to make cmdcreate think it is supported
  - boolean
    - `false` by default

## v0.9.4

- Updated copyright information
- Code optimizations
- cmdcreate will now exit if you create a command with no contents
- New configuration: `fake_arch`
  - A symbol or string of text to indicate a favorite command
  - String
    - `"★"` by default

## v0.9.5

- Attempt to fix AUR install errors

## v0.9.6

- Code optimizations
- Fixed multiple issues with the upgrade process
- You now have the option to build from source from the latest git in the upgrading process.
- Installing via `AUR` should now work on any cpu architecture

## v0.9.7

- Code optimizations
- You now have the option to build from source from the latest git from the AUR in the upgrading process
- Downgraded dependency `reqwest` to version `0.12` due to build issues AUR installation

## v0.9.8

- Code optimizations
- Random tweaks
- Config `fake_arch` has been renamed to `spoof_arch`
- Added additional message about disabling unsupported architectures in the configuration file

## v0.9.9

- Fixed dependency issues when installing on `Debian`/`Ubuntu` systems
- Code optimizations
- Fixed bug where that cmdcreate would crash if it tried to delete a folder that didn't exist
- Fixed misdetection of architecture
- Overhauled the upgrade process to be more reliable

---

## v1.0.0 🎉

- Added logging
- Added verbose output
  - Example: `[LOG] <file::function() of origin>: <details>`
- New debug flag: `-v`, `--verbose`
  - Used for verbose output of logs
- Added configurations:
  - `time_format`
    - Time format used for logs
    - String
    - Default: `%Y-%m-%d %H:%M:%S`
  - `verbose`
    - Enabled verbose logging by default
    - Boolean
    - Default: `false`
- Code optimizations
- File structure changes and organizations
- Installation of commands have now moved to `/usr/local/bin/<command name>`
- Fixed (yet again) issues with the distro detection and upgrade process
- The unsupported architecture message now gives you an option to disable it in the configuration file

## v1.0.1

- Fixed crash when first installed
- Fixed issues regarding building from source
- Code optimizations

## v1.0.2

- Fixed dependency issues when installing on `Fedora` systems
- Code optimizations
- Fixed issue when launching for the first time

## v1.0.3

- Code optimizations
- Added ability for testing using `cargo test`
- Added launch options for `Visual Studio Code`

## v1.0.4

- Code Optimizations
- When given the option to update to the latest git, it will now show you the latest commit.
- Fixed issues when updating through the AUR
- Fixed issues when creating a command

## v1.0.5

- Code optimizations
- When running the testing Python script, you now have the ability to easily abort.
- Provided tons of documentation
  - Commands and how to use them
  - Developing cmdcreate
  - Configuring cmdcreate
- You will now be warned if a command that you want to rename another command to with the same name
- You will now be thrown an error for trying to remove a command from favorites that isn't in there
- Fixed incorrect paths when repairing commands
- When upgrading, you are now asked if you want to upgrade before anything else.
