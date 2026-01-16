# cmdcreate

Cmdcreate allows you to create custom commands for your Linux terminal without needing to enter the same "complex" commands over and over.

---
  
## Usage

```
Commands:
  create   <command>    <contents>   Create a custom command
  remove   <command>                 Remove a custom command
  edit     <command>                 Modify contents of a command
  list                               Display custom commands
  search   <command>                 Searches for matched command
  reset                              Removes all installed commands
  display  <command>                 Display contents of a command
  rename   <command>    <new name>   Renames a command
  favorite <add/remove> <command>    Renames a command
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

## Installation (Latest release)

> [!NOTE]
> Only supported on x86_64 architectures (unless built from source).

## Arch Linux

### Install through an AUR manager

```
yay -S cmdcreate
```

**Latest git**

```
yay -S cmdcreate-git
```

### Or clone AUR repo

```
git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/cmdcreate
cd ~/cmdcreate
makepkg -si
```

**Latest git (may be unstable)**

```
git clone --branch cmdcreate-git --single-branch https://github.com/archlinux/aur.git ~/cmdcreate-git
cd ~/cmdcreate-git
makepkg -si
```

## Debian/Ubuntu

You can get the `.deb` package file from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

## Fedora

You can get the `.rpm` package file from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

## Other

You can the get the `x86_64` binary from the latest [release](https://github.com/owen-debiasio/cmdcreate/releases)

---

## Building from source (Latest git)

### Dependencies

- Cargo
- Rustup

## Clone repo

```
git clone https://github.com/owen-debiasio/cmdcreate.git
cd cmdcreate
cargo build --release
sudo cp target/release/cmdcreate /usr/local/bin/cmdcreate
```

> [!NOTE]
> The binary should be located at `target/release/cmdcreate`

---

## Example command usage

### Creation

```
$ cmdcreate create say-hi "echo hi"

Success! Created command: say-hi

$ say-hi
hi
```

### Deletion

```
$ cmdcreate remove say-hi
Are you sure you want to delete command "say-hi"? (y/N)
y

Removed command "say-hi"
```

### List commands

```
$ cmdcreate list
Installed commands: (1 installed)
--------
say-hi
```

---

## Configuring cmdcreate

> [!NOTE]
> You can find the avaliable configurations [here](https://github.com/owen-debiasio/cmdcreate/blob/main/config_example.toml)

---

(C) 2026 Owen Debiasio

Licensed under GPL-2.0-only
