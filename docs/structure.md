<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# The File Structure of cmdcreate

Here I provide the organization of the cmdcreate GitHub repository. Subject to
change.

> [!NOTE] File structure may be inaccurate, please let me know or update it
> yourself somehow.

## GitHub templates, actions, workflows

- 📁 [.github](https://github.com/owen-debiasio/cmdcreate/tree/main/.github)
  - 📁
    [ISSUE_TEMPLATE](https://github.com/owen-debiasio/cmdcreate/tree/main/.github/ISSUE_TEMPLATE)
    - [bug_report.md](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/ISSUE_TEMPLATE/bug_report.md)
    - [feature_request.md](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/ISSUE_TEMPLATE/feature_request.md)
  - 📁
    [workflows](https://github.com/owen-debiasio/cmdcreate/tree/main/.github/workflows)
    - [bash.yml](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/workflows/bash.yml)
    - [markdown.yml](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/workflows/markdown.yml)
    - [python.yml](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/workflows/python.yml)
    - [rust.yml](https://github.com/owen-debiasio/cmdcreate/blob/main/.github/workflows/rust.yml)

## JetBrains files

- 📁 [.idea](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea)
  - 📁
    [dictionaries](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/dictionaries)
    - [project.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/dictionaries/project.xml)
  - 📁
    [runConfigurations](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/runConfigurations)
    - [Test.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/runConfigurations/Test.xml)
  - [cmdcreate.iml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/cmdcreate.iml)
  - [misc.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/misc.xml)
  - [modules.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/modules.xml)
  - [rust.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/rust.xml)
  - [vcs.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/vcs.xml)
  - [workspace.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/workspace.xml)

## VSCode files

- 📁 [.vscode](https://github.com/owen-debiasio/cmdcreate/blob/main/.vscode)
  - [cmdcreate.code-workspace](https://github.com/owen-debiasio/cmdcreate/blob/main/.vscode/cmdcreate.code-workspace)
  - [launch.json](https://github.com/owen-debiasio/cmdcreate/blob/main/.vscode/launch.json)
  - [settings.json](https://github.com/owen-debiasio/cmdcreate/blob/main/.vscode/settings.json)

## Documentation

- 📁 [docs](https://github.com/owen-debiasio/cmdcreate/tree/main/docs)
  - 📁
    [resources](https://github.com/owen-debiasio/cmdcreate/tree/main/docs/resources)
    - [config_example.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/resources/config_example.toml)
  - [about.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/about.md)
  - [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/README.md)
  - [commands.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/commands.md)
  - [configurations.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/configurations.md)
  - [developing.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/developing.md)
  - [structure.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/structure.md)
  - [updates.md](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/updates.md)

## Developer Environment Setup

- 📁 [dev](https://github.com/owen-debiasio/cmdcreate/tree/main/dev)
  - [clean.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/clean.sh)
  - [format.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/format.sh)
  - [install.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/install.sh)
  - [setup_env.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/setup_env.sh)
  - [uninstall.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/uninstall.sh)

## Packaging cmdcreate

- 📁 [package](https://github.com/owen-debiasio/cmdcreate/tree/main/package)
  - [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/package/README.md)
  - [create_bin.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_bin.sh)
  - [create_deb.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_deb.sh)
  - [create_rpm.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_rpm.sh)
  - [package.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/package.sh)

## Main source code

- 📁 [src](https://github.com/owen-debiasio/cmdcreate/tree/main/src)
  - 📁
    [commands](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands)
    - [clean.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/clean.rs)
    - [create.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/create.rs)
    - [display.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/display.rs)
    - [edit.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/edit.rs)
    - [export.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/export.rs)
    - [favorite.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/favorite.rs)
    - [import.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/import.rs)
    - [list.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/list.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/mod.rs)
    - [remove.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/remove.rs)
    - [rename.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/rename.rs)
    - [search.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/search.rs)
    - [tools.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/tools.rs)
    - [update.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/update.rs)
  - 📁 [utils](https://github.com/owen-debiasio/cmdcreate/tree/main/src/utils)
    - [colors.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/colors.rs)
    - [fs.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs.rs)
    - [io.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/io.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/mod.rs)
    - [net.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/net.rs)
    - [sys.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys.rs)
  - [configs.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/configs.rs)
  - [init.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/init.rs)
  - [logger.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/logger.rs)
  - [main.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/main.rs)
  - [parse.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/parse.rs)
  - [usage.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/usage.rs)
  - [version.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/version.rs)
  - [meta.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/meta.rs)

## Testing cmdcreate

- 📁 [testing](https://github.com/owen-debiasio/cmdcreate/tree/main/testing)
  - 📁
    [scripts](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/scripts)
    - [backup.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/backup.py)
    - [create.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/create.py)
    - [display.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/display.py)
    - [edit.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/edit.py)
    - [listandfavorite.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/listandfavorite.py)
    - [remove.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/remove.py)
    - [rename.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/rename.py)
    - [search.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/search.py)
  - [lib.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/lib.py)
  - [main.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/main.py)
  - [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)
  - [shared.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/shared.py)

## Linting files

- [.markdownlint.json](https://github.com/owen-debiasio/cmdcreate/blob/main/.markdownlint.json)
- [.pylintrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.pylintrc)
- [.prettierrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.prettierrc)
- [.shellcheckrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.shellcheckrc)

## Other files

- [aur_update.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/aur_update.sh)
- [Cargo.lock](https://github.com/owen-debiasio/cmdcreate/blob/main/Cargo.lock)
- [Cargo.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/Cargo.toml)
- [changes.md](https://github.com/owen-debiasio/cmdcreate/blob/main/changes.md)
- [CODE_OF_CONDUCT.md](https://github.com/owen-debiasio/cmdcreate/blob/main/CODE_OF_CONDUCT.md)
- [codebook.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/codebook.toml)
- [CONTRIBUTING.md](https://github.com/owen-debiasio/cmdcreate/blob/main/CONTRIBUTING.md)
- [.gitignore](https://github.com/owen-debiasio/cmdcreate/blob/main/.gitignore)
- [LICENSE](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE)
- [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/README.md)
- [SECURITY.md](https://github.com/owen-debiasio/cmdcreate/blob/main/SECURITY.md)
