# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2023-08-25

### Added
- Add CI to build aarch64 macOS target and skip on test [#878](https://github.com/lsd-rs/lsd/pull/878) from [zwpaper](https://github.com/zwpaper)
- Add complete color theming support for Git [k4yt3x](https://github.com/k4yt3x)
- Add [Git integration](https://github.com/Peltoche/lsd/issues/7) from [hpwxf](https://github.com/hpwxf)
- In keeping with the coreutils change, add quotes and escapes for necessary filenames from [merelymyself](https://github.com/merelymyself)
- Add support for icon theme from [zwpaper](https://github.com/zwpaper)
- Add icon for kt and kts from [LeeWeeder](https://github.com/LeeWeeder)
- Add `--system-protected` to include files with the Windows `system` flag set,
  on other platform the same as `--all` [#752](https://github.com/Peltoche/lsd/issues/752)
- Add many icons from https://github.com/Peltoche/lsd/issues/764 [@TruncatedDinosour](https://ari-web.xyz/gh)
- Add support for localization from [scarf](https://github.com/scarf005)
- Add icons for cjs, cts and mts from [Han Yeong-woo](https://github.com/nix6839)
- Fix obsolete Nerd Font icons from [Han Yeong-woo](https://github.com/nix6839)

### Fixed
- `profile` and `.profile` now share the same icon from [Aaron Lichtman](https://github.com/alichtman)
- Make tox.ini files use the gear/settings icon ([#859](https://github.com/lsd-rs/lsd/pull/859))
- Do not quote filename when piping into another program from [TeamTamoad](https://github.com/TeamTamoad)
- Respect `hidden` flag on Windows [#752](https://github.com/Peltoche/lsd/issues/752)
- Do not show every file are `executable` (green) on Windows
  [#712](https://github.com/Peltoche/lsd/issues/712). Executables will be marked
  based on the file extension: `exe`, `msi`, `bat` and `ps1`.
  [`LS_COLORS`](README.md#Colors) can be used to customize.
- Handle dereference (-L) with broken symlink from [r3dArch](https://github.com/r3dArch)
- Avoid using Clap's deprecated structs and functions [sudame](https://github.com/sudame)
- Icon theme with overrides from config [sudame](https://github.com/sudame)
- Incorrect colorizing with `--size=bytes` [bells307](https://github.com/bells307)

### Changed
- Color theme is now expected to be in `$XDG/lsd/colors.yaml` by default from [peppidesu](https://github.com/peppidesu)
  Legacy behaviour (`themes` folder) is marked as deprecated but is still supported.
  [#749](https://github.com/lsd-rs/lsd/issues/749)
- Version sort option `-v, --versionsort` has been updated to match GNU version sort logic,
  similar to `ls -v` and `sort -V` [#801](https://github.com/lsd-rs/lsd/issues/801) from [juansc](https://github.com/juansc)

## [0.23.1] - 2022-09-13

### Fixed
- Fix tab completion for paths in ZSH from [duhdugg](https://github.com/duhdugg)
- Fix POSIX-compatible exit status from [duhdugg](https://github.com/duhdugg)

## [0.23.0] - 2022-09-05
### Added
- Add icon for Zstandard from [nix6839](https://github.com/nix6839)
### Changed
- Reduce the binary size and improve the performance from [sabify](https://github.com/sabify)
### Fixed
- Fix rendering issues in Windows from [meain](https://github.com/meain)

## [0.22.0] - 2022-06-12
### Added
- Add support for `--header` from [MichaelAug](https://github.com/MichaelAug)
- Add support for `--no-sort` `-U` from [MichaelAug](https://github.com/MichaelAug)
- Add `--group-directories-first` as an alias for `--group-dirs=first` to improve compatibility with `coreutils/ls`
- Add `--permission` flag to choose permission formatting (rwx, octal) from [meain](https://github.com/meain)
- Display MAC contexts and MAC and ACL indicators from [mmatous](https://github.com/mmatous)
- Add `--hyperlink` flag for adding hyperlinks to files from [KSXGitHub](https://github.com/KSXGitHub) and [meain](https://github.com/meain)
- Add icons for HEIC, PEM and TOML from [Nix](https://github.com/nix6839)
### Changed
- Show Docker icon for files with Dockerfile extension [#652](https://github.com/Peltoche/lsd/pull/652) from [TeamTamoad](https://github.com/TeamTamoad)
### Fixed
- Support non-bold bright colors [#248](https://github.com/Peltoche/lsd/issues/248) from [meain](https://github.com/meain)
- Don't automatically dereference symlinks in tree/recursive [#637](https://github.com/Peltoche/lsd/issues/637) from [meain](https://github.com/meain)
- Removed useless error message when attempting to make a hyperlink for a broken symlink from [KodiCraft](https://github.com/KodiCraft)

## [0.21.0] - 2022-01-16
### Added
- Added support for the MISSING / mi= dircolors variable for broken symlink targets.
- Add support for theme from [zwpaper](https://github.com/zwpaper) [#452](https://github.com/Peltoche/lsd/pull/452)
- Update theme to support partial themes [zwpaper](https://github.com/zwpaper) [#591](https://github.com/Peltoche/lsd/pull/591)
- Update minimal rust version to 1.42.0 from [zwpaper](https://github.com/zwpaper) [#534](https://github.com/Peltoche/lsd/issues/534)
- [`NO_COLOR`](https://no-color.org/) environment variable support from [AnInternetTroll](https://github.com/aninternettroll)
### Changed
- Change size to use bytes in classic mode from [meain](https://github.com/meain)
- Show tree edge before name block or first column if no name block from [zwpaper](https://github.com/zwpaper) [#468](https://github.com/Peltoche/lsd/issues/468)
- Added icons for Perl modules (.pm) and test scripts (.t)
- Add `--config-file` flag to read configuration file from a custom location
- Clarify custom date format for `date` field in configuration file in the README.
### Fixed
- Support all `strftime` like formatting [#532](https://github.com/Peltoche/lsd/issues/532)

## [0.20.1] - 2021-03-07
### Fixed
- Fix flaky tree --all test from [meain](https://github.com/meain)

## [0.20.0] - 2021-03-07
### Added
- Add support for changing the string between icon and name from [Finn Hediger](https://github.com/orangefran) [#363](https://github.com/Peltoche/lsd/issues/363)
- Add support for `TIME_STYLE` environment variable from [999eagle](https://github.com/999eagle)
- Add man page from [edneville](https://github.com/edneville)
### Changed
- Not showing `.` and `..` when `--tree` with `--all` from [zwpaper](https://github.com/zwpaper) [#477](https://github.com/Peltoche/lsd/issues/477)
### Fixed
- Fix handling blocks passed without -l in cli from [meain](https://github.com/meain)
- Fix sorting of . and .. when used with folder from [meain](https://github.com/meain)
- Fix arg parsing for flags that allow multiple values from [meain](https://github.com/meain)
- Fix tests involving config file for sorting from [meain](https://github.com/meain)

## [0.19.0] - 2020-12-13
### Added
- Add support for using a config file [kmoschcau](https://github.com/kmoschcau)
- Add support for `--extensionsort` `-X` from [aldhsu](https://github.com/aldhsu)
- Add support for `--versionsort` `-v` from [zwpaper](https://github.com/zwpaper)
- Add nix file icon from [zachcoyle](https://github.com/zachcoyle)
- Add Termux installation instructions from [kcubeterm](https://github.com/kcubeterm)
- Add ttc file icon from [zwpaper](https://github.com/zwpaper)
- Add support for config symlink arrow from [zwpaper](https://github.com/zwpaper) [#409](https://github.com/Peltoche/lsd/issues/409)
- Add julia file icon from [VentGrey](https://github.com/VentGrey)
- Add case-insensitive matching of known filenames and extensions from [poita66](https://github.com/poita66)
- Add Macports installation instructions from [ylluminarious](https://github.com/ylluminarious)
- Implement `--tree -d`, analogous to `tree -d` from [0jdxt](https://github.com/0jdxt) and [Utah Rust](https://github.com/utah-rust)
- Add support for displaying number of hard links from [thealakzam](https://github.com/thealakazam) [#407](https://github.com/Peltoche/lsd/issues/407)

### Changed
- Use last sort flag for sort field from [meain](https://github.com/meain)

### Fixed
- Fix group name show in gid from [zwpaper](https://github.com/zwpaper)
- Fix panic caused by invalid UTF-8 chars in extension from [zwpaper](https://github.com/zwpaper) and [0jdxt](https://github.com/0jdxt)

## [0.18.0] - 2020-08-29
### Added
- Add Support for `--dereference` from [zwpaper](https://github.com/zwpaper)
- Add more icons for wmv,wma and others from [0jdxt](https://github.com/0jdxt)
- Add Windows(Scoop) installation instructions from [turtlebasket](https://github.com/turtlebasket)
- Add opus filetype icon from [nabakolu](https://github.com/nabakolu)
- Add FreeBSD installation instructions from [andoriyu](https://github.com/andoriyu)

### Changed
- Drop snap support from [zwpaper](https://github.com/zwpaper)
- Improve `--ignore-glob` help message from [Pingger](https://github.com/Pingger)
- Separate symlink icons for dirs and files from [0jdxt](https://github.com/0jdxt)

### Fixed
- Fix listing of dir contents for symlinked dirs from [meain](https://github.com/meain)
- Fix grid rendering showing symlink entry files and folders from [meain](https://github.com/meain)
- Fix handling of special chars from [meain](https://github.com/meain)
- Fix regression in `--size short` from [meain](https://github.com/meain)
- Fix handling of relative paths in args from [dvvvvvv](https://github.com/dvvvvvv)
- Fix handling of broken symlinks from [zwpaper](https://github.com/zwpaper)
- Fix icons for lock files and ini files from [WhyNotHugo](https://github.com/WhyNotHugo)

## [0.17.0] - 2020-04-09
### Added
- Add some icons for the special Unix file types from [xSetech](https://github.com/xSetech)
- Add some integration tests from [allenap](https://github.com/allenap)
- Add the flag `--ignore-glob` from [sumitsahrawat](https://github.com/sumitsahrawat)
- Add the elixir icon from [JiNova](https://github.com/JiNova)
- Add the NixOS/nix installation instructions from [06kellyjac](https://github.com/06kellyjac)
- Add the Elm icon from [optikfluffel](https://github.com/optikfluffel)
- Add the date formatting feature from [dvvvvvv](https://github.com/dvvvvvv)
- Add the `inode` block from [zwpaper](https://github.com/zwpaper)
- Add the `--inode` flag from [zwpaper](https://github.com/zwpaper)
- Add the csharp, sln and razor icons from [jpda](https://github.com/jpda)

### Changed
- Move all the CI/CD from travis to github actions from [rivy](https://github.com/rivy)
- Allow the usage of several `--depth` arguments from [abazylewicz](https://github.com/abazylewicz)

### Fixed
- Fix the GID permissions display from [xduugu](https://github.com/xduugu)
- Fix the panic if the pipe is closed before the output is written from [Peltoche](https://github.com/Peltoche)
- Fix the broken softlink display from [zwpaper](https://github.com/zwpaper)

## [0.16.0] - 2019-08-02
### Added
- Add the flag `--blocks` from [meain](https://github.com/meain)
- Add the flag `--no-symlink` from [meain](https://github.com/meain)
- Add the `bytes` option to the `--size` flag from [Philipp-M](https://github.com/Philipp-M)
- Add the flag `--total-size` from [Philipp-M](https://github.com/Philipp-M)
- Add some icons from [JayXon](https://github.com/JayXon)

### Changed
- The flag `--tree` now works with the flag `--long` from [Monkeypac](https://github.com/Monkeypac)

### Fixed
- Fix the padding before the file name

## [0.15.1] - 2019-05-24
### Added
- Add the `Cargo.lock` icon from Holcomb

### Changed
- Update the Genntoo installation instructions from [lovesegfault](https://github.com/lovesegfault)

### Fixed
- Fix the `lsd *.gz` bug from [allenap](https://github.com/allenap)

## [0.15.0] - 2019-05-23
### Added
- Add the maxOS installation instructions from [salOmax](https://github.com/sal0max)
- Add the `--size` flag from [meain](https://github.com/meain)
- Add the current and parent directory from [hemreari](https://github.com/hemreari)
- Add the `--almost-all` flag from [hemreari](https://github.com/hemreari)
- Add the `--group-dirs` flag support for the `--tree` display from [JD557](https://github.com/JD557)
- Add the Windows support from [danieldulaney](https://github.com/danieldulaney)
- Add the `--directory-only` from from [alienap](https://github.com/allenap)
- Add the `--sizesort` flag from [hjanuschka](https://github.com/hjanuschka)

### Changed
- Change the permissions colors to stick with the ANSI colors from [meain](https://github.com/meain)
- Print errors to stderr from [atanunq](https://github.com/atanunq)

### Fixed
- Fix ANSI colors for Windows 10 from [rivy](https://github.com/rivy)
- Fix some snapcraft permission errors from [Peltoche](https://github.com/Peltoche)
- Fix the multi values flag parsing from [meain](https://github.com/meain)
- Fix the `ls -lh ..` bug from [hemreari](https://github.com/hemreari)
- Fix the wildcard for the windows build from [rivy](https://github.com/rivy)

## [0.14.0] - 2019-03-12
### Added
- Add the `-h` option for retro compatibility from [khross](https://github.com/khross)

### Changed
- Update the format for the relative times from [meain](https://github.com/meain)

### Fixed
- Fix the visible width calculation from  [meain](https://github.com/meain)
- Fix a panic une case of invalid modification time

## [0.13.0] - 2019-03-04
### Added
- Add some support for the LS_COLORS env variable from [meain](https://github.com/meain)
- Add the --classic flag from [loewenheim](https://github.com/loewenheim)

### Changed
- Improve the tree display

### Fixed
- Fix the display when not outputting to a tty from [meain](https://github.com/meain)

## [0.12.0] - 2019-01-23
### Added
- Add the --depth parameter for the -R and --tree options from [jorpic](https://github.com/jorpic)
- Add the directory-order flag
- Add a basic unicode support from [loewenheim](https://github.com/loewenheim)
- Add the background color for the files with the setup permission from [loewenheim](https://github.com/loewenheim)

### Changed
- Do not use the the custom icons for the directories from [cat12079801](https://github.com/cat12079801)

### Fixed
- Fix the --icon=never in case of no tiiy
- Fix a panic in case of multiple --icon option set
- Fix some permission display


## [0.11.1] - 2018-12-27
### Fixed
- Fix a panic when a group/user name is not available

## [0.11.0] - 2018-12-20
### Added
- Add the sort by time flag from [boxdot](https://github.com/boxdot)
- Add the reverse sort flag from [boxdot](https://github.com/boxdot)
- Add the support to the arm-unknown-linux-gnueabihf platform

### Fixed
- Fix the width calculation when using the grid output from [kkk669](https://github.com/kkk669)


## [0.10.0] - 2018-12-16
### Added
- Add a CHANGELOG.md
- Add the --date flag with the relative date display from [meain](https://github.com/meain)
- Add new icons

### Changed
- Accept the same flag several times and keep only the latest value

### Fixed
- Fix the snap installation instructions into the README


## [0.9.0] - 2018-12-12
### Added
- Add a custom color for all the special files (char / pipe / block)
- Add some tests on metas
- Add the green colorization for the executable file from [LippyBoy](https://github.com/LippyBoy)
- Add the rust and swift icons from [LippyBoy](https://github.com/LippyBoy)
- Add exa to the README.md benchmarks
- Add the -F (--classify) flag
- Add a template for the Github bug reports

### Changed
- Change the file icon for an empty one
- Change the size display for all the non files node and display '-' instead; from [meain](https://github.com/meain)

### Fixed
- Fix the file name ordering by removing the case sensitivity

### Removed
- Remove the Installation steps from the ToC inside the README
- Remove the TODO section inside the README


## [0.8.0] - 2018-12-08
### Added
- Add the --color flag
- Add a Contributor and Credit section into the README
- Add a Snap / Ubuntu Installation section into the README

### Changed
- Change the display order from left-right to top-down

### Fixed
- Fix the cargo install instructions from [sharkdp](https://github.com/sharkdp)
- Fix the license registration into the Cargo.toml from [Crestwave](https://github.com/Crestwave)
- Fix the license into the snacraft.yml file


## [0.7.12] - 2018-12-07
### Added
- Add the Snap deployment support


## [0.7.0] - 2018-12-06
### Added
- Add the help texts to the cli

### Fixed
- Fix the alias section into the REDME from [domgreen](https://github.com/domgreen)


## [0.6.3] - 2018-12-05
### Added
- Add support for the non tty outputs


## [0.6.2] - 2018-12-05
### Fixed
- Fix the output format for the narrow tty from [yannleretaille](https://github.com/yannleretaille)
- Fix some types


## [0.6.0] - 2018-12-04
### Added
- Add the '--tree' flat


## [0.5.0] - 2018-12-04
### Added
- Add the '--recursive' flat
- Add support for the broken symlinks

### Changed
- Print the symlinks target with the relative path


## [0.4.1] - 2018-12-04
### Added
- Add the '-1' flag


## [0.4.0] - 2018-12-01
### Added
- Add the setup/setgid/sticky bit support
- Add the support for al lthe special files (block / char / pipe / ...)


## [0.3.1] - 2018-11-30
### Fixed
- Fix the file size values


## [0.3.0] - 2018-11-27
### Added
- Add the LSDelux name into the README
- Add the travis CI

### Fixed
- Fix the colors by using the Fixex 256 colors


## [0.2.0] - 2018-11-25
### Added
- Add some badges
- Add the table of content (ToC) inside the README
- Add the '.cfg' icon

### Changed
- Change the component alignment by using term_grid



[v1.0.0]: https://github.com/lsd-rs/lsd/compare/0.23.1...v1.0.0
[0.23.1]: https://github.com/Peltoche/lsd/compare/0.23.0...0.23.1
[0.23.0]: https://github.com/Peltoche/lsd/compare/0.22.0...0.23.0
[0.22.0]: https://github.com/Peltoche/lsd/compare/0.21.0...0.22.0
[0.21.0]: https://github.com/Peltoche/lsd/compare/0.20.1...0.21.0
[0.20.1]: https://github.com/Peltoche/lsd/compare/0.20.0...0.20.1
[0.20.0]: https://github.com/Peltoche/lsd/compare/0.19.0...0.20.0
[0.19.0]: https://github.com/Peltoche/lsd/compare/0.18.0...0.19.0
[0.18.0]: https://github.com/Peltoche/lsd/compare/0.17.0...0.18.0
[0.17.0]: https://github.com/Peltoche/lsd/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/Peltoche/lsd/compare/0.15.1...0.16.0
[0.15.1]: https://github.com/Peltoche/lsd/compare/0.15.0...0.15.1
[0.15.0]: https://github.com/Peltoche/lsd/compare/0.14.1...0.15.0
[0.14.0]: https://github.com/Peltoche/lsd/compare/0.13.1...0.14.0
[0.13.0]: https://github.com/Peltoche/lsd/compare/0.12.1...0.13.0
[0.12.0]: https://github.com/Peltoche/lsd/compare/0.11.1...0.12.0
[0.11.1]: https://github.com/Peltoche/lsd/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/Peltoche/lsd/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/Peltoche/lsd/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/Peltoche/lsd/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/Peltoche/lsd/compare/0.7.12...0.8.0
[0.7.12]: https://github.com/Peltoche/lsd/compare/0.7.0...0.7.12
[0.7.0]: https://github.com/Peltoche/lsd/compare/0.6.3...0.7.0
[0.6.3]: https://github.com/Peltoche/lsd/compare/0.6.2...0.6.3
[0.6.2]: https://github.com/Peltoche/lsd/compare/0.6.0...0.6.2
[0.6.0]: https://github.com/Peltoche/lsd/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/Peltoche/lsd/compare/0.4.0...0.5.0
[0.4.1]: https://github.com/Peltoche/lsd/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/Peltoche/lsd/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/Peltoche/lsd/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/Peltoche/lsd/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/Peltoche/lsd/compare/0.1.0...0.2.0
