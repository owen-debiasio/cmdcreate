# Developing cmdcreate

Developing shouldn't be too hard, just follow the
instructions listed below.

## Setting up developing environment

### Run setup script **(recommended)**

You can just setup the environment through a Bash script I've written.

```bash
curl -sSL https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/dev/setup_env.sh | bash
```

### Manual setup

If you want to set things up yourself, don't worry. I've got instructions below.

#### Installing dependencies

> [!NOTE]  
> Packages I've listed may be incorrect. Please let me know if I'm missing any
> or got names wrong.

##### Arch

```bash
sudo pacman -S rustup curl openssl git python-black shfmt base-devel python-pylint shellcheck markdownlint-cli2
```

##### Debian/Ubuntu

```bash
sudo apt install curl libssl-dev libssl3 build-essential pkg-config git shfmt python3-black pylint shellcheck
```

###### Install Rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### Fedora

```bash
sudo dnf install rustup curl libssl-devel openssl-libs git shfmt python3-black pylint shellcheck
```

##### Other

- Install necessary packages from your package manager.

#### Install Markdownlint

> [!NOTE]  
> If you use Arch Linux, skip this step. You can install it using:
>
> `sudo pacman -S markdownlint-cli2`

##### Install NPM

###### Ubuntu/Debian

```bash
sudo apt install nodejs npm
```

###### Fedora

```bash
sudo dnf install nodejs npm
```

##### Download and install markdownlint

```bash
npm install -g markdownlint-cli2
```

#### Setup environment

```bash
rustup default stable
git clone https://github.com/owen-debiasio/cmdcreate.git <desired directory>
cd <desired directory>
find . -maxdepth 1 -name "*.sh" -exec chmod +x {} +
```

#### Installing/Uninstalling test version

> [!NOTE]  
> Provided command is for when you are in cmdcreate's parent directory.

##### Installing

```bash
./testing/install.sh
```

##### Uninstalling

```bash
./testing/uninstall.sh
```

#### IDEs I recommend for their purposes

- **RustRover**
  - Main IDE for developing cmdcreate
- **PyCharm**
  - Working with the testing scripts
- **Visual Studio Code**
  - Editing and revising documentation

#### Linters

I recommend using the following linters and formatters to keep the cmdcreate codebase clean:

##### Rust

- RustFmt
- Rust-Analyzer

##### Python

- PyLint
- Black

##### Shell Scripts

- Shfmt
- Shellcheck

##### Markdown, other

- Prettier
- Markdownlint

> [!NOTE]  
> In the end, I don't care what you use, these are just my recommendations.

#### Utilities I provide

##### Testing purposes

- [testing/install.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/install.sh)
  - **Install a test version of cmdcreate, like a non productive ready
    version.**
- [testing/uninstall.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/uninstall.sh)
  - **Remove that testing version.**
- [testing/features/main.py](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/features/main.py)
  - **Run a script that goes through the features of cmdcreate and runs them.**

> [!NOTE]  
> You can read more here:
> [Testing the Features of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/testing/README.md)

##### Packaging cmdcreate

- [package/format.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/format.sh)
  - **Formats all of the code in cmdcreate, including Bash, Python, and main
    Rust source code.**
- [package/create_bin.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_bin.sh)
  - **Packages a standalone `x86_64` binary.**
- [package/create_deb.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_deb.sh)
  - **Packages a `Debian` package.**
- [package/create_rpm.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/create_rpm.sh)
  - **Packages an `RPM` package.**
- [package/package.sh](https://github.com/owen-debiasio/cmdcreate/blob/main/package/package.sh)
  - **Goes through and packages cmdcreate to .deb, .rpm, and binary files.**

> [!NOTE]  
> You can read more here:
> [Packaging cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/package/README.md)

## Get to know cmdcreate

I highly recommend checking out
[The File Structure of cmdcreate](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/structure.md).
Get to know the file system a little bit.
