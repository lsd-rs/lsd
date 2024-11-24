<div align="center">

<p>
  <sup>
    <a href="https://github.com/sponsors/zwpaper">LSD is supported by the community.</a>
  </sup>
</p>
<sup>Special thanks to:</sup>
<br>
<br>
<a href="https://www.warp.dev/?utm_source=github&utm_medium=referral&utm_campaign=lsd_20231001">
<div>
  <picture>
    <img alt="Warp" width="300" src="https://github.com/user-attachments/assets/2bda420d-4211-4900-a37e-e3c7056d799c">
  </picture>
</div>
  <b>Warp, the intelligent terminal</b>
  <div>
    <sup>Available for MacOS and Linux<br>
Visit warp.dev to learn more
    </sup>
  </div>
</a>
<hr>
</div>

**IMPORTANT**: This is the development documents,
please check the docs in [Tags](https://github.com/lsd-rs/lsd/tags) if you installed from the released ones.

The current newest release is: [v1.1.5](https://github.com/lsd-rs/lsd/tree/v1.1.5)

---

# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://raw.githubusercontent.com/lsd-rs/lsd/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)
[![build](https://github.com/lsd-rs/lsd/workflows/CICD/badge.svg)](https://github.com/lsd-rs/lsd/actions)
[![codecov](https://codecov.io/gh/lsd-rs/lsd/branch/master/graph/badge.svg)](https://codecov.io/gh/lsd-rs/lsd)
[![versions](https://img.shields.io/repology/repositories/lsd)](https://repology.org/project/lsd/versions)

![image](https://raw.githubusercontent.com/lsd-rs/lsd/assets/screen_lsd.png)

This project is a rewrite of GNU `ls` with lots of added features like colors, icons, tree-view, more formatting options etc.
The project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls) project.


## Installation

<details>
<summary>Packaging status</summary>
<a href="https://repology.org/project/lsd/versions">
    <img src="https://repology.org/badge/vertical-allrepos/lsd.svg?columns=3" alt="Packaging status">
</a>
</details>

### Prerequisites

Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions. Don't forget to setup your terminal in order to use the correct font.

| OS/Distro                       | Command                                                              |
| ------------------------------- | -------------------------------------------------------------------------------|
| Archlinux                       | `pacman -S lsd`                                                                |
| Fedora                          | `dnf install lsd`                                                              |
| Gentoo                          | `sudo emerge sys-apps/lsd`                                                     |
| macOS                           | `brew install lsd` or `sudo port install lsd`                                  |
| NixOS                           | `nix-env -iA nixos.lsd`                                                        |
| FreeBSD                         | `pkg install lsd`                                                              |
| NetBSD or any `pkgsrc` platform | `pkgin install lsd` or `cd /usr/pkgsrc/sysutils/lsd && make install`           |
| OpenBSD                         | `pkg_add lsd`                                                                  |
| Windows                         | `scoop install lsd` or `winget install --id lsd-rs.lsd` or `choco install lsd` |
| Android (via Termux)            | `pkg install lsd`                                                              |
| Debian sid and bookworm         | `apt install lsd`                                                              |
| Ubuntu 23.04 (Lunar Lobster)    | `apt install lsd`                                                              |
| Earlier Ubuntu/Debian versions  | **snap discontinued**, use [From Binaries](#from-binaries)                     |
| Solus                           | `eopkg it lsd`                                                                 |
| Void Linux                      | `sudo xbps-install lsd`                                                        |
| openSUSE                        | `sudo zypper install lsd`                                                      |

### From source

With Rust's package manager cargo, you can install lsd via:

```sh
cargo install lsd
```

If you want to install the latest master branch commit:

```sh
cargo install --git https://github.com/lsd-rs/lsd.git --branch master
```

### From Binaries

The [release page](https://github.com/lsd-rs/lsd/releases) includes precompiled binaries for Linux, macOS and Windows for every release. You can also get the latest binary of `master` branch from the [GitHub action build artifacts](https://github.com/lsd-rs/lsd/actions?query=branch%3Amaster+is%3Asuccess+event%3Apush) (choose the top action and scroll down to the artifacts section).

## Configuration

`lsd` can be configured with a configuration file to set the default options.
Check [Config file content](#config-file-content) for details.

### Config file location

### Non-Windows

On non-Windows systems `lsd` follows the
[XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
convention for the location of the configuration file. A `config.yaml` or `config.yml` file will be searched for in these locations, in order:

- `$HOME/.config/lsd`
- `$XDG_CONFIG_HOME/lsd`

On most systems these are mapped to the same location, which is `~/.config/lsd/config.yaml`.

### Windows

On Windows systems `lsd` searches for `config.yaml` or `config.yml` in the following locations, in order:

- `%USERPROFILE%\.config\lsd`
- `%APPDATA%\lsd`

These are usually something like `C:\Users\username\AppData\Roaming\lsd\config.yaml` and `C:\Users\username\.config\lsd\config.yaml` respectively.

### Custom

You can also provide a configuration file from a non-standard location:
`lsd --config-file [PATH]`

### Config file content

<details open>
<summary>This is an example config file with the default values and some additional remarks.</summary>

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
# Possible values: permission, user, group, context, size, date, name, inode, links, git
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
  # Possible values: default, custom
  # When "custom" is set, lsd will look in the config directory for `colors.yaml`.
  theme: default

# == Date ==
# This specifies the date format for the date column. The freeform format
# accepts a strftime like string.
# When "classic" is set, this is set to "date".
# Possible values: date, locale, relative, '+<date_format>'
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
  separator: " "

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

# == Permission ==
# Specify the format of the permission column
# Possible value: rwx, octal, attributes (windows only), disable
# permission: rwx

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

# == Hyperlink ==
# Attach hyperlink to filenames
# Possible values: always, auto, never
hyperlink: never

# == Symlink arrow ==
# Specifies how the symlink arrow display, chars in both ascii and utf8
symlink-arrow: ⇒

# == Header ==
# Whether to display block headers.
# Possible values: false, true
header: false

# == Literal ==
# Whether to show quotes on filenames.
# Possible values: false, true
literal: false

# == Truncate owner ==
# How to truncate the username and group names for a file if they exceed a certain
# number of characters.
truncate-owner:
  # Number of characters to keep. By default, no truncation is done (empty value).
  after:
  # String to be appended to a name if truncated.
  marker: ""
```

</details>

## Theme

`lsd` can be configured with theme files to set the colors or icons.

### Color Theme

Color theme can be configured in the [configuration file](#configuration)(color.theme).
The valid theme configurations are:

- `default`: the default color scheme shipped in `lsd`
- `custom`: use a custom color scheme defined in `colors.yaml`
- *(deprecated) theme_file_name(yaml): use the theme file to specify colors (without the `yaml` extension)*

When set to `custom`, `lsd` will look for `colors.yaml` in the
XDG Base Directory, e.g. ~/.config/lsd/colors.yaml

When configured with the `theme-file-name` which is a `yaml` file,
`lsd` will look up the theme file in the following way:

- relative name: check the XDG Base Directory, e.g. ~/.config/lsd/themes/<theme-file-name>.yaml
- absolute name: use the file path and name to find theme file

Check [Color Theme file content](#color-theme-file-content) for details.

#### Color Theme file content

Theme file use the [crossterm](https://crates.io/crates/crossterm)
to configure the colors, check [crossterm](https://docs.rs/crossterm/0.20.0/crossterm/style/enum.Color.html)
for supported colors.

Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg

Please notice that color values would ignore the case, both lowercase and UPPERCASE is supported.

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
  octal: 6
  acl: dark_cyan
  context: cyan
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
git-status:
  default: 245
  unmodified: 245
  ignored: 245
  new-in-index: dark_green
  new-in-workdir: dark_green
  typechange: dark_yellow
  deleted: dark_red
  renamed: dark_green
  modified: dark_yellow
  conflicted: dark_red
```

When creating a theme for `lsd`, you can specify any part of the default theme,
and then change its colors, the items missed would fall back to use the default colors.

### Icon Theme

Icon theme can be configured in a fixed location, `$XDG_CONFIG_DIR/lsd/icons.yaml`,
for example, `~/.config/lsd/icons.yaml` on macOS,
please check [Config file location](#config-file-location) to make sure where is `$XDG_CONFIG_DIR`.

As the file name indicated, the icon theme file is a `yaml` file.

Check [Icon Theme file content](#icon-theme-file-content) for details.

#### Icon Theme file content

`lsd` support 3 kinds of icon overrides, by `name`, by `filetype` and by `extension`.
The final set of icons used will be a combination of what is shipped with in `lsd` with overrides from config applied on top of it.
*You can find the default set of icons [here](src/theme/icon.rs).*

Both nerd font glyphs and Unicode emojis can be used for icons. You can find an example of icons customization below.

```yaml
name:
  .trash: 
  .cargo: 
  .emacs.d: 
  a.out: 
extension:
  go: 
  hs: 
  rs: 🦀
filetype:
  dir: 📂
  file: 📄
  pipe: 📩
  socket: 󰆨
  executable: 
  symlink-dir: 
  symlink-file: 
  device-char: 
  device-block: 󰜫
  special: 
```

## External Configurations

### Required

Enable nerd fonts for your terminal, URxvt for example in `.Xresources`:

```sh
URxvt*font:    xft:Hack Nerd Font:style=Regular:size=11
```

### Optional

In order to use lsd when entering the `ls` command, you need to add this to your shell
configuration file (~/.bashrc, ~/.zshrc, etc.):

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

### Uses unknown compression for member 'control.tar.zst' when using deb

Zst compression is supported starting from `Debian 12` and `Ubuntu 21.10`,
Please use the `_xz.deb` released starting from `lsd v1.1.0`.

Please check https://github.com/lsd-rs/lsd/issues/891 for details or manual fixes.

### Custom Color Schemes for Windows
For `lsd` currently, it reads a system environment variable called LS_COLORS. Please look at the marked solution in [this post](https://github.com/orgs/lsd-rs/discussions/958#discussioncomment-7659375), which contains a guide on how to set a color scheme.

### Icons not showing up

For `lsd` to be able to display icons, the font has to include special font glyphs. This might not be the case for most fonts that you download. Thankfully, you can patch most fonts using [NerdFont](https://www.nerdfonts.com/) and add these icons. Or you can just download an already patched version of your favorite font from [NerdFont font download page](https://www.nerdfonts.com/font-downloads).
Here is a guide on how to set up fonts on [macOS](https://github.com/lsd-rs/lsd/issues/199#issuecomment-494218334) and [Android](https://github.com/lsd-rs/lsd/issues/423).

To check if the font you are using is set up correctly, try running the following snippet in a shell and see if that [prints a folder icon](https://github.com/lsd-rs/lsd/issues/510#issuecomment-860000306). If it prints a box, or question mark or something else, then you might have some issues in how you set up the font or how your terminal emulator renders the font.

```sh
echo $'\uf115'
```

### Icons missing or not rendering correctly using PuTTY/KiTTY on Windows

First of all, make sure a patched font is installed and PuTTY/KiTTY is configured to use it, please check [Prerequisites](#prerequisites).

There are problems for PuTTY/KiTTY to show 2 char wide icons, make sure using a 1 char wide font like [Hack Regular Nerd Font Complete Mono Windows Compatible](https://github.com/ryanoasis/nerd-fonts/blob/master/patched-fonts/Hack/Regular/complete/Hack%20Regular%20Nerd%20Font%20Complete%20Mono%20Windows%20Compatible.ttf), check [this issue](https://github.com/lsd-rs/lsd/issues/331) for detail.

### Colors

You can customize filetype colors using `LS_COLORS` and other colors using the theme.

The default colors are:

| User/Group                                                            | Permission                                                                             | File Type (changes based on your terminal colorscheme)                                                  | Date                                                                                 | File Size                                                                   |
| :-------------------------------------------------------------------- | :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| ![#ffffd7](https://via.placeholder.com/15/ffffd7/000000?text=+) User  | ![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) Read                   | ![#0087ff](https://via.placeholder.com/15/0087ff/000000?text=+) Directory                               | ![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) within the last hour | ![#ffffaf](https://via.placeholder.com/15/ffffaf/000000?text=+) Small File  |
| ![#d7d7af](https://via.placeholder.com/15/d7d7af/000000?text=+) Group | ![#d7ff87](https://via.placeholder.com/15/d7ff87/000000?text=+) Write                  | ![#00d700](https://via.placeholder.com/15/00d700/000000?text=+) Executable File                         | ![#00d787](https://via.placeholder.com/15/00d787/000000?text=+) within the last day  | ![#ffaf87](https://via.placeholder.com/15/ffaf87/000000?text=+) Medium File |
|                                                                       | ![#af0000](https://via.placeholder.com/15/af0000/000000?text=+) Execute                | ![#ffffff](https://via.placeholder.com/15/ffffff/000000?text=+) Non-Executable File                     | ![#00af87](https://via.placeholder.com/15/00af87/000000?text=+) older                | ![#d78700](https://via.placeholder.com/15/d78700/000000?text=+) Large File  |
|                                                                       | ![#ff00ff](https://via.placeholder.com/15/ff00ff/000000?text=+) Execute with Stickybit | ![#af0000](https://via.placeholder.com/15/af0000/000000?text=+) Broken Symlink                          |                                                                                      | ![#ffffff](https://via.placeholder.com/15/ffffff/000000?text=+) Non File    |
|                                                                       | ![#d75f87](https://via.placeholder.com/15/d75f87/000000?text=+) No Access              | ![#00d7d7](https://via.placeholder.com/15/00d7d7/000000?text=+) Pipe/Symlink/Blockdevice/Socket/Special |                                                                                      |                                                                             |
|                                                                       |                                                                                        | ![#d78700](https://via.placeholder.com/15/d78700/000000?text=+) CharDevice                              |                                                                                      |                                                                             |

_Checkout [trapd00r/LS_COLORS](https://github.com/trapd00r/LS_COLORS) and [sharkdp/vivid](https://github.com/sharkdp/vivid) for help in theming using `LS_COLORS`._

### First char of folder/file getting trimmed

Workaround for Konsole: ㅤEdit the config file (or [create it](#config-file-location) if it doesn't already exist) and paste the following into it (contains invisible Unicode characters):

```yml
icons:
    separator: " ㅤ"
```

This is a known issue in a few terminal emulators. Try using a different terminal emulator like. [Alacritty](https://github.com/alacritty/alacritty) and [Kitty](https://github.com/kovidgoyal/kitty) are really good alternatives. You might also want to check if your font is responsible for causing this.
To verify this, try running lsd with icons disabled and if it still does not have the first character, then this is an lsd bug:

```sh
lsd --icon never --ignore-config
```

### UTF-8 Chars

`lsd` will try to display the UTF-8 chars in file name, A `U+FFFD REPLACEMENT CHARACTER`(�) is used to represent the invalid UTF-8 chars.

### Icons are showing up strangely

Nerd Fonts is moving the code points of the Material Design Icons in 3.0, so lsd has updated the icons in #830. If your icons look weird, use fonts that have been patched using Nerd Fonts v2.3.0 or later.

See also: <https://github.com/ryanoasis/nerd-fonts/releases/tag/v2.3.3>

## Contributors

Everyone can contribute to this project, improving the code or adding functions. If anyone wants something to be added we will try to do it.

As this is being updated regularly, don't forget to rebase your fork before creating a pull-request.

## Credits

Special thanks to:

- [meain](https://github.com/meain) for all his contributions and reviews
- [danieldulaney](https://github.com/danieldulaney) for the Windows integration
- [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of CI stuff.
- [athityakumar](https://github.com/athityakumar) for the project [colorls](https://github.com/athityakumar/colorls)
- [All the other contributors](https://github.com/lsd-rs/lsd/graphs/contributors)
