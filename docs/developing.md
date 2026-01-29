# Developing cmdcreate

Developing shouldn't be too hard, just install dependencies listed below.

## Installing dependencies

> [!NOTE]
> Packages I've listed may be incorrect. Please let me know if I'm missing any or got names wrong.

**Arch**

```
sudo pacman -S rustup curl openssl git python-black shfmt base-devel
```

**Debian/Ubuntu**

```
sudo apt install rustup curl libssl-dev libssl3 build-essential pkg-config git shfmt python3-black
```

**Fedora**

```
sudo dnf install rustup curl libssl-devel openssl-libs git shfmt python3-black
```

**Other**

- Install necessary packages from your package manager

## Setup enviorment

```
rustup default stable
git clone https://github.com/owen-debiasio/cmdcreate.git <desired directory>
cd <desired directory>
chmod +x *.sh
```

## Installing/Uninstalling test version

> [!NOTE]
> Provided command is for when you are in cmdcreate's parent directory.

*Installing*

```
./testing/install.sh
```

*Uninstalling*

```
./testing/uninstall.sh
```

## IDEs I recommend for their purposes

- **RustRover**
    - Main IDE for developing cmdcreate
- **PyCharm**
    - Working with the testing scripts
- **Visual Studio Code**
    - Editing and revising documentation

> [!NOTE]
> In the end, I don't care what you use, these are just my recommendations.
