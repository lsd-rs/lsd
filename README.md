<div align="center">
   <sup>Special thanks to:</sup>
   <br>
   <br>
   <a href="https://go.warp.dev/lsd">
      <img alt="Warp sponsorship" width="400" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/refs/heads/main/Github/Sponsor/Warp-Github-LG-02.png">
   </a>

### [Warp, built for coding with multiple AI agents](https://go.warp.dev/lsd)
[Available for MacOS, Linux, & Windows](https://go.warp.dev/lsd)<br>

&nbsp;&nbsp;&nbsp;&nbsp;

   <sup>Maintained with ❤️ + 🤖 by Pochi</sup>
   <br>
   <br>
   <a href="https://app.getpochi.com">
      <img alt="Pochi AI Coding Assistant" width="160" src="https://github.com/TabbyML/pochi/blob/main/packages/vscode/assets/icons/logo128.png?raw=true">
   </a>

### [Pochi is an AI agent designed for software development.](https://app.getpochi.com)
[It operates within your IDE, using a toolkit of commands to write and refactor code autonomously across your entire project.](https://app.getpochi.com)<br>

</div>

---

> [!IMPORTANT]
> This is the documentation for the development version of lsd.
> Please consult the documentation on [the Tags page](https://github.com/lsd-rs/lsd/tags) if you are looking for the documentation of individual releases.
>
> The current newest release is: [v1.2.0](https://github.com/lsd-rs/lsd/tree/v1.2.0)

# LSD (LSDeluxe)
[![license](http://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://raw.githubusercontent.com/lsd-rs/lsd/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)
[![build](https://github.com/lsd-rs/lsd/workflows/CICD/badge.svg)](https://github.com/lsd-rs/lsd/actions)
[![codecov](https://codecov.io/gh/lsd-rs/lsd/branch/master/graph/badge.svg)](https://codecov.io/gh/lsd-rs/lsd)
[![versions](https://img.shields.io/repology/repositories/lsd)](https://repology.org/project/lsd/versions)

![lsd sample output](https://raw.githubusercontent.com/lsd-rs/lsd/assets/screen_lsd.png)

This project is a rewrite of GNU `ls` with lots of added features like colors, icons, tree-view, more formatting options etc.
The project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls) project.

## Installation
### Prerequisites
>[!TIP]
> Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for help with installing Nerd Fonts

1. In order for icons to work you need to have a patched font like [nerd-font](https://www.nerdfonts.com) or [font-awesome](https://fontawesome.com) installed on your machine and your terminal needs to be configured to use the patched font of your choosing.
2. If you intend to install `lsd` from source you need to have a working Rust toolchain (obviously) on your machine.

### Installing with a package manager
<details>
<summary>Packaging status</summary>
<a href="https://repology.org/project/lsd/versions">
    <img src="https://repology.org/badge/vertical-allrepos/lsd.svg?columns=3" alt="Packaging status">
</a>
</details>

Please consult the table below for the installation command associated with your package manager.

| OS/Distro                       | Command                                                                                       |
| ------------------------------- | ----------------------------------------------------------------------------------------------|
| Archlinux                       | `pacman -S lsd`                                                                               |
| Fedora                          | `dnf install lsd`                                                                             |
| Gentoo                          | `sudo emerge sys-apps/lsd`                                                                    |
| macOS                           | `brew install lsd` or `sudo port install lsd`                                                 |
| NixOS                           | `nix-env -iA nixos.lsd`                                                                       |
| FreeBSD                         | `pkg install lsd`                                                                             |
| NetBSD or any `pkgsrc` platform | `pkgin install lsd` or `cd /usr/pkgsrc/sysutils/lsd && make install`                          |
| OpenBSD                         | `pkg_add lsd`                                                                                 |
| Windows                         | `scoop install lsd` or `winget install --id lsd-rs.lsd` or `choco install lsd`                |
| Android (via Termux)            | `pkg install lsd`                                                                             |
| Debian sid and bookworm         | `apt install lsd`                                                                             |
| Ubuntu 23.04 (Lunar Lobster)    | `apt install lsd`                                                                             |
| Earlier Ubuntu/Debian versions  | **snap discontinued**, use the method described [here](#installing-binaries-directly) instead |
| Solus                           | `eopkg it lsd`                                                                                |
| Void Linux                      | `sudo xbps-install lsd`                                                                       |
| openSUSE                        | `sudo zypper install lsd`                                                                     |

### Installing from source
With [Rust's package manager cargo](https://doc.rust-lang.org/stable/cargo/), you can install lsd via:
```sh
cargo install lsd
```

And if you want to install the latest `main` branch commit you can do so via:
```sh
cargo install --git https://github.com/lsd-rs/lsd.git --branch main
```

### Installing binaries directly
The [release page](https://github.com/lsd-rs/lsd/releases) includes precompiled binaries for Linux, macOS, and Windows for every release. You can also get the latest binary of the `main` branch from the [GitHub action build artifacts](https://github.com/lsd-rs/lsd/actions?query=branch%3Amain+is%3Asuccess+event%3Apush) (choose the top action and then scroll down to the artifacts section).

#### Configuring your shell to use lsd instead of ls (optional)
In order to use lsd instead of entering the `ls` command, you need to create an alias for ls in to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc...). The simplest variant of such an alias is:
```sh
alias ls='lsd'
```
The alias above will replace a stock ls command with an lsd command without additional parameters.

Some examples of other useful aliases are:
```sh
alias l='lsd -l'
alias la='lsd -a'
alias lla='lsd -la'
alias lt='lsd --tree'
```

## Customizing lsd (configuration and theming)
> [!TIP]
> In order to make the customization process easier for you we’ve supplied sample files. These files contain the entries for all the defaults that `lsd` comes with after installation. You can find the sample files in the [documentation folder](./doc)`.
>
> We've also supplied a [color reference](./doc/colors.md) where we’ve documented the default colors `lsd` uses in its output. You can also preview there.

In order to tailor `lsd` to your specific needs you can create any of the following three files and make adjustments as you see fit.

1. `config.yaml` → [config sample file here](./doc/samples/config-sample.yaml)
2. `colors.yaml` → [colors sample file here](./doc/samples/colors-sample.yaml)
3. `icons.yaml` → [icons sample file here](./doc/samples/icons-sample.yaml)

Note that it is _not_ required to have all three of the files present in order for your configuration to be applied. For example, if you [only want to customize the icons](#customization-example) then only [`icons.yaml`](./doc/icons-sample.yaml) needs to be present in the [configuration directory](#config-file-locations); [`config.yaml`](./doc/config-sample.yaml), and [`colors.yaml`](./doc/colors-sample.yaml) do not have to be present in order for your icon modifications to be applied.

### Config file locations
> [!TIP]
> You can also instruct `lsd` to look for configuration files in a custom location of your choosing by using the following command: `lsd --config-file [YOUR_CUSTOM_PATH]`. This is particularly useful when testing a configuration changes before commiting to them.

#### Unix (Linux, Mac, etc...)
On non-Windows systems `lsd` follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html), thus `lsd` will look for configuration files any of the following locations:

- `$HOME/.config/lsd`
- `$XDG_CONFIG_HOME/lsd`

On most systems these variables are mapped to the same location, which is usually `~/.config/lsd/`. If `lsd` does not detect the location, or if the location exists but does not contain any of the three configuration files, the default configuration will be used instead.

#### Windows
On Windows systems `lsd` will look for configuration files in the following locations, **in order**:

1. `%USERPROFILE%\.config\lsd`
2. `%APPDATA%\lsd`

These locations are usually something like `C:\Users\username\AppData\Roaming\lsd\`, and `C:\Users\username\.config\lsd\` respectively.

### Quick customization example
For this example let's assume you're already content `lsd`, but there are a few of the default icons that really bug you and you want to change them to something that suits your needs better. All you have to do is create an `icons.yaml` file in the [configuration directory](#config-file-locations) and configure your custom icon there. Here’s how.

There are 3 kinds of icon overrides available in `lsd`:

- `name`
- `filetype`
- `extension`

Both nerd font glyphs and Unicode emojis can be used for icons. The final set of icons that `lsd` will use is a combination of the [default icons](./src/theme/icon.rs) with the custom icons you’ve set in the `icons.yaml` file.

> [!NOTE]
> Aside from the [icon sample file](./doc/icons-sample.yaml), you can also find the default icon set in the source code [here](src/theme/icon.rs).

A short example for each type of the icon overrides is shown below.

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

## F.A.Q and troubleshooting
### How can I enable nerd fonts using xresources?
To enable nerd fonts for your terminal, URxvt for example, in `.Xresources` take a look at the example below:

```sh
URxvt*font:    xft:Hack Nerd Font:style=Regular:size=11
```

### Why am I seeing `Uses unknown compression for member ‘control.tar.zst'` when using deb?
Zst compression is only supported from `Debian 12`, `Ubuntu 21.10`, and upward. Starting from `lsd v1.1.0` please use the `_xz.deb` release instead. See [this issue](https://github.com/lsd-rs/lsd/issues/891) for additional details and manual fixes.

### How can I set custom color schemes for Windows?
In order to display a custom color scheme `lsd` reads a system environment variable called `LS_COLORS`. If your custom color scheme is not working `LS_COLORS` is most likely missing. Please look at [the marked solution in this post](https://github.com/orgs/lsd-rs/discussions/958#discussioncomment-7659375), which contains instructions on how to set a custom color scheme on Windows for guidance.

### Why are icons not showing up
> [!IMPORTANT]
> Always check if the font you are using is correctly set up! Run the following snippet in your terminal emulator and verify that the output [prints a folder icon](https://github.com/lsd-rs/lsd/issues/510#issuecomment-860000306). If it prints a box, or question mark, or something else, then you might have some issues in how you set up the font or how your terminal emulator renders the font.
>
>    ```sh
>    echo $'\uf115'
>    ```

For `lsd` to be able to display icons the font has to include special font glyphs. If icons are not being displayed it could be the case that your current font does not include such glyphs. Thankfully, you can patch most fonts using [NerdFont](https://www.nerdfonts.com/) and add these icons to your current font.

Alternatively, you can also download an already patched version of your favorite font from the [NerdFont font download page](https://www.nerdfonts.com/font-downloads).

Here is a guide on how to set up fonts on [macOS](https://github.com/lsd-rs/lsd/issues/199#issuecomment-494218334), and on [Android](https://github.com/lsd-rs/lsd/issues/423).

### Why are Icons missing or not rendering correctly using PuTTY/KiTTY on Windows?
First of all, make sure a patched font is available on your local machine and that PuTTY/KiTTY is configured to use that font. If you are not certain what this entails please read the [Prerequisites](#prerequisites).

Please note that there are problems for PuTTY/KiTTY displaying 2 character wide icons which may be the case for the font you configured. To ensure only 1 character wide icons are used by your font, please select a font like [Hack Regular Nerd Font Complete Mono Windows Compatible](https://github.com/ryanoasis/nerd-fonts/blob/master/patched-fonts/Hack/Regular/complete/Hack%20Regular%20Nerd%20Font%20Complete%20Mono%20Windows%20Compatible.ttf) (see [this issue](https://github.com/lsd-rs/lsd/issues/331) for further details).

### Why is the first character of a folder/file getting trimmed?
> [!NOTE]
> **Workaround for [Konsole](https://apps.kde.org/konsole/):** edit the config file (or create it if it doesn't already exist) and paste the following configuration directive into it.
>
>    ```yaml
>    # CAREFUL: use copy-paste because this block contains invisible Unicode characters!
>      icons:
>        separator: " ㅤ"
>    ```

This is a known issue in a few terminal emulators. Try using a different terminal emulator like [Alacritty](https://github.com/alacritty/alacritty) or [Kitty](https://github.com/kovidgoyal/kitty) and see if these suit your needs.

You might also want to check if your font is responsible for causing this. To verify this, try running lsd with icons disabled and if the first character is still being trimmed, you’ve discovered a bug in `lsd`. Until the bug is fixed you can use the following command as workaround:

```sh
lsd --icon never --ignore-config
```

### Why are there weird (UTF-8) characters in the output?
`lsd` will always attempt to display the UTF-8 characters in file name, but a `U+FFFD REPLACEMENT CHARACTER`(�) is used to represent the invalid UTF-8 characters. If you are seeing this in your `lsd` output your filename contains an invalid UTF-8 character.

### Why are the icons are showing up strangely?
Nerd Fonts is moving the code points of the Material Design Icons in version 3.0, so starting from #830 `lsd` is using an updated the icon set. If your icons look weird, use fonts that have been patched using [Nerd Fonts v2.3.0](https://github.com/ryanoasis/nerd-fonts/releases/tag/v2.3.3) or later.

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
