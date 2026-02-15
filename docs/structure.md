# The File Structure of cmdcreate

Here I provide the organization of the cmdcreate Github repository. Subject to
change.

## Github templates, actions, workflows

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
    - [Run_cmdcreate_Tests.xml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/runConfigurations/Run_cmdcreate_Tests.xml)
  - [.cmdcreate.iml](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/cmdcreate.iml)
  - [.gitignore](https://github.com/owen-debiasio/cmdcreate/blob/main/.idea/.gitignore)
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
  - [format.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/clean.sh)
  - [format.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/dev/format.sh)
  - [setup_env.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/setup_env.sh)

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
    - [repair.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/repair.rs)
    - [search.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/search.rs)
    - [tools.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/tools.rs)
    - [update.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/update.rs)
  - 📁 [utils](https://github.com/owen-debiasio/cmdcreate/tree/main/src/utils)
    - [colors.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/colors.rs)
    - [fs.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/fs.rs)
    - [io.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/io.rs)
    - [net.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/net.rs)
    - [mod.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/mod.rs)
    - [sys.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/utils/sys.rs)
  - [configs.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/configs.rs)
  - [init.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/init.rs)
  - [logger.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/logger.rs)
  - [main.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/main.rs)
  - [parse.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/parse.rs)
  - [usage.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/usage.rs)
  - [version.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/version.rs)

## Testing cmdcreate

- 📁 [testing](https://github.com/owen-debiasio/cmdcreate/tree/main/testing)
  - 📁
    [features](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/features)
    - 📁
      [scripts](https://github.com/owen-debiasio/cmdcreate/tree/main/testing/features/scripts)
      - [backup.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/backup.py)
      - [create.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/create.py)
      - [display.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/display.py)
      - [edit.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/edit.py)
      - [listandfavorite.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/listandfavorite.py)
      - [remove.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/remove.py)
      - [rename.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/rename.py)
      - [repair.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/repair.py)
      - [search.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/scripts/search.py)
    - [lib.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/lib.py)
    - [main.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/main.py)
    - [shared.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/shared.py)
  - [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)
  - [install.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/install.sh)
  - [uninstall.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/uninstall.sh)

## Linting files

- [.markdownlint.json](https://github.com/owen-debiasio/cmdcreate/blob/main/.markdownlint.json)
- [.pylintrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.pylintrc)
- [.prettierrc](https://github.com/owen-debiasio/cmdcreate/blob/main/.prettierrc)

## Other files

- [CODE_OF_CONDUCT.md](https://github.com/owen-debiasio/cmdcreate/blob/main/CODE_OF_CONDUCT.md)
- [CONTRIBUTING.md](https://github.com/owen-debiasio/cmdcreate/blob/main/CONTRIBUTING.md)
- [Cargo.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/Cargo.toml)
- [LICENSE](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE)
- [README.md](https://github.com/owen-debiasio/cmdcreate/blob/main/README.md)
- [SECURITY.md](https://github.com/owen-debiasio/cmdcreate/blob/main/SECURITY.md)
- [changes.md](https://github.com/owen-debiasio/cmdcreate/blob/main/changes.md)
- [codebook.toml](https://github.com/owen-debiasio/cmdcreate/blob/main/codebook.toml)
- [.gitignore](https://github.com/owen-debiasio/cmdcreate/blob/main/.gitignore)
