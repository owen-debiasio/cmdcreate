<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# cmdcreate

Easily create and manage your own custom commands

[Badge Commits]:
  https://img.shields.io/github/commit-activity/m/owen-debiasio/cmdcreate?label=Commits
[Badge License]: https://img.shields.io/badge/License-GPLv3-blue.svg
[Badge Issues]: https://img.shields.io/github/issues/owen-debiasio/cmdcreate

![Badge Commits] ![Badge License] ![Badge Issues]

---

<h2>Documentation and Resources</h2>

I do my best to provide as much documentation as I can. You can view the table
of contents and information here:
[Intro to Documentation and Resources](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/README.md)

---

## Installation

> [!NOTE]  
> If you are looking for information on updates and how to update, check out
> [Information Regarding cmdcreate and Updates](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/updates.md).
> While you are at it, also check out the
> [Version Support Information](https://github.com/owen-debiasio/cmdcreate/tree/main?tab=security-ov-file#)
> for more information about updates.

### Arch Linux (AUR manager)

#### Stable

```bash
yay -S cmdcreate
```

#### Latest git

> [!WARNING]  
> Latest git builds have a possibility of being broken, unfinished, and
> potentially dangerous to your system. Use with caution.

```bash
yay -S cmdcreate-git
```

### Manual Package Install

> [!NOTE]  
> If using the standalone binary, you are able to run it portably, without
> installation if you wish.

You can find `.deb`, `.rpm` and standalone binary packages in the
[latest release](https://github.com/owen-debiasio/cmdcreate/releases/latest).

### Build From Source

#### Dependencies

##### Gather Prerequisites

- Rust (`stable` branch)
  - Cargo
  - Rustup
  - [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild)
- Zig

##### Setup

> [!NOTE]  
> I recommend using the stable branch, but you can choose whatever if needed.

```bash
rustup update stable
```

##### Add needed target

> [!IMPORTANT]  
> The target you choose is based on your CPU architecture. Choose accordingly.

###### x86_64

```bash
rustup target add x86_64-unknown-linux-musl
```

###### x86, i686, i386

```bash
rustup target add i686-unknown-linux-musl
```

###### aarch64 (ARM64)

```bash
rustup target add aarch64-unknown-linux-musl
```

###### arm (32-bit, armv7)

```bash
rustup target add armv7-unknown-linux-musleabihf
```

##### Install `cargo-zigbuild`

> [!IMPORTANT]  
> Make sure you have `zig` installed, or else it will not build

```bash
cargo install cargo-zigbuild
```

#### Build & Install

##### Clone and enter repository

```bash
git clone https://github.com/owen-debiasio/cmdcreate.git --depth=1
cd cmdcreate
```

###### Automatic **(recommended)**

```bash
./dev/install.sh
```

###### Manual

> [!WARNING]  
> Be prepared to manually troubleshoot issues.

> [!NOTE]  
> The resulting binary after you build will be located in the following:
>
> `target/<target of your choice>/release/cmdcreate`

```bash
cargo zigbuild --release --target <target of your choice>
sudo install -Dm755 target/<target of your choice>/release/cmdcreate /usr/local/bin/cmdcreate
```

---

(C) 2026 Owen Debiasio

Licensed under
**[GPL Version 3](https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE)**
