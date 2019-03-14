# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-orange.svg)](https://raw.githubusercontent.com/Peltoche/ical-rs/master/LICENSE)
[![Build Status](https://travis-ci.org/Peltoche/lsd.svg?branch=master)](https://travis-ci.org/Peltoche/lsd)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)
[![Snap Status](https://build.snapcraft.io/badge/Peltoche/lsd.svg)](https://build.snapcraft.io/user/Peltoche/lsd)

# Table of Contents

- [Description](#description)
- [Screenshot](#screenshot)
- [Installation](#installation)
- [Configurations](#configurations)
  * [Required](#required)
  * [Optional](#optional)
- [Benchmark](#benchmark)
- [F.A.Q.](#faq)
- [Contributors](#contributors)
- [Credits](#credits)

## Description

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences.  For example it is written in rust and not in ruby which makes it much faster ([see the benchmarks](#benchmark)).

## Screenshot

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

### Prerequisites

Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions. Don't forget to setup your terminal in order to use the correct font.

### Archlinux

```sh
pacman -S lsd
```

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/Peltoche/lsd/releases) and install it via:

```sh
sudo dpkg -i lsd_7.2.0_amd64.deb  # adapt version number and architecture
```

### On Gentoo

Package available on [package.gentoo.org](https://packages.gentoo.org/packages/sys-apps/lsd) (maintainned by Georgy Yakovlev)

### From Snap

```sh
sudo snap install lsd --classic
```

### On macOS

via [Homebrew](https://brew.sh/):

```sh
brew install lsd
```

### From Sources

With Rust's package manager cargo, you can install lsd via:

```sh
cargo install lsd
```

### From Binaries

The [release page](https://github.com/Peltoche/lsd/releases) includes precompiled binaries for Linux and macOS.


## Configurations

### Required

In order to use lsd instead of the default ls you need to add this to you shell
configuration file  (~/.bashrc, ~/.zshrc, etc.) :

  ```sh
  alias ls='lsd'
  ```

### Optional

Some examples of useful aliases. You can add this to you shell configuration
file  (~/.bashrc, ~/.zshrc, etc.) just under the alias above :

  ```sh
  alias l='ls -l'
  alias la='ls -a'
  alias lla='ls -la'
  alias lt='ls --tree'
  ```


## Benchmark

Result from `hyperfine --warmup 10 'lsd -la /etc/*' 'colorls -la /etc/*' 'exa -la /etc/*' --export-markdown out.md`:

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `lsd -la /etc/*` | 9.8 ± 0.7 | 8.6…11.9 |
| `colorls -la /etc/*` | 387.3 ± 4.1 | 379.8…393.6 |
| `exa -la /etc/*` | 15.4 ± 1.8 | 14.0…24.0 |

## F.A.Q.

### Default Colors

In the future the possibility to customize the colors might be implemented.
For now, the default colors are:

| User/Group | Permissions | File Types | Last time Modified | File Size |
|:---|:---|:---|:---|:---|
|![#ffffd7](https://placehold.it/17/ffffd7/000000?text=+) User|![#00d700](https://placehold.it/17/00d700/000000?text=+) Read |![#0087ff](https://placehold.it/17/0087ff/000000?text=+) Directory|![#00d700](https://placehold.it/17/00d700/000000?text=+) within the last hour|![#ffffaf](https://placehold.it/17/ffffaf/000000?text=+) Small File|
|![#d7d7af](https://placehold.it/17/d7d7af/000000?text=+) Group|![#d7ff87](https://placehold.it/17/d7ff87/000000?text=+) Write|![#00d700](https://placehold.it/17/00d700/000000?text=+) Executable File|![#00d787](https://placehold.it/17/00d787/000000?text=+) within the last day|![#ffaf87](https://placehold.it/17/ffaf87/000000?text=+) Medium File|
||![#af0000](https://placehold.it/17/af0000/000000?text=+) Execute|![#d7d700](https://placehold.it/17/d7d700/000000?text=+) Non-Executable File|![#00af87](https://placehold.it/17/00af87/000000?text=+) older|![#d78700](https://placehold.it/17/d78700/000000?text=+) Large File|
||![#ff00ff](https://placehold.it/17/ff00ff/000000?text=+) Execute with Stickybit|![#af0000](https://placehold.it/17/af0000/000000?text=+) Broken Symlink||![#ffffff](https://placehold.it/17/ffffff/000000?text=+) Non File|
||![#d75f87](https://placehold.it/17/d75f87/000000?text=+) No Access|![#00d7d7](https://placehold.it/17/00d7d7/000000?text=+) Pipe/Symlink/Blockdevice/Socket/Special|||
|||![#d78700](https://placehold.it/17/d78700/000000?text=+) CharDevice|||



## Contributors

Everyone can contribute to this project, improving the code or adding functions. If anyone wants something to be added we will try to do it.

As this is being updated regularly, don't forget to rebase your fork before creating a pull-request.

## Credits

Special thanks to [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of stuff.
