<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# The File Structure of cmdcreate

Here I provide the organization of the cmdcreate GitHub repository. Subject to
change.

> [!NOTE]  
> File structure may be inaccurate. Please let me know if it needs to be updated

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
    - [config.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/config.rs)
    - [clean.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/clean.rs)
    - [create.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/create.rs)
    - [display.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/display.rs)
    - [doc.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/doc.rs)
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
    - 📁
      [sys](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/)
      - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/mod.rs)
      - [arguments.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/arguments.rs)
      - [cpu.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/cpu.rs)
      - [distro.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/distro.rs)
      - [env.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/env.rs)
      - [command.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/command.rs)
    - [colors.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/colors.rs)
    - [fs.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs.rs)
    - [io.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/io.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/mod.rs)
    - [net.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/net.rs)
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
  - 📁 [src](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/src/)
    - 📁
      [tests](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/tests/)
      - [init_tests.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/init_tests.py)
      - [run_tests.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/run_tests.py)
    - [colors.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/colors.py)
    - [io_utils.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/io_utils.py)
    - [sys_utils.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/sys_utils.py)
    - [test_features.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/scripts/test_features.py)
  - [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)
  - [pyproject.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/pyproject.toml)

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
