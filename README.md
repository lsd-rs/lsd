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

Result from `hyperfine --warmup 10 'lsd -la /etc/*' 'colorls -la /etc/*' 'exa -la /etc/*' --export-markdown out.md`:

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `lsd -la /etc/*` | 9.8 ± 0.7 | 8.6…11.9 |
| `colorls -la /etc/*` | 387.3 ± 4.1 | 379.8…393.6 |
| `exa -la /etc/*` | 15.4 ± 1.8 | 14.0…24.0 |


## Contributors

Everyone can contribute to this project, improving the code or adding functions. If anyone wants something to be added we will try to do it.

As this is being updated regularly, don't forget to rebase your fork before creating a pull-request.

## Credits

Special thanks to [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of stuff.
