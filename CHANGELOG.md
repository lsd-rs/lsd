# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
- Add the background color for the files with the setuid permission from [loewenheim](https://github.com/loewenheim)

### Changed
- Do not use the the custom icons for the directoriesa from [cat12079801](https://github.com/cat12079801)

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
- Add the suport to the arm-unknown-linux-gnueabihf platform

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
- Add a custom color for all the spectial files (char / pipe / block)
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
- Add the setuid/setgid/sticky bit support
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
- Change the component alignement by using term_grid


[Unreleased]: https://github.com/Peltoche/lsd/compare/0.14.0...HEAD
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
