<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

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
- `--get_offline_files` argument: Downloads files needed to use cmdcreate
  offline.
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
  - When you don't enter enough arguments for a command, it will display its
    usage.

## v0.5.1

- New arguments:
  - `--credits`: Displays credits for cmdcreate
  - `--debugging`: Displays flags used for debugging
    - Flags for debugging: - `--arg_count`: Displays number of arguments
      supplied - `--force_system_shell`: Forces system shell to be used when
      running commands
- Cleaned up code a bit

## v0.5.2

- Fixed bug where cmdcreate would allow you to delete a command that doesn't
  exist
- Cleaned up code a bit

## v0.5.3

- Fixed bug where cmdcreate would allow you to delete a command that didn't
  exist
- Fixed bug where you would have insufficient write access to commands.

## v0.5.4

- New commands:
  - `check`: Allows you to check for cmdcreate updates.
  - `update`: Allows you to update cmdcreate easily.

## v0.5.5

- Updated AUR update method by cloning the `cmdcreate` branch from the AUR
  directly.
- Code optimizations and cleanup
- Other small changes

## v0.5.6

- `update` command is now disabled if you run the latest version.
- Added `vscodium-insiders` as a supported editor
- Removed `credits` flag, no purpose.

## v0.5.7

- When resetting cmdcreate, it will now have a double check just in case
- cmdcreate no longer checks for updates when running `cmdcreate` with no
  arguments/flags

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
- Debugging flag: `--offline`: Allows you to run commands without an internet
  connection

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
- You now have the option to build from source from the latest git in the
  upgrading process.
- Installing via `AUR` should now work on any cpu architecture

## v0.9.7

- Code optimizations
- You now have the option to build from source from the latest git from the AUR
  in the upgrading process
- Downgraded dependency `reqwest` to version `0.12` due to build issues AUR
  installation

## v0.9.8

- Code optimizations
- Random tweaks
- Config `fake_arch` has been renamed to `spoof_arch`
- Added additional message about disabling unsupported architectures in the
  configuration file

## v0.9.9

- Fixed dependency issues when installing on `Debian`/`Ubuntu` systems
- Code optimizations
- Fixed bug where that cmdcreate would crash if it tried to delete a folder that
  didn't exist
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
- The unsupported architecture message now gives you an option to disable it in
  the configuration file

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
- When given the option to update to the latest git, it will now show you the
  latest commit.
- Fixed issues when updating through the AUR
- Fixed issues when creating a command

## v1.0.5

- Code optimizations
- When running the testing Python script, you now have the ability to easily
  abort.
- Provided tons of documentation
  - Commands and how to use them
  - Developing cmdcreate
  - Configuring cmdcreate
- You will now be warned if a command that you want to rename another command to
  with the same name
- You will now be thrown an error for trying to remove a command from favorites
  that isn't in there
- Fixed incorrect paths when repairing commands
- When upgrading, you are now asked if you want to upgrade before anything else.

## v1.0.6

- Code optimizations
- You can now stack arguments
  - Example: `$ cmdcreate -Vvcr`
- Various documentation improvements and additions
  - Added information about resources
  - Added information about updates
  - Added information about the file structure
  - Various quality of life changes
- I've hopefully made cmdcreate more developer-friendly
- Fixed ~/rpmbuild from not being deleted

## v1.0.7

- Fixed license file from not being downloaded
- Code optimizations
- Created splash screen for verbose output
- Using `format.sh` now uses markdownlint
- Added documentation info about cmdcreate
- Improved documentation
- You can now use the `EDITOR` environment variable when using `cmdcreate edit`

## v1.0.8

- Removed the CPU Arch. warning from the init process
- Code optimizations
- Removed `-F`/`--force_system_shell` flags
- Improved the developer environment setup script.
- Checking for updates can now detect if you are running a development build

## v1.0.9

- Code optimizations
- Cleaned up code
- Reorganized file structure
- Added restrictions for updating on unsupported architectures
- The verbose intro now shows whether the installation is a stable or
  development build

## v1.1.0

- You will now be kicked out from checking for updates or updating cmdcreate if
  you have no internet
- Optimized internal updating process
- Added command: `clean`
  - Can clean up old and unused files
- New developer utility: `dev/clean.sh`
  - Cleans up the development environment
- Reorganized file structure
  - Only applies to development tools
- Made shell files look nicer

## v1.1.1

- You now need to run cmdcreate as root (like using sudo)
- Installation of commands has moved to /usr/local/bin/(command)
- Removed command `repair`: No longer needed
- Update methods are only listed if they apply to your system
- Removed `repair` test
- Code optimizations
- Added flag: `-o`, `--offline`
  - Used when you want to use cmdcreate offline
- Removed home directory information from verbose init
- Added internet connection status to verbose init
- Certain things are now restricted if you don't have an internet connection

## v1.1.2

- Fixed offline mode being detected incorrectly
- Added a hidden offline flag for dev/install.sh
- Code optimizations
- Fixed random crash when args are being parsed
- Fixed error when updating via binary

## v1.1.3

- Many code optimizations
- Building from source when updating should now build properly
- Building from source now dynamically shows the warning for manual intervention

## v1.1.4

- Change license to GPLv3
- Moved configurations from .config/cmdcreate/config.toml to /etc/cmdcreate.toml
- Fixed configs not being read
- Code optimizations
- `-m` and `--monochrome` flags: Disables colored output
- New configuration
  - `disable_color`
    - Located in `appearance` category
    - Enables or disables color
    - Boolean
    - Default: `false`
- When installing through the AUR, .deb package, or .rpm package, they now
  include the License file in their respective directories.

## v1.1.5

- Fixed license not being read on Debian systems
- Fixed incorrect response parsing during the update process
- Fixed other things along the update process
- Code optimizations
- Overhauled the debug verbose logging intro

## v1.1.6

- Removed the following flags/args (No need for them anymore):
  - `-g`/`--get_offline_files`, `-r`/`--remove_offline_files`
    - No need for offline files anymore
  - `-d`/`--debug`
    - No need for this menu
- Removed debug flag menu, moved `-V`/`verbose` and `-f`/`--force` to normal
  usage screen
- Slight tweaks to the usage dialog
- Code logic fixes
- Code optimizations

## v1.1.7

- Many code optimizations
- Generational amounts of refactoring
- Commands created by cmdcreate are now identified by a header inserted into the
  file
- Small quality of life changes
- cmdcreate now auto-downloads the license if not installed (`curl` is required)

## v1.1.8

- Added flags `-b`/`--bypass-root`
  - Allows you to bypass root requirements
  - **ONLY USE IF YOU KNOW WHAT YOU ARE DOING**
- Code optimizations
  - Optimized how parsing works
  - Optimized parts of the updating process

## v1.1.9

- Optimized how internal project metadata is handled
- Optimized how logging is categorized
- Other code optimizations
- Fixed typo
- Removed command "clean": Fundamentally useless

## v1.2.0

- Code optimizations
  - Optimized command and argument parsing
- New flags for command `create`:
  - `--in_editor`/`-i`
    - Allow to write the contents of the new command from a text editor
- You can now delete multiple commands at once
  - Example:
    - `cmdcreate remove command1 command2`
- Added success checks for the following commands:
  - `create`
  - `remove`
  - `rename`
  - `favorite`
- Command `rename` now handles favorite command status properly
- Fixed issues with internet detecting
- Other small tweaks

## v1.2.0-2 (HOTFIX)

- ACTUALLY fix issues with internet detecting

## v1.2.1

- Code optimizations
- Fixed incorrect error message when renaming command
- Fixed autogenerated message from not being written for configs

## v1.2.2

- Added better dynamic info for the verbose logging intro
- Code optimizations
  - Optimized how commands are initially retrieved
  - Heavily organized file structure among system functions
- Made verbose output nicer in the development environment setup script
  (dev/setup_env.sh)
- Success of installing license is now determined

## v1.2.3

- You can now create a command while not entering the command contents when
  running with `-i`/`--in-editor`:
  - `cmdcreate create command-name -i`
- Enhanced the updating experience
  - Made it less loud, only necessary output
- Code optimizations
  - Updated how messages are printing and better string formatting for functions
    such as:
    - `run_shell_command!()`
    - `output!()`
      - Replacing the use of `println!()`
    - `input!()`
- Overhauled output messages and text to look nicer
  - As of this update, there are some inconsistencies, which will be fixed in
    the future.

## 1.2.4

- Continued migration to macro my macro `output!()` located in src/utils/io.rs
- Updated dependencies
  - reqwest
    - `0.12.28` -> `0.13.2`
  - toml
    - `1.0.6` -> `1.1.2`
- Updated both `cmdcreate` and `cmdcreate-git` PKGBUILD scripts
  - They were messy
  - Added fixes needed for updated dependencies
- Created script to more easily install cmdcreate on non-Arch based distros
- Script `cmdcreate/dev/uninstall.sh` now handles the license when uninstalling

## 1.2.5

- Development utility bash scripts now have color to make them look cleaner
- Favorites data and logs are now stored in `/usr/share/cmdcreate/`
- Added more dependencies to development environment setup script
- `src/dev/format.sh` now formats/fixes Markdown (`.md`) and YAML (`.yml`) files
- Workflow `markdown` now lints Markdown (`.md`) and YAML (`.yml`) files
- Added new command: `config`
  - Usage:
    - `cmdcreate config <add/remove> <category> <value>`

## 1.2.6

- Brought back shell configuration after I accidentally removed it
- Code optimizations
- Running `cmdcreate -v` now shows the build status
- Added `xed` as a supported text editor
- Added new dependency:
  - `less`
- Added new command (`doc`):
  - Usage:
    - `sudo cmdcreate doc <information>`
  - View the following documentation references:
    - `main` (The main README file)
    - `changelog` (The complete version history of cmdcreate)
    - `license` (The license for cmdcreate)
    - `security` (Security information)
    - `contributing` (Information about contributing)
    - `intro` (Intro to the documentation)
    - `about` (About cmdcreate and its purpose)
    - `commands` (About the current commands in cmdcreate)
    - `configurations` (Information of configuring cmdcreate)
    - `developing` (Information on developing cmdcreate)
    - `structure` (The file structure of cmdcreate)
    - `updates` (Information on updates)
    - `testing` (Information about testing the features of cmdcreate)
    - `packaging` (Information about packaging cmdcreate)
- Retired the following flags for replacement with `cmdcreate doc`
  - `-c`/`--changelog`
    - New usage:
      - `cmdcreate doc changelog`
  - `-l`/`--license`
    - New usage:
      - `cmdcreate doc license`
- Viewing the changelog and license (along with new the documentation viewer)
  now uses `less`

## v1.2.7

- Fixed `cmdcreate doc changelog`, I got the code logic wrong
- Added missing `code_of_conduct` option for `cmdcreate doc`
- In addition to `cmdcreate doc list`, you can now just run `cmdcreate doc` to
  get the list of available documentation.

## v1.2.8

- Improved the visual output when building from source
- Updated dependency 'reqwest' to v0.13.3
- New flag: `-s`/`--silent`
  - Suppresses any output that's not an error
- Began work on new testing suite
- Code optimizations
- Updated documentation for cmdcreate commands
- `dev/format.sh` now runs clippy
- Overhauled the messages for getting usage for cmdcreate
- Changes for command `config`:
  - New subcommands:
    - help
      - Get documentation for configuring cmdcreate
    - example
      - View an example config file
    - edit
      - Edit the configurations through a text editor
    - display
      - Display the configurations to output
  - Overhauled to be more user-friendly
  - Fixed weird crashes

## v1.2.9

- Code optimizations
  - Reorganized filesystem related functions
    - Now located in `src/utils/fs/`
- Changes in usage for running as root
  - You now only need to run cmdcreate for some commands
    - You will need root for commands like:
      - `create`
      - `remove`
      - `edit`
    - You will not need root for commands like:
      - `list`
      - `search`
      - `export`
    - `cmdcreate update` will require root depending on how you update
    - Running flags no longer require root by themselves
- Slightly improved the updating experience
- Root status is now included in the verbose debug intro
- Logs are now stored in `/tmp/`. They are deleted on reboot/shutdown.
- Reintroduced the ability to update via the AUR when using `cmdcreate update`
- Overhauled filesystem initialization
- Fixed documentation not being deleted after being retrieved

## v1.3.0

- Fixed passed args not being collected/read properly
- Code optimizations
- Fixed permission denied error for when you download license not as root
- Y/N question prompts can no longer be silent, even when passing
  `-s`/`--silent`. This is done to prevent cmdcreate from doing things without
  your consent.
- Shell commands are now made silent through the `-s`/`--silent` flags rather
  than manually
- `dev/clean.sh` now formats code before cleaning up cargo
  - Before, it would clean up the Rust environment first, then run `clippy`
    which created the directory `target` again.
- Slightly modified the output for the usage of cmdcreate to be better at color
  coding
- Fix crash when user tries to build from source without root
- Changed log directory to `/tmp/cmdcreate-logs/`

## v1.3.1

- Enhanced UX
- Code optimizations
- Fixed hang on startup when running for the first time
  - I have no idea if this problem was exclusive to virtual machines, but that's
    how I discovered the issue.
- You can now use the following subcommands of command `config` without root:
  - `help`
  - `example`
  - `display`
- New flag for documentation file `changelog` in command
  `cmdcreate doc changelog`
  - `--latest`/`-l`
    - Get the latest entry in the changelog

## v1.3.2

- Fixed flag `-l` not being detected when running `cmdcreate doc changelog -l`
- cmdcreate is now statically linked, meaning that dependencies such as
  `openssl` and `cmake` are no longer needed
- Added support for 32 bit systems, and options for 32 bit binaries amd packages

## v1.3.3

- Replaced dependency `reqwest` with `ureq`
  - Less/no build dependencies
  - Shorter compile time
- Code optimizations
- `dev/install.sh` now installs a statically linked file

## v1.3.4

- Organized file structure for command source files (`src/commands/*`)
- Fixed parsing errors when adding a new config
- Code optimizations
- Added support for 32 and 64-bit ARM CPUs
  - `aarch64`
  - `armv7`

## v1.3.5

- Absolutely overhaul file structure
- Code optimizations
- Overhauled how usage of commands is displayed
- Fixed confusing parsing for command `export`
- Change description

## v1.3.6

- Fix usage displayed for commands `search` and `favorite`
- Code optimizations
  - Especially involving the files for building from source
- Update file structure of building from source scripts

## v1.3.7

- New releases of cmdcreate are how handled by an automatic CI
- Code optimizations
- `./aur_update.sh` is removed, now no longer needed.
- New development script: `./update_tag`.
  - Pushes new tag version to trigger update CI

## v1.3.8

- Fix building from source issues
- Fix permission issues regarding logging
- Code optimizations
- The cmdcreate install script (`install.sh`) now handles ARM 64 bit (`aarch64`)
  ARM 32 bit (`armv7`).
- Enhanced distro detection
- Enhanced license path detection

## v1.3.9

- "Fix" the permission issues when running not as root on Debian/Ubuntu systems
- cmdcreate can now be used not as root. This helps support on immutable
  distros.
  - These are the paths used when not as root:
    - Command installation directory: `~/.local/bin/cmdcreate/`
    - Favorite commands: `~/.local/share/cmdcreate/favorites`
    - Configuration file: `~/.config/cmdcreate/cmdcreate.toml`
    - License: `~/.local/share/cmdcreate/`
- Code optimizations
- New configurations
  - New category: `internet`
    - `force_disable`: If "true" internet will be force disabled by default.
      (Default: "false")
    - `sample_dns`: The default is set to Cloudflare. Used to check internet
      connectivity. (Default: "1.1.1.1:53")

## v1.4.0

This is a big one! Although this update is mostly focused on developing, there
are also plenty of new features, changes and fixes.

### Added

- Subcommand `list` for command `config`. Lists the available categories and
  keys/values.
- New configurations
  - Category: `self`
    - `disable_root_usage`
      - If `true`, cmdcreate is prevented from running as root.
      - `false` (disabled) by default
      - Can only be enabled when running cmdcreate as root, then can only be
        disabled by modifying `/etc/cmdcreate.toml`.
      - This setting is useful for immutable/atomic distros.
      - This also disables the ability to update via most methods.
  - Category: `update`
    - `zig_version`
      - The version that is downloaded when building cmdcreate from source when
        using `sudo cmdcreate update`
      - `0.16.0` by default **(as of June 27, 2026)**

### Changes/Fixes

- Various code optimizations
- Fixed building from source when running `sudo cmdcreate update`
- `zig` and `rustup` are no longer handled via the distros package manager
- Fixed download links for `zig`
- If you are using an immutable distro, the following restrictions are applied:
  - No updating
  - No running cmdcreate as root
- (Almost) all text output is now logged in the log file
- Overhauled how verifying settings works when running `cmdcreate config add` or
  `cmdcreate config remove`
- The default shell is now the shell defined through the `$SHELL` environment
  variable

### Developers

#### Added

- You can now test features using `cargo test`
  - Replaces the Python testing suite
- New developer utility: `./dev/test.sh`
  - Tests cmdcreate using `cargo test`

#### Removals/Deprecations

- Remove `./dev/clean.sh` as a dev utility. There is virtually nothing to clean,
  just run `cargo clean` instead.
- Deprecation of the `install.sh` install script. It was unmaintained and I'll
  be honest and say I vibecoded it. It's lowkey for the best
- I am now no longer providing the Python testing utilities. They will be
  replaced by tests you can run using `cargo test`.
- Removed Python linting and code formatters from dev dependencies such as
  `black` and `pylint`.

#### Changes/Fixes

- Linting tools are now optional when using `./dev/format.sh`
- Overhauls to `./dev/install.sh` and `./dev/uninstall.sh`
  - You must now specify if you want to install via these methods:
    - User
      - Pass `-u`/`--user`
      - Only option for immutable distros, but usable everywhere.
    - System
      - Pass `-s`/`--system`
      - Can use on any mutable distro
  - Only one installation of cmdcreate is allowed.
- Move `./update_tag.sh` to `./dev/update_tag.sh`

## v1.4.1

### Changed

- The main usage paragraph is no longer logged (the text you see without running
  commands or passing flags)
- Change non-root license path to `~/.local/share/doc/cmdcreate/LICENSE`
- Overhauled distro detection, made more efficient and less confusing

### Removed

- Remove flags `-b`/`--bypass-root`: No longer needed due to the ability to use
  cmdcreate unelevated
- Information for installation method in verbose intro
- Build warning when building from source on Debian systems using
  `cmdcreate update`
- Claim for universal compatibility when using `cmdcreate update`

### Fixed

- cmdcreate now launches faster due to logic enhances in the detection of the
  build status.
- Flags `-s`/`--silent` didn't do anything due to intentional differences in
  macros I forgot about

### Developers

#### Removed

- Remove test `distro_detection_returns_known_or_unknown`: No longer applicable

## v1.4.2

### Changed

- When cleaning up building from source when using `cmdcreate update`, directory
  `/tmp/cmdcreate` is also a directory to be cleaned up.

### Fixed

- Incorrect message when cleaning up building from source
- `cmdcreate export` creates an empty file when no commands are installed

### Developers

#### Added

- Add Zed config to make Rust Analyzer use `clippy` over `cargo check` in
  `.zed/settings.json`

## v1.4.3

### Changed

- Update cmdcreate for rust `1.97.1`

### Fixed

- Fix variable referencing issues in
  `cmdcreate::utils::fs::paths::PATHS.favorites`

### Developers

#### Added

- `marksman` as a linter. Installed through `brew`.

#### Removed

- The following dependencies, they are no longer needed:
  - `npm`
  - `nodejs`

#### Changed

- `brew` is now used to install the following on `fedora`, `debian`, and
  `ubuntu` systems:
  - `markdownlint-cli2`
  - `prettier`
  - `zig` **(`Debian`/`Ubuntu` only)**
  - `brew`

## v1.4.4

### Changes/Fixes

- Code optimizations
  - Removed unused function
    `cmdcreate::utils::fs::paths::get_program_binary_path()`

### Developers

#### Fixed

- Incorrect email in `./package/create_deb.sh`
