# cmdcreate

Cmdcreate allows you to create custom commands for your convenience without needing to enter the same "complex" commands over and over.

---
  
## Usage

```none
Commands:
  create   <command>    <contents>   Create a custom command
  remove   <command>                 Remove a custom command
  edit     <command>                 Modify contents of a command
  list                               Display custom commands
  search   <command>                 Searches for matched command
  reset                              Removes all installed commands
  display  <command>                 Display contents of a command
  rename   <command>    <new name>   Renames a command
  favorite <add/remove> <command>    Adds or removes a command from favorites
  repair                             Repairs installed commands if needed

 Update:
    check                            Checks for updates
    update                           Updates your system

 Backup:
    export <output directory>        Checks for updates
    import <file input>              Updates your system

Flags:
  -v, --version                      Displays version
  -c, --changelog                    Displays changelog
  -l, --license                      Displays license
  -d, --debugging                    Displays flags used for debugging

  Offline:
    -g, --get_offline_files          Downloads files for offline use
    -r, --remove_offline_files       Removes files for offline use
```

---

## Installation

> [!NOTE]
> Only supported on x86_64 architectures (unless built from source).

## Arch Linux

### Install through an AUR manager

#### Stable

```bash
yay -S cmdcreate
```

#### Latest git

> [!WARNING]
> Latest git builds have a possibility of being broken, unfinished, and potentially dangerous to your system. Use with caution.

```bash
yay -S cmdcreate-git
```

### Or clone AUR repo

#### Stable

```bash
git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate
cd ~/.cache/cmdcreate
makepkg -si
```

#### Latest git (may be broken/unstable)

> [!WARNING]
> Latest git builds have a possibility of being broken, unfinished, and potentially dangerous to your system. Use with caution.

```bash
git clone --branch cmdcreate-git --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate-git
cd ~/.cache/cmdcreate-git
makepkg -si
```

## Debian/Ubuntu

You can get the `.deb` package file from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

## Fedora

You can get the `.rpm` package file from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

## Other

You can the get the `x86_64` binary from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

---

## Building from source (Latest git), development

### Dependencies

> [!NOTE]
> You can find more about developing cmdcreate here:
> [Development Documentation](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/developing.md)

- Rustup
- Openssl (development, other)
- nano (not for building, but recommended, or you could use another text editor listed in this source code file: [cmdcreate/src/commands/edit.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/edit.rs) **(Lines 18-35)**
- git
- curl (not for building, but recommended)

> [!NOTE]
> You need the latest Rust version, so install rustup and run:
> `rustup default stable`

#### Arch Linux

```bash
sudo pacman -S rustup curl openssl nano git
```

#### Debian/Ubuntu

```bash
sudo apt install rustup curl libssl-dev libssl3 build-essential pkg-config git nano
```

#### Fedora

```bash
sudo dnf install rustup curl libssl-devel openssl-libs git nano
```

#### Other distros

- Install necessary packages from your package manager

## Download and install

```bash
git clone https://github.com/owen-debiasio/cmdcreate.git
cd cmdcreate
rustup default stable
cargo build --release
sudo install -Dm755 target/release/cmdcreate /usr/local/bin/cmdcreate
```

> [!NOTE]
> The binary should be located at `target/release/cmdcreate`

---

## Documentation and Resources

> [!NOTE]
> You can view the table of contents and information here: [Intro to Documentation and Resources](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/README.md)

---

(C) 2026 Owen Debiasio

Licensed under GPL v2.0
