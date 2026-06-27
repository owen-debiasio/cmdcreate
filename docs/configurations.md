<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# Configuring cmdcreate

You are able to configure cmdcreate in various ways. All are listed here.
[example configuration](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/resources/config_example.toml).

## Categories

## self

### disable_root_usage

#### Prevent running cmdcreate as root

- **Default:** `false`

##### Example

```toml
[self]
disable_root_usage = "true"
```

> [!WARNING]  
> Can only be enabled when running cmdcreate as root, then can only be disabled
> by modifying `/etc/cmdcreate.conf`.

> [!TIP]  
> This setting is useful for `immutable`/`atomic` distros.

> [!NOTE]  
> This also disables the ability to update via most methods.

## sys

### shell

#### Your preferred shell that cmdcreate uses to run shell commands

- **Default:** `sh`

##### Example

```toml
[sys]
shell = "bash"
```

> [!NOTE]  
> I recommend using `bash` just for compatibility’s sake.

## logs

### time_format

#### Formatting of time present in logs

- **Default:** `%Y-%m-%d %H:%M:%S`

##### Example

```toml
[logs]
time_format = "%Y-%m-%d %H:%I:%S"
```

### verbose

#### If "true", logs will be printed to output

- **Default:** `false`

##### Example

```toml
[logs]
verbose = "true"
```

## appearance

### favorite_indicator

#### A symbol or string of text to indicate a favorite command

- **Default:** `★`

##### Example

```toml
[appearance]
favorite_indicator = "<fav>"
```

### disable_color

#### If "true", colorized output will be disabled

- **Default:** `true`

##### Example

```toml
[appearance]
disable_color = "true"
```

## internet

### force_disable

#### If "true" internet will be force disabled by default

- **Default:** `false`

##### Example

```toml
[internet]
force_disable = "true"
```

> [!NOTE]  
> This prevents updates, update checking, and other online features.

### sample_dns

#### The default is set to Cloudflare. Used to check internet connectivity

- **Default:** `1.1.1.1:53`

##### Example

```toml
[internet]
sample_dns = "8.8.8.8:53"
```

## updates

### zig_version

#### The version that is downloaded when building cmdcreate from source when using `sudo cmdcreate update`

- **Default:** `0.16.0`

##### Example

```toml
[update]
zig_version = "0.15.0"
```
