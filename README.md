# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-orange.svg)](https://raw.githubusercontent.com/Peltoche/ical-rs/master/LICENSE)
[![Build Status](https://travis-ci.org/Peltoche/lsd.svg?branch=master)](https://travis-ci.org/Peltoche/lsd)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)
[![Snap Status](https://build.snapcraft.io/badge/Peltoche/lsd.svg)](https://build.snapcraft.io/user/Peltoche/lsd)

# Table of Contents

- [Description](#description)
- [Screenshot](#screenshot)
- [Installation](#installation)
  * [Prerequisites](#prerequisites)
  * [On Ubuntu](#on-ubuntu)
  * [From Snap](#from-snap)
  * [From Sources](#from-sources)
  * [From Binaries](#from-binaries)
- [Configurations](#configurations)
  * [Required](#required)
  * [Optional](#optional)
- [Benchmark](#benchmark)
- [Todo](#todo)
- [Contributors](#contributors)
- [Credits](#credits)

## Description

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences.  For example it is written in rust and not in ruby which makes it really faster ([see the benchmarks](#benchmark)).

## Screenshot

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

### Prerequisites

Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions. Don't forget to setup your terminal in order to use the correct font.

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/Peltoche/lsd/releases) and install it via:

```sh
sudo dpkg -i lsd_7.2.0_amd64.deb  # adapt version number and architecture
```

### From Snap

```sh
sudo snap install lsd

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

Result from `hyperfine --warmup 10 'lsd -la /etc/*' 'colorls -la /etc/*' --export-markdown out.md`:

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `lsd -la /etc/*` | 11.0 ± 0.5 | 9.9…13.0 |
| `colorls -la /etc/*` | 503.3 ± 5.6 | 494.6…513.4 |

## TODO

- [x] Handle the `-l` option (used by default for now)
- [x] Handle the `-a` option
- [x] Add icons before the files names
- [x] Handle all the file types (block/char/pipe/etc)
- [x] Handle the tree (`--tree`) output option
- [ ] Handle the json (`--json`) output option


## Contributors

Everyone can contribute to this project, improving the code or adding functions. If anyone wants something to be added we will try to do it.

As this is being updated regularly, don't forget to rebase your fork before creating a pull-request.

## Credits

Special thanks to [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of stuff.
