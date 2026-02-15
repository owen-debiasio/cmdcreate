# Commands and how to use them

Here I provide the available commands present in cmdcreate and how they are
used. Information in this file is subject to change, and not all possibilities
are recorded. This file just covers the main ideas of what should happen and how
commands should function.

## Usage of all main commands

```none
Commands:
  create   <command>    <contents>   Create a custom command
  remove   <command>                 Remove a custom command
  edit     <command>                 Modify contents of a command
  list                               Display custom commands
  search   <command>                 Searches for matched command
  reset                              Removes all installed commands
  display  <command>                 Display contents of a command
  rename   <command>    <new name>   Renames a command
  favorite <add/remove> <command>    Adds or removes a command from favorites
  clean                              Can clean up old and unused files

 Backup:
    export <output directory>        Checks for updates
    import <file input>              Imports your exported commands
```

## Create

### Create a command

#### Usage

`cmdcreate create <name> "<contents of command>`

#### Example output

```bash
$ cmdcreate create abc "echo xyz"

Success! Created command: "abc"
```

## Remove

### Remove a command

#### Usage

`cmdcreate remove <name> (<contents of command>)`

#### Example output

##### Normal

```bash
$ cmdcreate remove abc
Are you sure you want to delete command "abc"?
(Y or N)
y

Removed command "abc"
```

##### Aborted

```bash
Are you sure you want to delete command "abc"?
(Y or N)
n

Aborted.
```

##### Forced

```bash
$ cmdcreate remove abc -f (or "--force")

Removed command "abc"
```

##### Attempt to remove command that doesn't exist

```bash
$ cmdcreate remove hijk

Error: Command "hijk" is not installed
```

> [!NOTE]  
> Can be forced when applying arg(s): `-f`, `--force`

## Edit

### Modify contents of a command

#### Usage

`cmdcreate edit <name>`

#### Example output

##### Normal (automatic)

```bash
$ cmdcreate edit abc

*Open up in text editor*
```

##### Using `EDITOR` environment variable

```bash
$ EDITOR=nano cmdcreate edit abc

*Open up in nano*
```

> [!NOTE]  
> Your text editor may not be detected! Check supported editors
> [here](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/edit.rs)
> (Lines 24-39)

## List

### Display installed commands

#### Usage

`cmdcreate list`

#### Example output

##### If commands are installed

```bash
$ cmdcreate list
Installed commands: (1)
--------
abc
```

##### If command is in favorites

```bash
$ cmdcreate list
Installed commands: (1)
--------
★ abc
```

> [!NOTE]  
> You can modify the favorite command indicator (default is "★") See
> [here](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/resources/config_example.toml)
> for more

##### If commands are not installed

```bash
$ cmdcreate list
Error: No commands are installed.
```

## Search

### Searches for matched command

#### Usage

`cmdcreate search <command to match>`

#### Example outputs

##### If one command is matched

```bash
$ cmdcreate search a
--------
abc
--------
Found one match for "a"
```

##### If more than one command is matched

```bash
$ cmdcreate search a
--------
abc
cba
--------
Found 2 matches for "a".
```

##### If no commands are matched

```bash
$ cmdcreate search d
Error: No installed commands contain: "d"
```

## Display

### Display contents of a command

#### Usage

`cmdcreate display <name>`

#### Example output

```bash
$ cmdcreate edit abc
*Print to output*
```

## Rename

### Rename a command

#### Usage

`cmdcreate rename <command> <new name>`

#### Example output

##### Rename command, no conflicts

```bash
$ cmdcreate rename abc xyz
Successfully renamed command "abc" to "xyz"
```

##### Rename command, name conflict

```bash
$ cmdcreate rename xyz cba
The new name (cba) is already installed! Do you want to delete it?
(Y or N)

y

Removed command "cba"
Successfully renamed command "xyz" to "cba"
```

##### Rename command, name conflict (aborted)

```bash
$ cmdcreate rename xyz cba
The new name (cba) is already installed! Do you want to delete it?
(Y or N)

n
Error: You need to remove the old command before proceeding!
```

##### Rename command, name conflict (forced)

```bash
$ cmdcreate rename xyz cba -f (or --force)

Removed command "cba"
Successfully renamed command "xyz" to "cba"
```

> [!NOTE]  
> Can be forced when applying arg(s): `-f`, `--force`

## Favorite

### Adds or removes a command from favorites

#### Usage

`cmdcreate favorite <add/remove> <command>`

#### Example output

##### Add command to favorites

```bash
$ cmdcreate favorite add cba
Command "cba" added to favorites.
```

##### Add command to favorites that's already in there

```bash
$ cmdcreate favorite add cba
Command "cba" is already in favorites.
```

##### Remove command from favorites

```bash
$ cmdcreate favorite remove cba
Command "cba" removed from favorites.
```

##### Remove command from favorites that isn't in there

```bash
$ cmdcreate favorite remove cba
Error: Command isn't in favorites: cba
```

### Repairs installed commands if needed

#### Usage

`cmdcreate repair`

#### Example output

##### Successful repair

```bash
$ cmdcreate repair
Repairing command: "cba"

Broken commands have been repaired.
```

##### No commands need repairs

```bash
$ cmdcreate repair
No commands needed repairs.
```

> [!NOTE]  
> `cmdcreate repair` repairs commands that are missing files in
> `/usr/local/bin/`

## Export

### Exports your installed commands

#### Usage

`cmdcreate export <output directory>`

#### Example output

##### Successful export

```bash
$ cmdcreate export ~/Downloads
Successfully exported commands to: "/home/owen/Downloads/export.cmdcreate"
```

> [!NOTE]  
> `cmdcreate export *` Exports files in this format:
> `<command> | <contents> (| favorite) (If command is a favorite)`

## Import

### Imports your installed commands

#### Usage

`cmdcreate import <input file>`

#### Example output

##### Successful import

```bash
$ cmdcreate import ~/Downloads/export.cmdcreate
Installing command: "xyz"
Installing command: "cba"
Command "cba" added to favorites. # (Shows if command was exported as favorite)
```

## Clean

### Clean up old and unused files

#### Usage

`cmdcreate clean`

#### Example output

##### Normal usage

```bash
$ cmdcreate clean
Do you want to clean cmdcreate?
(Y or N)
y

Do you want to delete old log files?
(Y or N)
y

Log files cleared.

Do you want to repair broken commands?
(Y or N)
y
No commands needed repairs.

Do you want to delete offline files?
(Y or N)
y

Offline files cleared.

Cleaned up cmdcreate.
```

##### Forced

```bash
$ cmdcreate clean -f

Cleaned up cmdcreate.
```
