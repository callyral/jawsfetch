# jawsfetch

Shark-themed (by default) Linux system information tool written in Rust

## Features

### Keys

* Ascii art

* Distro name

From `/etc/os-release`

* Kernel version

From `/proc/version`

* Package amount

Support for dpkg, pacman and xbps.

* Shell

From `$SHELL` environment variable.

* Uptime

From `/proc/uptime`

* XDG Session

From either `$XDG_CURRENT_DESKTOP` or `$XDG_SESSION_DESKTOP`.

### Configuration

* Ascii File

Located at `$XDG_CONFIG_HOME/jawsfetch/ascii` or `~/.config/jawsfetch/ascii`

Formatted as plain text. Just put whatever ascii art inside it and it should work.

* Order File

Located at `$XDG_CONFIG_HOME/jawsfetch/order` or `~/.config/jawsfetch/order`

Customize the order that each key appears in.

Example:

```
ascii
session
shell
ascii
```

## Help

```
Usage: jawsfetch [OPTIONS]

Options:
  -c, --color <COLOR>  Select color [default: blue]
  -b, --no-bold        Disable bold colors
  -k, --kernel-long    Long kernel version
  -s, --shell-long     Long shell path
  -h, --help           Print help
```

## To-do

* Package amount: Fedora/dnf support

This required using `dnf list installed | sed '1d' | wc -l`, meaning a second pipe.

* Configuration: A config file for things like color, boldness, etc.

This would mean not having to pass a custom color as an argument. For now you could make a shell alias to something like `jawsfetch -bc red`.

## Crates used

### Clap

This crate is used for argument parsing, as writing my own would've been very difficult.
