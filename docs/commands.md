# Commands and how to use them

## Create

### Create a command

**Usage**

```
cmdcreate create <name> "<contents of command>"
```

**Example output**

```
$ cmdcreate create abc "echo xyz"

Success! Created command: "abc"
```

## Remove

### Remove a command

**Usage**

```
cmdcreate remove <name> (<contents of command>)
```

**Example outputs**

*Normal*

```
$ cmdcreate remove abc
Are you sure you want to delete command "abc"?
(Y or N)
y

Removed command "abc"
```

*Aborted*

```
Are you sure you want to delete command "abc"?
(Y or N)
n

Aborted.
```

*Forced*

```
$ cmdcreate remove abc -f (or "--force")

Removed command "abc"
```

*Attempt to remove command that doesn't exist*

```
$ cmdcreate remove hijk

Error: Command "hijk" is not installed
```

> [!NOTE]
> Can be forced when applying arg(s): `-f`, `--force`

## Edit

### Modify contents of a command

**Usage**

```
cmdcreate edit <name>
```

**Example output**

```
$ cmdcreate edit abc

*Open up in text editor*
```

> ![NOTE]
> Your text editor may not be detected! Check supported editors [here](https://github.com/owen-debiasio/cmdcreate/blob/main/src/commands/edit.rs) (Lines 18-35)

## List

### Display installed commands

**Usage**

```
cmdcreate list
```

**Example outputs**

*If commands are installed*

```
$ cmdcreate list
Installed commands: (1)
--------
abc
```

*If command is in favorites*

```
$ cmdcreate list
Installed commands: (1)
--------
★ abc
```

> [!NOTE]
> You can modify the favorite command indicator (default is "★")
> See [here](https://github.com/owen-debiasio/cmdcreate/blob/main/config_example.toml) for more

*If commands are not installed*

```
$ cmdcreate list
Error: No commands are installed.
```

## Search

### Searches for matched command

**Usage**

```
cmdcreate search <command to match>
```

**Example outputs**

*If one command is matched*

```
$ cmdcreate search a
--------
abc
--------
Found one match for "a"
```

*If more than command is matched*

```
$ cmdcreate search a
--------
abc
cba
--------
Found 2 matches for "a".
```

*If no commands are matched*

```
$ cmdcreate search d
Error: No installed commands contain: "d"
```

## Display

### Display contents of a command

**Usage**

```
cmdcreate edit <name>
```

**Example output**

```
$ cmdcreate edit abc

*Open up in text editor*
```

## Rename

### Rename a command

**Usage**

```
cmdcreate rename <command> <new name>
```

**Example output**

*Rename command, no conflicts*

```
$ cmdcreate rename abc xyz
Successfully renamed command "abc" to "xyz"
```

*Rename command, name conflict*

```
$ cmdcreate rename xyz cba
The new name (cba) is already installed! Do you want to delete it?
(Y or N)

y

Removed command "cba"
Successfully renamed command "xyz" to "cba"
```

*Rename command, name conflict (aborted)*

```
$ cmdcreate rename xyz cba
The new name (cba) is already installed! Do you want to delete it?
(Y or N)

n
Error: You need to remove the old command before proceeding! 
```

*Rename command, name conflict (forced)*

```
$ cmdcreate rename xyz cba -f (or --force)

Removed command "cba"
Successfully renamed command "xyz" to "cba"
```

> [!NOTE]
> Can be forced when applying arg(s): `-f`, `--force`

## Favorite

### Adds or removes a command from favorites

**Usage**

```
cmdcreate favorite <add/remove> <command>
```

**Example output**

**Add command to favorites**

```
$ cmdcreate favorite add cba
Command "cba" added to favorites.
```

**Add command to favorites thats already in there**

```
$ cmdcreate favorite add cba
Command "cba" is already in favorites.
```

**Remove command from favorites**

```
$ cmdcreate favorite remove cba
Command "cba" removed from favorites.
```

**Remove command from favorites that isn't in there**

```
$ cmdcreate favorite remove cba
Error: Command isn't in favorites: cba
```

## Repair

### Repairs installed commands if needed

**Usage**

```
cmdcreate repair
```

**Example output**

*Successful repair*

```
$ cmdcreate repair
Repairing command: "cba"

Broken commands have been repaired.
```

*No commands need repairs*

```
$ cmdcreate repair
No commands needed repairs.
```

> [!NOTE]
> `cmdcreate repair` repairs commands that are missing files in `/usr/local/bin/`

## Export

### Exports your installed commands

**Usage**

```
cmdcreate export <output directory>
```

**Example output**

*Successful export*

```
$ cmdcreate export ~/Downloads
Successfully exported commands to: "/home/owen/Downloads/export.cmdcreate"
```

> [!NOTE]
> `cmdcreate export *` Exports files in this format:
> `<command> | <contents> (| favorite) (If command is a favorite)`

## Import

### Imports your installed commands

**Usage**

```
cmdcreate import <input file>
```

**Example output**

*Successful import*

```
$ cmdcreate import ~/Downloads/export.cmdcreate
Installing command: "xyz"
Installing command: "cba"
Command "cba" added to favorites. # (Shows if command was exported as favorite)
```
