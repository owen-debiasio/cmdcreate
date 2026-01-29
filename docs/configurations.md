# Configuring cmdcreate

You are able to configure cmdcreate in various ways. All are listed here. You can find an example `.toml` configuration [here](https://github.com/owen-debiasio/cmdcreate/blob/main/docs/config_example.toml).

## Categories

## [sys]

### shell

- Your preferred shell for which cmdcreate runs shell commands in
- Default: `sh`

> [!NOTE]
> I recommend using `bash` just for compatability sake.

### spoof_arch

- If "true", the architecture will be spoofed to x86_64 **(Useful when running on a non-x86_64 CPU)**
- Default: `false`

> [!NOTE]
> Architectures of `x86_64` aren't really supported vy me. If you have an issue, please report it.

## [logs]

### time_format

- Formatting of time present in logs
- Default: `%Y-%m-%d %H:%M:%S`

### verbose

- If "true", logs will be printed to output
- Default: `false`

## [appearence]

### favorite_indicator

- A symbol or string of text to indicate a favorite command
- Default: `★`
