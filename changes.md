# Cmdcreate Changelog

## (Order: Newest releases - oldest)

### v1.0.6

- Code optimizations
- You can now stack arguments
  - Example: `$ cmdcreate -Vvcr`

### v1.0.5

- Code optimizations
- Testing Python script can now be aborted easily
- Provided extensive documentation
  - Commands & usage
  - Developing cmdcreate
  - Configuring cmdcreate
- Warning when renaming to an existing command
- Error when removing a command from favorites that isn't in there
- Fixed incorrect paths when repairing commands
- Upgrade now asks for confirmation before proceeding

### v1.0.4

- Code optimizations
- Shows latest commit when updating to latest git
- Fixed AUR update issues
- Fixed issues when creating a command

### v1.0.3

- Code optimizations
- Added ability to test using `cargo test`
- Added Visual Studio Code launch options

### v1.0.2

- Fixed Fedora dependency issues
- Code optimizations
- Fixed first-launch issues

### v1.0.1

- Fixed crash when first installed
- Fixed source build issues
- Code optimizations

### v1.0.0 🎉

- Added logging
- Added verbose output
  - Example: `[LOG] <origin>: <details>`
- New debug flag: `-v`, `--verbose`
- Added configurations:
  - `time_format`
    - Default: `%Y-%m-%d %H:%M:%S`
  - `verbose`
    - Default: `false`
- Commands now install to `/usr/local/bin/<command name>`
- Improved upgrade & distro detection
- Unsupported architecture warning can be disabled in config

---

### v0.9.9

- Fixed Debian/Ubuntu dependency issues
- Code optimizations
- Fixed crash when deleting a folder that didn't exist
- Fixed architecture misdetection
- Overhauled upgrade process for reliability

### v0.9.8

- Code optimizations
- Renamed config `fake_arch` → `spoof_arch`
- Added message about disabling unsupported architectures

### v0.9.7

- Code optimizations
- Option to build latest git from AUR
- Downgraded `reqwest` to `0.12` due to AUR build issues

### v0.9.6

- Code optimizations
- Fixed multiple upgrade issues
- Added option to build from latest git
- AUR installs now work on any CPU architecture

### v0.9.5

- Attempt to fix AUR install errors

### v0.9.4

- Updated copyright information
- Code optimizations
- Prevents creating commands with no contents
- Favorite symbol configuration
  - Default: `"★"`

### v0.9.3

- Code optimizations
- CPU architecture detection
- New configuration: `fake_arch`
  - Spoofs architecture support

### v0.9.2

- Updated dependency `reqwest` → `0.13`
- Overhauled shell scripts
- Code optimizations

### v0.9.1

- Code optimizations
- Fixed `-f` / `--force` flags
- Improved `search` output formatting

### v0.9.0

- Added configuration support
- License changed to `GPL-2.0-only`
- Code optimizations

---

### v0.8.9

- Fixed file-not-found bug
- Code optimizations
- Small tweaks

### v0.8.8

- Ctrl-C is no longer disabled
- Removed `bash` dependency
- Code optimizations

### v0.8.7

- Added more metadata to `Cargo.toml`
- Upgrade process improvements
- Major refactor & optimization

### v0.8.6

- Fixed AUR install issues
- Removed `--supported_editors`
- Fixed Ctrl-C crash
- Code optimizations

### v0.8.5

- Upgrade command overhaul
- Improved distro detection
- Ctrl-C disabled (later reverted)

### v0.8.4

- Fixed error when running `cmdcreate edit`
- Added editors: `mousepad`, `zed`, `zed-beta`
- Added license info to version display
- File structure tweaks
- Code optimizations

### v0.8.3

- Major fixes & optimizations
- Python testing fixes for version 3.13.11

### v0.8.2

- Fix attempt (fully resolved in v0.8.3)

### v0.8.1

- File restructure
- Improved test documentation

### v0.8.0

- Skipped v0.7.9 due to major changes
- Added testing library
  - Backup import/export
  - Command creation
  - Editing
  - Listing
  - Favoriting
  - Removing
  - Renaming
  - Repairing
  - Searching

---

### v0.7.8

- Quick fix

### v0.7.7

- Tons of code optimizations
- Tons of bug fixes
- Added `repair` command
  - Fixes missing binaries in `/usr/bin`
  - Usage: `cmdcreate repair`

### v0.7.6

- Fixed listing issues when updating via `.rpm` / `.deb`
- Misc tweaks

### v0.7.5

- Attempted to fix AUR compile error

### v0.7.4-2

- AUR compile error hotfix

### v0.7.4

- Import/export commands fixed
- Code optimizations
- Other tweaks

### v0.7.3

- Updated update information
- Code optimizations

### v0.7.2

- Optimized changelog
- Added `favorite` command
  - `add` / `remove`
  - Favorites show `★`

### v0.7.1

- Fixed command deletion bugs
- Added documentation & comments
- Code optimizations

### v0.7.0

- Fixed slow startup
- Fixed command deletion bugs
- Added `import` / `export` commands
  - `import <file input>`
  - `export <output directory>`

---

### v0.6.9

- Fixed broken update checking
- Fedora system support
- Added `.rpm` install/update support
- Removed `reset` command
- Removed `--arg_count` and `--offline` flags
- Bug fixes
- Upgrade improvements

### v0.6.8

- Code optimizations
- File structure overhaul

### v0.6.7

- Heavy code optimizations

### v0.6.6

- Bug fixes
- Added AUR update options
  - Use AUR manager (Yay/Paru)
  - Clone repo manually

### v0.6.5

- Code optimizations
- Added `--force` / `-f` flag
- Improved debugging menu
- Bug fixes

### v0.6.4

- Bug fix
  - Fixed wrong filename retrieval in upgrader
- Code optimizations

### v0.6.3

- Output fixes
- Organized file structure
- Colored output
- Help output overhaul

### v0.6.2

- Code optimizations

### v0.6.1

- Code optimizations
- Tweaked `rename` output

### v0.6.0

- Improved error handling
- Heavy optimizations
- Fixed `%` output bug in `display`
- Added debug flag `--offline`

---

### v0.5.9

- Added `rename` command
  - Usage: `cmdcreate display <command> <new name>`
- Code optimizations

### v0.5.8

- Added `display` command
- Added `kwrite` as supported editor
- Intro overhaul
- Code optimizations

### v0.5.7

- Added double-confirm reset
- Removed update checks when running with no args

### v0.5.6

- Disabled update command if already on latest version
- Added `vscodium-insiders`
- Removed `credits` flag

### v0.5.5

- Updated AUR update method
- Code cleanup & optimizations

### v0.5.4

- Added `check` command
- Added `update` command

### v0.5.3

- Fixed command deletion bug
- Fixed insufficient write permissions

### v0.5.2

- Fixed deleting non-existent commands bug
- Code cleanup

### v0.5.1

- Added `--credits` (later removed)
- Added debugging flags
  - `--arg_count`
  - `--force_system_shell`
- Code cleanup

### v0.5.0

- Fixed incorrect version reporting
- Improved `list` command
  - Shows number of installed commands
- Improved `search` command
  - Shows result count
- Better usage error messages

---

### v0.4.9

- Added `.deb` install support
- Added dependencies: `bash`, `curl`, `nano`

### v0.4.8

- Fixed offline files directory bug
- Added `--remove_offline_files`

### v0.4.7

- Bug fixes
- Improved error handling
- Added `--get_offline_files`
- Added `vscodium` editor

### v0.4.6

- Added `reset` command
- Code cleanup

### v0.4.5

- Added `--license`
- Tweaked intro
- Code cleanup

### v0.4.4

- Added `search` command

### v0.4.3

- Listing commands now shows only names
- Added `--changelog` flag
- Code cleanup

### v0.4.2

- Added `vi` and `emacs`
- Added update explanation

### v0.4.1

- Added `code-insiders`
- Overhauled intro

### v0.4.0

- Added `edit` argument
- Added `--version`
- Added `--supported_editors`
- Added flag descriptions
- Bug fixes

### v0.3.0

- Changed command usage
- Added delete confirmation
- Other overhauls

### v0.2.0

- `create` now shows action info
- Code cleanup
