# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://raw.githubusercontent.com/Peltoche/lsd/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)
[![build](https://github.com/Peltoche/lsd/workflows/CICD/badge.svg)](https://github.com/Peltoche/lsd/actions)
[![codecov](https://codecov.io/gh/Peltoche/lsd/branch/master/graph/badge.svg)](https://codecov.io/gh/Peltoche/lsd)
[![versions](https://img.shields.io/repology/repositories/lsd)](https://repology.org/project/lsd/versions)

# Table of Contents

- [Description](#description)
- [Screenshot](#screenshot)
- [Installation](#installation)
- [Configuration](#configuration)
- [External Configurations](#external-configurations)
  - [Required](#required)
  - [Optional](#optional)
- [F.A.Q.](#faq)
- [Contributors](#contributors)
- [Credits](#credits)

## Description

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences.  For example it is written in rust and not in ruby which makes it much faster.

## Screenshot

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

### Prerequisites

Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions. Don't forget to setup your terminal in order to use the correct font.

See [this issue comment](https://github.com/Peltoche/lsd/issues/199#issuecomment-494218334) for detailed instructions on how to configure iTerm2 font settings correctly.

### On Archlinux

```sh
pacman -S lsd
```

### On Fedora

```sh
dnf install lsd
```

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/Peltoche/lsd/releases) and install it via:

```sh
sudo dpkg -i lsd_0.20.1_amd64.deb # adapt version number and architecture
```

### On Gentoo

```sh
sudo emerge sys-apps/lsd
```

(Ebuild maintained by Georgy Yakovlev)

### On macOS

via [Homebrew](https://brew.sh/):

```sh
brew install lsd
```

or [MacPorts](https://www.macports.org/):

```sh
sudo port install lsd
```

### On NixOS/From nix

```sh
nix-env -iA nixos.lsd
```

Or add `lsd` to your `configuration.nix` like so:

```nix
# ...
environment.systemPackages = with pkgs; [
  # other packages ...
  lsd
];
# ...
```

### On FreeBSD

```sh
pkg install lsd
```

### On NetBSD

_... and other platforms using `pkgsrc`_

Using the package manager:

``` sh
pkgin install lsd
```

Building from source:

``` sh
cd /usr/pkgsrc/sysutils/lsd
make install
```

### On Windows

Install with [Scoop](https://scoop.sh):

```powershell
scoop install lsd
```

### On Android (via Termux)

```sh
pkg install lsd
```

[Setup Nerd Fonts in Termux](https://github.com/Peltoche/lsd/issues/423)

### From Sources

With Rust's package manager cargo, you can install lsd via:

```sh
cargo install lsd
```

If you want to install the latest master branch commit:

```sh
cargo install --git https://github.com/Peltoche/lsd.git --branch master
```

### From Binaries

The [release page](https://github.com/Peltoche/lsd/releases) includes precompiled binaries for Linux and macOS.

## Configuration

`lsd` can be configured with a configuration file to set the default options.
Check [Config file content](#config-file-content) for details.

### Config file location

#### Non-Windows

On non-Windows systems `lsd` follows the
[XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
convention for the location of the configuration file. The configuration dir
`lsd` uses is itself named `lsd`. In that directory it looks first for a file
called `config.yaml`.
For most people it should be enough to put their config file at
`~/.config/lsd/config.yaml`.

#### Windows

On Windows systems `lsd` only looks for the `config.yaml` files in one location:
`%APPDATA%\lsd\`

#### Custom

You can also provide a configuration file from a non standard location:
`lsd --config-file [PATH]`

### Config file content

This is an example config file with the default values and some additional
remarks.

```yaml
# == Classic ==
# This is a shorthand to override some of the options to be backwards compatible
# with `ls`. It affects the "color"->"when", "sorting"->"dir-grouping", "date"
# and "icons"->"when" options.
# Possible values: false, true
classic: false

# == Blocks ==
# This specifies the columns and their order when using the long and the tree
# layout.
# Possible values: permission, user, group, size, size_value, date, name, inode
blocks:
  - permission
  - user
  - group
  - size
  - date
  - name

# == Color ==
# This has various color options. (Will be expanded in the future.)
color:
  # When to colorize the output.
  # When "classic" is set, this is set to "never".
  # Possible values: never, auto, always
  when: auto
  # How to colorize the output.
  # When "classic" is set, this is set to "no-color".
  # Possible values: default, <theme-file-name>
  # when specifying <theme-file-name>, lsd will look up theme file
  # XDG Base Directory if relative, e.g. ~/.config/lsd/themes/<theme-file-name>.yaml,
  # The file path if absolute
  theme: default

# == Date ==
# This specifies the date format for the date column. The freeform format
# accepts an strftime like string.
# When "classic" is set, this is set to "date".
# Possible values: date, relative, '+<date_format>'
# `date_format` will be a `strftime` formatted value. e.g. `date: '+%d %b %y %X'` will give you a date like this: 17 Jun 21 20:14:55
date: date

# == Dereference ==
# Whether to dereference symbolic links.
# Possible values: false, true
dereference: false

# == Display ==
# What items to display. Do not specify this for the default behavior.
# Possible values: all, almost-all, directory-only
# display: all

# == Icons ==
icons:
  # When to use icons.
  # When "classic" is set, this is set to "never".
  # Possible values: always, auto, never
  when: auto
  # Which icon theme to use.
  # Possible values: fancy, unicode
  theme: fancy
  # Separator between icon and the name
  # Default to 1 space
  separator: ' '


# == Ignore Globs ==
# A list of globs to ignore when listing.
# ignore-globs:
#   - .git

# == Indicators ==
# Whether to add indicator characters to certain listed files.
# Possible values: false, true
indicators: false

# == Layout ==
# Which layout to use. "oneline" might be a bit confusing here and should be
# called "one-per-line". It might be changed in the future.
# Possible values: grid, tree, oneline
layout: grid

# == Recursion ==
recursion:
  # Whether to enable recursion.
  # Possible values: false, true
  enabled: false
  # How deep the recursion should go. This has to be a positive integer. Leave
  # it unspecified for (virtually) infinite.
  # depth: 3

# == Size ==
# Specifies the format of the size column.
# Possible values: default, short, bytes
size: default

# == Sorting ==
sorting:
  # Specify what to sort by.
  # Possible values: extension, name, time, size, version
  column: name
  # Whether to reverse the sorting.
  # Possible values: false, true
  reverse: false
  # Whether to group directories together and where.
  # When "classic" is set, this is set to "none".
  # Possible values: first, last, none
  dir-grouping: none

# == No Symlink ==
# Whether to omit showing symlink targets
# Possible values: false, true
no-symlink: false

# == Total size ==
# Whether to display the total size of directories.
# Possible values: false, true
total-size: false

# == Symlink arrow ==
# Specifies how the symlink arrow display, chars in both ascii and utf8
symlink-arrow: ⇒
```

## Theme

`lsd` can be configured with a theme file to set the colors.

Theme can be configured in the [configuration file](#configuration)(color.theme),
The valid theme configurations are:
- `default`: the default color scheme shipped in `lsd`
- theme-file-name(yaml): use the theme file to specify colors(without the `yaml` extension)

when configured with the `theme-file-name` which is a `yaml` file,
`lsd` will look up the theme file in the following way:
- relative name: check the themes under XDG Base Directory, e.g. ~/.config/lsd/themes/<theme-file-name>.yaml
- absolute name: use the file path and name to find theme file

Check [Theme file content](#theme-file-content) for details.

### Theme file content

Theme file use the [crossterm](https://crates.io/crates/crossterm)
configure the colors, check [crossterm](https://docs.rs/crossterm/0.20.0/crossterm/style/enum.Color.html)
for the supported colors.

Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg

Please notice that color value would ignore case, both lowercase and UPPERCASE is supported.

This is the default theme scheme shipped with `lsd`.

```yaml
user: 230
group: 187
permission:
  read: dark_green
  write: dark_yellow
  exec: dark_red
  exec-sticky: 5
  no-access: 245
date:
  hour-old: 40
  day-old: 42
  older: 36
size:
  none: 245
  small: 229
  medium: 216
  large: 172
inode:
  valid: 13
  invalid: 245
links:
  valid: 13
  invalid: 245
tree-edge: 245
```


## External Configurations

### Required

Enable nerd fonts for your terminal, URxvt for example:

.Xresources

```
URxvt*font:    xft:Hack Nerd Font:style=Regular:size=11
```

### Optional

In order to use lsd when entering the `ls` command, you need to add this to your shell
configuration file  (~/.bashrc, ~/.zshrc, etc.):

  ```sh
  alias ls='lsd'
  ```

Some further examples of useful aliases:

  ```sh
  alias l='ls -l'
  alias la='ls -a'
  alias lla='ls -la'
  alias lt='ls --tree'
  ```

## F.A.Q

### Default Colors

In the future the possibility to customize the colors might be implemented.
For now, the default colors are:

| User/Group | Permission | File Types | Date | File Size |
|:---|:---|:---|:---|:---|
|![#ffffd7](https://via.placeholder.com/15/ffffd7/000000?text=+) User|![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) Read |![#0087ff](https://via.placeholder.com/15/0087ff/000000?text=+) Directory|![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) within the last hour|![#ffffaf](https://via.placeholder.com/15/ffffaf/000000?text=+) Small File|
|![#d7d7af](https://via.placeholder.com/15/d7d7af/000000?text=+) Group|![#d7ff87](https://via.placeholder.com/15/d7ff87/000000?text=+) Write|![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) Executable File|![#00d787](https://via.placeholder.com/15/00d787/000000?text=+) within the last day|![#ffaf87](https://via.placeholder.com/15/ffaf87/000000?text=+) Medium File|
||![#af0000](https://via.placeholder.com/15/af0000/000000?text=+) Execute|![#d7d700](https://via.placeholder.com/15/d7d700/000000?text=+) Non-Executable File|![#00af87](https://via.placeholder.com/15/00af87/000000?text=+) older|![#d78700](https://via.placeholder.com/15/d78700/000000?text=+) Large File|
||![#ff00ff](https://via.placeholder.com/15/ff00ff/000000?text=+) Execute with Stickybit|![#af0000](https://via.placeholder.com/15/af0000/000000?text=+) Broken Symlink||![#ffffff](https://via.placeholder.com/15/ffffff/000000?text=+) Non File|
||![#d75f87](https://via.placeholder.com/15/d75f87/000000?text=+) No Access|![#00d7d7](https://via.placeholder.com/15/00d7d7/000000?text=+) Pipe/Symlink/Blockdevice/Socket/Special|||
|||![#d78700](https://via.placeholder.com/15/d78700/000000?text=+) CharDevice|||

### UTF-8 Chars

`lsd` will try to display the UTF-8 chars in file name, A `U+FFFD REPLACEMENT CHARACTER`(�) is used to represent the invalid UTF-8 chars.

## Contributors

Everyone can contribute to this project, improving the code or adding functions. If anyone wants something to be added we will try to do it.

As this is being updated regularly, don't forget to rebase your fork before creating a pull-request.

## Credits

Special thanks to:

- [meain](https://github.com/meain) for all his contributions and reviews
- [danieldulaney](https://github.com/danieldulaney) for the Windows integration
- [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of CI stuff.
- [athityakumar](https://github.com/athityakumar) for the project [colorls](https://github.com/athityakumar/colorls)
- [All the other contributors](https://github.com/Peltoche/lsd/graphs/contributors)
