# cmdcreate v0.8.1
Cmdcreate allows you to create custom commands for your Linux terminal without needing to enter the same "complex" commands over and over (unless if your are lazy like me).
  
## Usage:

```
Commands:
  create   <command>     <contents>  Create a custom command
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
  -s, --supported_editors            Displays supported text editors
  -c, --changelog                    Displays changelog
  -l, --license                      Displays license
  -d, --debugging                    Displays flags used for debugging

  Offline:
    -g, --get_offline_files          Downloads files for offline use
    -r, --remove_offline_files       Removes files for offline use
```

# Installation
**NOTE: Only supported on x86_64 architectures.**

## Arch Linux
Install through the AUR

`$ yay -S cmdcreate` (Or another AUR manager)

## Debian/Ubuntu
You can get the `.deb` file from the latest [release](https://github.com/Meme-Supplier/cmdcreate/releases)

## Fedora
You can get the `.rpm` file from the latest [release](https://github.com/Meme-Supplier/cmdcreate/releases)

## Other
You can the get the `x86_64` binary from the latest [release](https://github.com/Meme-Supplier/cmdcreate/releases)

## You can also run it portably

# Example usage

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
