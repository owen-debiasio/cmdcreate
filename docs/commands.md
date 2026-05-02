<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# Commands and how to use them

Here I provide the available commands present in cmdcreate and how they are
used. Information in this file is subject to change, and not all possibilities
are recorded. This file just covers the main ideas of what should happen and how
commands should function.

## Usage of all main commands

```none
Commands:
  create     Create a command
  remove     Remove a command
  edit       Modify contents of a command
  list       Display installed commands
  search     Searches for matched command
  display    Display contents of a command
  rename     Renames a command
  favorite   Adds or removes a command from favorites
  config     Manage your configurations for cmdcreate
  doc        View various documentation references

  Backup:
    export   Exports your installed commands
    import   Imports your exported commands
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

## Config

### Manage your configurations for cmdcreate

#### Usage

`cmdcreate config <help/example/add/remove/edit/display> <category> <value(="setting")>`

#### Example output

##### Successful addition

```bash
$ sudo cmdcreate config add sys shell="zsh"
Successfully updated config: shell set to "zsh".
```

##### Successful removal

```bash
$ sudo cmdcreate config remove sys shell
Successfully removed config: shell from sys.
```

> [!NOTE]  
> If you remove the last remaining key in a category, cmdcreate will
> automatically remove the category header to keep the file clean.

##### Unsuccessful addition

###### Invalid format

```bash
$ sudo cmdcreate config add sys shell
Error: Invalid format! Use key=value
Example: cmdcreate config add sys shell="bash"
```

###### Key not found

```bash
$ sudo cmdcreate config remove sys fake_key
Error: Config key 'fake_key' not found in category 'sys'.
```

###### Edit config

```bash
$ sudo cmdcreate config edit
# The file `/etc/cmdcreate.toml` will open in a text editor
```

###### Display config

```bash
$ sudo cmdcreate config display
# The file `/etc/cmdcreate.toml` will be opened in a pager
```

###### Example

```bash
$ sudo cmdcreate config example
# An example config will be opened in a pager
```

###### Help

> [!NOTE]  
> You could also run: `sudo cmdcreate doc configurations`

```bash
$ sudo cmdcreate config help
# Documentation for configurations will be opened in a pager
```

## Doc

### Have documentation for cmdcreate at your fingertips

#### Usage

`cmdcreate doc <list>/<information>`

#### Example output

##### View information of your choice

> [!NOTE]  
> The resource `about` is being used in this example

```bash
$ sudo cmdcreate doc about
# Documentation for your information your request will be opened in a pager
```

##### List available information to view

```bash
$ sudo cmdcreate doc list
> Available options:
----Main Repository Information----
main              The main README file.
changelog         The complete version history of cmdcreate.
license           The license for cmdcreate.
security          Security information.
contributing      Information about contributing.
code_of_conduct   View information about the code of conduct.
----Main Documentation----
intro             Intro to the documentation.
about             About cmdcreate and its purpose.
commands          About the current commands in cmdcreate.
configurations    Information of configuring cmdcreate.
developing        Information on developing cmdcreate.
structure         The file structure of cmdcreate.
updates           Information on updates.
----Other Information for Development----
testing           Information about testing the features of cmdcreate.
packaging         Information about packaging cmdcreate.
```
