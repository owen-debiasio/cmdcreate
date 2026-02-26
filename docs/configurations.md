<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 Owen Debiasio
Licensed under the GNU General Public License v3.0 or later.
--->

# Configuring cmdcreate

You are able to configure cmdcreate in various ways. All are listed here. You
can find an example `.toml` configuration
[here](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/resources/config_example.toml).

## Categories

## sys

### shell

- Your preferred shell for which cmdcreate runs shell commands in
- Default: `sh`

> [!NOTE]  
> I recommend using `bash` just for compatibility’s sake.

## logs

### time_format

- Formatting of time present in logs
- Default: `%Y-%m-%d %H:%M:%S`

### verbose

- If "true", logs will be printed to output
- Default: `false`

## appearance

### favorite_indicator

- A symbol or string of text to indicate a favorite command
- Default: `★`
