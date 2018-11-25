# LSD

A ls command with a lot of pretty colors.

## Description

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences. For example is written is rust and not ruby
which make it really faster (around 10 times).

## Screenshot

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

### Archlinux (AUR)

```
# With yaourt
yaourt lsd-git

# With yay
yay lsd-git
```

### Other

1. Install rust
2. Install the patched fonts of powerline nerd-font and/or font-awesome. Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions.
       *Note for `rbenv` users - In case of load error when using `lc`, please try the below patch.*
3. Install the lsd package with `cargo install lsd`

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
