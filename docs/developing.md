<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# Developing cmdcreate

Developing shouldn't be too hard, just follow the instructions listed below.

## Setting up developing environment

### Run setup script **(recommended)**

You can set up the entire development environment using the provided Bash
script.

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/dev/setup_env.sh)
```

This script will:

- Detect your Linux distribution and install necesarry packages.
- Install `Rust` and the `Zig` compiler
- Install `cargo-zigbuild` for 32-bit binaries.
- Clone the repository and configure `musl`

---

### Manual setup

#### Installing dependencies

> [!IMPORTANT]  
> To build cmdcreate successfully (especially the `ring` crate), you MUST have
> `Zig` installed.

##### Arch Linux

```bash
sudo pacman -S --needed --noconfirm \
    rustup curl openssl git base-devel \
    shfmt shellcheck bash-language-server \
    python-black python-pylint python-lsp-server \
    nodejs npm markdownlint-cli2 prettier marksman vscode-json-languageserver \
    rpm-tools dpkg
```

##### Debian / Ubuntu

```bash
sudo apt-get install -y \
    curl libssl-dev build-essential pkg-config git \
    shfmt shellcheck \
    black pylint \
    nodejs npm rpm dpkg-dev
```

##### Fedora

```bash
sudo dnf install -y \
    curl openssl-devel git gcc gcc-c++ make \
    shfmt ShellCheck nodejs-bash-language-server \
    python3-black python3-pylint \
    nodejs npm rpm-build dpkg-dev
```

---

#### Set up Rust

##### Install Rust

> [!NOTE]  
> If you installed `rustup` on Arch using `pacman` you can skip this step.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, load Cargo into your shell:

```bash
source "$HOME/.cargo/env"
```

Set the default toolchain:

```bash
rustup default stable
```

##### Install dependencies

```bash
rustup component add rust-analyzer
rustup target add x86_64-unknown-linux-musl i686-unknown-linux-musl
cargo install cargo-zigbuild
```

---

#### Install Markdownlint and Prettier

> [!NOTE]  
> Arch Linux users can skip this step if `markdownlint-cli2` and `prettier` were
> installed via `pacman`.

```bash
sudo npm install -g prettier markdownlint-cli2@0.13.0
```

---

#### Setup environment

Clone the repository and prepare the scripts:

```bash
git clone https://github.com/owen-debiasio/cmdcreate.git <desired directory>
cd <desired directory>
find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +
```

The `<desired directory>` may include paths such as `~/dev/cmdcreate` or
`$HOME/dev/cmdcreate`.

---

#### Installing / Uninstalling test version

> [!NOTE]  
> Run these commands from the **parent directory** of `cmdcreate`.

##### Installing

```bash
./dev/install.sh
```

##### Uninstalling

```bash
./dev/uninstall.sh
```

---

---

## Utilities I provide

### Standard Utilities

- `dev/setup_env.sh` Sets up the development environment.

- `dev/clean.sh` Cleans up the development environment.

- `dev/format.sh` Formats all Bash, Python, and Rust code.

- `dev/install.sh` Install a test (non-production) version of cmdcreate.

- `dev/uninstall.sh` Remove the testing version.

### Testing Purposes

- `testing/src/test_cmdcreate.py` Run a script that exercises cmdcreate
  features.

> [!NOTE]  
> You can read more here:
> [Testing the Features of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)

---

### Packaging cmdcreate

> [!NOTE]  
> Packaged and standalone binaries are automatically moved to `~/Downloads.`
> Feel free to modify the script to change this. Might break things though.

#### Automatic packaging

- `package/package.sh` Builds all packages automatically (`.deb`, `.rpm`,
  binary). Both `x86` and `x64`.

#### Manual packaging

##### Debian package

- `package/create_deb.sh` Packages a 32 and 64 bit `.deb` package.

##### `.rpm` package

- `package/create_rpm.sh` Packages a 32 and 64 bit `.rpm` package.

##### Standalone binary

###### Via automatic script

- `package/create_rpm.sh` Compiles both 32 and 64 bit binaries.

###### Manually

```bash
export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-fno-sanitize=all"
export CFLAGS="-O3 -pipe"

cargo zigbuild --release --target $(uname -m)-unknown-linux-musl
```

> [!NOTE]  
> You can read more here:
> [Packaging cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/package/README.md)

### Troubleshooting

#### `Invalid -flto mode: 'flto=auto'`

This is caused by Arch Linux system flags in `/etc/makepkg.conf`. `Zig (Clang)`
does not recognize the `'auto'` value. The development scripts override `CFLAGS`
to solve this, but if building manually, ensure `CFLAGS` does not contain
`-flto=auto`.

#### `Undefined symbol: ring_core...`

This occurs when the assembly components of the `ring` crate are not linked
correctly. Ensure:

1. You are using `cargo zigbuild`
2. `RUSTFLAGS` includes `-C link-arg=-fno-sanitize=all`.
3. You have cleared any local `.cargo/config.toml` that might be forcing a
   different linker.

---

## Get to know cmdcreate

I highly recommend checking out
[The File Structure of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/structure.md).
Get familiar with the project layout before making changes.
