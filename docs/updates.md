# Information Regarding cmdcreate and Updates

Here I provide information regarding update cycles and how you should update
cmdcreate based on your situation.

## Packaging information

[![Packaging status](https://repology.org/badge/vertical-allrepos/cmdcreate.svg)](https://repology.org/project/cmdcreate/versions)

## Checking for updates

You can also manually check for updates. You must run this manually because
there is currently no way to automate this.

```bash
cmdcreate check
```

> [!NOTE]
>
> - This only checks for the latest stable version, not latest git commits.
> - You can't update on Arch-based distros via this method. Please use an AUR manager.
> - Update methods are only listed if they apply to your system

## Updating cmdcreate

### Arch Linux

#### Install through an AUR manager

##### Stable

```bash
yay -Syyu cmdcreate
```

##### Latest git

> [!WARNING]  
> Latest git builds have a possibility of being broken, unfinished, and
> potentially dangerous to your system. Use with caution.

```bash
yay -Syyu cmdcreate-git
```

#### Clone AUR repo

##### Stable

```bash
git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate
cd ~/.cache/cmdcreate
makepkg -si
```

##### Latest git (may be broken/unstable)

> [!WARNING]  
> Latest git builds have a possibility of being broken, unfinished, and
> potentially dangerous to your system. Use with caution.

```bash
git clone --branch cmdcreate-git --single-branch https://github.com/archlinux/aur.git ~/.cache/cmdcreate-git
cd ~/.cache/cmdcreate-git
makepkg -si
```

### Everyone else

This applies to people running Ubuntu, Fedora, Debian, and any other distro that
isn't Arch Linux.

#### How to update

I highly recommend running the built in tool for updating cmdcreate:

```bash
cmdcreate update
```

##### Available options

- Download and install through the latest `Ubuntu/Debian` package file (`.deb`)
  - **Stable version only**
  - **x86_64 architectures only**
- Download and install through the latest `Fedora` package file (`.rpm`)
  - **Stable version only**
  - **x86_64 architectures only**
- Download the raw cmdcreate binary file
  - **Stable version only**
  - **x86_64 architectures only**
  - **Should work on any distro**
- Build from source
  - **Latest git only**
  - **Should work on any CPU architecture**
  - **Should work on any distro**

<!-- markdownlint-disable-file MD032 -->

> [!WARNING]
>
> - Latest git builds have a possibility of being broken, unfinished, and
>   potentially dangerous to your system. Use with caution.
> - Building from source on `Debian/Ubuntu` systems may involve manual
>   intervention

> [!NOTE]  
> This tool is always available to you regardless of your distro

## Update cycles and when cmdcreate gets updated

I usually update cmdcreate every few days, or maybe weeks if I've been busy.

It's very rare that I add any new features in one update. I usually add less
features over time, but I guess the frequent updates make up for it.

### How I version cmdcreate

| Update type | Formatting | Frequency      |
| ----------- | ---------- | -------------- |
| Normal      | x.x.1      | Every few days |
| Semi-Major  | x.1.x      | 1-3 Months     |
| Major       | 1.x.x      | About a year   |

#### More information

You can find more information about updates here:
**[Version Support Information](https://github.com/owen-debiasio/cmdcreate/blob/main/SECURITY.md)**
