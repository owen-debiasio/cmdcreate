# Developing cmdcreate

Developing shouldn't be too hard, just follow the instructions listed below.

## Setting up developing environment

### Run setup script **(recommended)**

You can set up the entire development environment using the provided Bash script.

```bash
curl -sSL https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/dev/setup_env.sh -o /tmp/setup_env.sh
bash -i /tmp/setup_env.sh
```

This script will:

- Detect your Linux distribution
- Install all required dependencies
- Install Rust using `rustup`
- Clone the `cmdcreate` repository
- Prepare shell scripts for development

---

### Manual setup

If you prefer setting everything up yourself, follow the instructions below.

#### Installing dependencies

> [!NOTE]  
> Package names vary between distributions. The lists below match what the setup
> script installs.

##### Arch Linux

```bash
sudo pacman -S --needed \
  rustup curl openssl git base-devel \
  shfmt shellcheck \
  python-black python-pylint \
  nodejs npm markdownlint-cli2
```

##### Debian / Ubuntu

```bash
sudo apt update
sudo apt install -y \
  curl libssl-dev build-essential pkg-config git \
  shfmt shellcheck \
  black pylint \
  nodejs npm
```

##### Fedora

```bash
sudo dnf install -y \
  curl openssl-devel git \
  shfmt ShellCheck \
  python3-black python3-pylint \
  nodejs npm
```

---

#### Install Rust (Debian / Ubuntu / Fedora)

Rust is installed using `rustup`.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

After installation, load Cargo into your shell:

```bash
source "$HOME/.cargo/env"
```

Set the default toolchain:

```bash
rustup default stable
```

---

#### Install Markdownlint

> [!NOTE]  
> Arch Linux users can skip this step if `markdownlint-cli2` was installed via
> `pacman`.

```bash
npm install -g markdownlint-cli2
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
./testing/install.sh
```

##### Uninstalling

```bash
./testing/uninstall.sh
```

---

## IDEs I recommend for their purposes

- **RustRover**
  - Primary IDE for developing cmdcreate
- **PyCharm**
  - Working with testing scripts
- **Visual Studio Code**
  - Editing and revising documentation

---

## Linters

I recommend using the following linters and formatters to keep the cmdcreate
codebase clean:

### Rust

- RustFmt
- Rust-Analyzer

### Python

- PyLint
- Black

### Shell Scripts

- Shfmt
- Shellcheck

### Markdown / Other

- Prettier
- Markdownlint

> [!NOTE]  
> In the end, I don't care what you use, these are just my recommendations.

---

## Utilities I provide

### Testing purposes

- `testing/install.sh`  
  Install a test (non-production) version of cmdcreate.

- `testing/uninstall.sh`  
  Remove the testing version.

- `testing/features/main.py`  
  Run a script that exercises cmdcreate features.

> [!NOTE]  
> You can read more here: [Testing the Features of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)

---

### Packaging cmdcreate

- `package/format.sh`  
  Formats all Bash, Python, and Rust code.

- `package/create_bin.sh`  
  Packages a standalone `x86_64` binary.

- `package/create_deb.sh`  
  Packages a Debian `.deb`.

- `package/create_rpm.sh`  
  Packages an RPM.

- `package/package.sh`  
  Builds all supported package formats.

> [!NOTE]  
> You can read more here: [Packaging cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/package/README.md)

---

## Get to know cmdcreate

I highly recommend checking out [The File Structure of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/structure.md). Get familiar with the project layout before making changes.
