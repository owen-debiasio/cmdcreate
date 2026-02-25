<!--
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# The File Structure of cmdcreate

Here I provide the organization of the cmdcreate GitHub repository.

## GitHub Configuration & Workflows

- 📁 [.github](https://github.com/owen-debiasio/cmdcreate/tree/main/.github)
  - 📁
    [ISSUE_TEMPLATE](https://github.com/owen-debiasio/cmdcreate/tree/main/.github/ISSUE_TEMPLATE):
    Templates for bugs and features.
  - 📁
    [workflows](https://github.com/owen-debiasio/cmdcreate/tree/main/.github/workflows):
    CI/CD for Bash, Markdown, Python, and Rust.

## Developer & Environment Tools

- 📁 [dev](https://github.com/owen-debiasio/cmdcreate/tree/main/dev): Scripts
  for formatting, environment setup, and cleaning.
- 📁 [.idea](https://github.com/owen-debiasio/cmdcreate/tree/main/.idea):
  JetBrains/IntelliJ project configuration.
- 📁 [.vscode](https://github.com/owen-debiasio/cmdcreate/tree/main/.vscode): VS
  Code workspace and launch settings.

## Documentation

- 📁 [docs](https://github.com/owen-debiasio/cmdcreate/tree/main/docs)
  - 📁
    [resources](https://github.com/owen-debiasio/cmdcreate/tree/main/docs/resources):
    Supplemental files like `config_example.toml`.
  - [Developing.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/developing.md),
    [Commands.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/commands.md),
    etc.

## Main Source Code (Rust)

- 📁 [src](https://github.com/owen-debiasio/cmdcreate/tree/main/src)
  - 📁
    [commands](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands):
    Implementation of all CLI subcommands (create, edit, search, etc.).
  - 📁 [utils](https://github.com/owen-debiasio/cmdcreate/tree/main/src/utils):
    Shared logic for networking, filesystem, and system I/O.
  - `main.rs`, `init.rs`, `logger.rs`, `parse.rs`: Core application logic.

## Testing Suite (Python & Scripts)

- 📁 [testing](https://github.com/owen-debiasio/cmdcreate/tree/main/testing)
  - 📁
    [scripts](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/scripts):
    Python-based functional tests for individual features.
  - `main.py`, `shared.py`: Test runner and shared utilities.

## Packaging

- 📁 [package](https://github.com/owen-debiasio/cmdcreate/tree/main/package):
  Scripts for generating `.deb`, `.rpm`, and standalone binaries.

## Root Configuration Files

- [Cargo.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/Cargo.toml):
  Rust project metadata and dependencies.
- [COPYING](https://github.com/owen-debiasio/cmdcreate/blob/main/COPYING): Full
  GPLv3 License text.
- [LICENSE](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE):
  License metadata.
- [.pylintrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.pylintrc),
  [.prettierrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.prettierrc):
  Linting and formatting rules.
