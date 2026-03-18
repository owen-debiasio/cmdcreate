<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# cmdcreate

Create your own custom commands for your convenience without needing to enter
the same "complex" commands over and over.

[Badge Commits]:
  https://img.shields.io/github/commit-activity/m/owen-debiasio/cmdcreate?label=Commits
[Badge License]: https://img.shields.io/badge/License-GPLv3-blue.svg
[Badge Issues]: https://img.shields.io/github/issues/owen-debiasio/cmdcreate

![Badge Commits] ![Badge License] ![Badge Issues]

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
> **[Version Support Information](https://github.com/owen-debiasio/cmdcreate/tree/main?tab=security-ov-file#)**
> for more information about updates.

> [!WARNING]  
> cmdcreate is mainly supported on systems with the `x86_64` CPU architecture.
> It should work, but currently the only way I recommend to install it is to
> build it from source. If you follow the
> **[steps for building from source](https://github.com/owen-debiasio/cmdcreate?tab=readme-ov-file#building-from-source-latest-git)**,
> (or using the
> [AUR](https://github.com/owen-debiasio/cmdcreate/blob/main/README.md#arch-linux)),
> you should be fine.

## Packaging information

[![Packaging status](https://repology.org/badge/vertical-allrepos/cmdcreate.svg)](https://repology.org/project/cmdcreate/versions)

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

#### Package installation (recommended)

You can get the `.deb` package file here:
**[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)**

#### Using curl

```bash
curl -Lf -o /tmp/cmdcreate-{version}-linux-x86_64.deb \
https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{version}-linux-x86_64.deb && \
sudo dpkg -i /tmp/cmdcreate-{version}-linux-x86_64.deb
```

### Fedora

#### Package installation (recommended)

You can get the `.rpm` package file file here:
**[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)**

#### Using curl

```bash
curl -Lf -o /tmp/cmdcreate-{version}-linux-x86_64.rpm \
https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{version}-linux-x86_64.rpm && \
sudo rpm -Uvh /tmp/cmdcreate-{version}-linux-x86_64.rpm
```

### Other

#### Standalone binary

You can the get the standalone `x86_64` binary file here:
**[Latest Release](https://github.com/owen-debiasio/cmdcreate/releases/latest)**

#### Using curl

```bash
curl -Lf -o /tmp/cmdcreate-{version}-linux-x86_64-bin \
https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{version}-linux-x86_64-bin && \
sudo install -Dm755 /tmp/cmdcreate-{version}-linux-x86_64-bin /usr/bin/cmdcreate
```

---

## Building from source (Latest git)

### Dependencies

> [!NOTE]  
> You can find more about developing cmdcreate here:
> [Development Documentation](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/developing.md)

- Rustup
- Openssl
- git
- curl

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
sudo dnf install curl libssl-devel openssl-libs git nano
```

##### Install Rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Other distros

- Install necessary packages from your package manager

### Download and install

```bash
git clone https://github.com/owen-debiasio/cmdcreate.git
cd cmdcreate
rustup default stable
cargo build --release
sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate
cd "$HOME"
rm -rf cmdcreate
```

---

(C) 2026 Owen Debiasio

Licensed under
**[GPL Version 3](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE)**
