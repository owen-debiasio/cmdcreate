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

## Cargo configuration

- 📁 [.cargo](https://github.com/owen-debiasio/cmdcreate/tree/main/.cargo)
  - [config.toml](https://github.com/owen-debiasio/cmdcreate/tree/main/.cargo/config.toml)

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
    [codeStyles](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/codeStyles)
    - [codeStyleConfig.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/codeStyles/codeStyleConfig.xml)
    - [Project.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/codeStyles/Project.xml)
  - 📁
    [dictionaries](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/dictionaries)
    - [project.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/dictionaries/project.xml)
  - 📁
    [inspectionProfiles](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/inspectionProfiles)
    - [Project_Default.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/inspectionProfiles/Project_Default.xml)
  - 📁
    [runConfigurations](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/runConfigurations)
    - [Test.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/runConfigurations/Test.xml)
  - [.gitignore](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/.gitignore)
  - [cmdcreate.iml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/cmdcreate.iml)
  - [cmdcreate-testing.iml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/cmdcreate-testing.iml)
  - [discord.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/discord.xml)
  - [jsonSchemas.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/jsonSchemas.xml)
  - [misc.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/misc.xml)
  - [modules.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/modules.xml)
  - [pyProjectModel.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/pyProjectModel.xml)
  - [vcs.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/vcs.xml)

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

- 📁
  [commands](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands)
  - 📁
    [config](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/config)
    - [config_add.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/config/config_add.rs)
    - [config_remove.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/config/config_remove.rs)
    - [main.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/config/main.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/config/mod.rs)

  - 📁
    [core](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/core)
    - 📁
      [backup_actions](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/core/backup_actions)
      - [export.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/backup_actions/export.rs)
      - [import.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/backup_actions/import.rs)
      - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/backup_actions/mod.rs)

    - 📁
      [favorite](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/core/favorite)
      - [add_favorite.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/favorite/add_favorite.rs)
      - [main.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/favorite/main.rs)
      - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/favorite/mod.rs)
      - [remove_favorite.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/favorite/remove_favorite.rs)

    - [create.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/create.rs)
    - [display.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/display.rs)
    - [edit.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/edit.rs)
    - [list.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/list.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/mod.rs)
    - [remove.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/remove.rs)
    - [rename.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/rename.rs)
    - [search.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/core/search.rs)

  - 📁
    [updater](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/updater)
    - 📁
      [update_methods](https://github.com/owen-debiasio/cmdcreate/tree/main/src/commands/updater/update_methods)
      - [aur.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/update_methods/aur.rs)
      - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/update_methods/mod.rs)
      - [package.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/update_methods/package.rs)
      - [source.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/update_methods/source.rs)
    - [interactive.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/interactive.rs)
    - [main.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/main.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/updater/mod.rs)

  - [doc.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/doc.rs)
  - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/mod.rs)
  - [tools.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/tools.rs)

- 📁 [utils](https://github.com/owen-debiasio/cmdcreate/tree/main/src/utils)
  - 📁
    [sys](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/mod.rs)
    - [arguments.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/arguments.rs)
    - [cpu.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/cpu.rs)
    - [distro.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/distro.rs)
    - [env.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/env.rs)
    - [command.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys/command.rs)
  - 📁 [fs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/mod.rs)
    - [arguments.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/core.rs)
    - [cpu.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/init.rs)
    - [distro.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/misc.rs)
    - [env.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs/paths.rs)
  - [colors.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/colors.rs)
  - [git.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/git.rs)
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
      - [init_tests.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/tests/init_tests.py)
      - [run_tests.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/tests/run_tests.py)
    - [colors.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/colors.py)
    - [io_utils.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/io_utils.py)
    - [sys_utils.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/sys_utils.py)
    - [test_features.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/src/test_features.py)
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
