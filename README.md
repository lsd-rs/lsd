# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-orange.svg)](https://raw.githubusercontent.com/Peltoche/ical-rs/master/LICENSE)
[![Build Status](https://travis-ci.org/Peltoche/lsd.svg?branch=master)](https://travis-ci.org/Peltoche/lsd)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)

# Table of Contents

1. [Description](#description)
2. [Screenshot](#screenshot)
3. [Installation](#installation)
    1. [Archlinux](#archlinux)
    2. [Other](#other)
4. [Benchmark](#benchmark)
4. [Todo](#todo)

## Description

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences. For example is written is rust and not ruby
which make it really faster ([see the benchmarks](#benchmark)).

## Screenshot

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

1. Install rust
2. Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions.
3. Install the lsd package with `cargo install lsd`

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
- [ ] Handle the tree (`--tree`) output option
- [ ] Handle the json (`--json`) output option
- [ ] Handle Named pipes
- [ ] Handle sockets
- [ ] Handle block devices
- [ ] Handle character devices
