# cmdcreate

Create your own custom commands for your convenience without needing to enter
the same "complex" commands over and over.

---

## Documentation and Resources

I do my best to provide as much documentation as I can. You can view the table
of contents and information here:
[Intro to Documentation and Resources](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/README.md)

---

## Installation

> [!NOTE]  
> If you are looking for information on updates and how to update, check out
> **[Information Regarding cmdcreate and Updates](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/updates.md)**.
> While you are at it, also check out the
> **[Version Support Information](https://github.com/owen-debiasio/cmdcreate/blob/main/SECURITY.md)**
> for more information about updates.

> [!WARNING]  
> cmdcreate is mainly supported on systems with the `x86_64` CPU architecture.
> It should work, but currently the only way I recommend to install it is to
> build it from source. If you follow the
> **[steps for building from source](https://github.com/owen-debiasio/cmdcreate/blob/main/README.md#building-from-source-latest-git-development)**,
> (or using the
> [AUR](https://github.com/owen-debiasio/cmdcreate/blob/main/README.md#arch-linux)),
> you should be fine.

### Arch Linux

#### Install through an AUR manager

##### Stable

```bash
yay -S cmdcreate
```

##### Latest git

> [!WARNING]  
> Latest git builds have a possibility of being broken, unfinished, and
> potentially dangerous to your system. Use with caution.

```bash
yay -S cmdcreate-git
```

#### Clone AUR repo

##### Stable

```bash
git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate
cd ~/.cache/cmdcreate
makepkg -si
```

##### Latest git

> [!WARNING]  
> Latest git builds have a possibility of being broken, unfinished, and
> potentially dangerous to your system. Use with caution.

```bash
git clone --branch cmdcreate-git --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate-git
cd ~/.cache/cmdcreate-git
makepkg -si
```

### Debian/Ubuntu

You can get the `.deb` package file here:
[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)

### Fedora

You can get the `.rpm` package file file here:
[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)

### Other

You can the get the standalone `x86_64` binary file here:
[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)

---

## Building from source (Latest git), development

### Dependencies

> [!NOTE]  
> You can find more about developing cmdcreate here:
> [Development Documentation](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/developing.md)

- Rustup
- Openssl (development, other)
- nano (not for building, but recommended, or you could use another text editor
  listed in this source code file:
  [cmdcreate/src/commands/edit.rs](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/edit.rs)
  **(Lines 24-39)**
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
sudo apt install curl libssl-dev libssl3 build-essential pkg-config git nano
```

##### Install Rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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

(C) 2026 Owen Debiasio

Licensed under
**[MIT](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE.md)**
