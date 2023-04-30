//! This module defines the [Sorting] options. To set it up from [Cli], a [Config]
//! and its [Default] value, use the [configure_from](Sorting::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// A collection of flags on how to sort the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Sorting {
    pub column: SortColumn,
    pub order: SortOrder,
    pub dir_grouping: DirGrouping,
}

impl Sorting {
    /// Get a `Sorting` struct from [Cli], a [Config] or the [Default] values.
    ///
    /// The [SortColumn], [SortOrder] and [DirGrouping] are configured with their respective
    /// [Configurable] implementation.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let column = SortColumn::configure_from(cli, config);
        let order = SortOrder::configure_from(cli, config);
        let dir_grouping = DirGrouping::configure_from(cli, config);
        Self {
            column,
            order,
            dir_grouping,
        }
    }
}

/// The flag showing which column to use for sorting.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SortColumn {
    None,
    Extension,
    #[default]
    Name,
    Time,
    Size,
    Version,
    GitStatus,
}

impl Configurable<Self> for SortColumn {
    /// Get a potential `SortColumn` variant from [Cli].
    ///
    /// If either the "timesort" or "sizesort" arguments are passed, this returns the corresponding
    /// `SortColumn` variant in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        let sort = cli.sort.as_deref();

        if cli.timesort || sort == Some("time") {
            Some(Self::Time)
        } else if cli.sizesort || sort == Some("size") {
            Some(Self::Size)
        } else if cli.extensionsort || sort == Some("extension") {
            Some(Self::Extension)
        } else if cli.versionsort || sort == Some("version") {
            Some(Self::Version)
        } else if cli.gitsort || sort == Some("git") {
            Some(Self::GitStatus)
        } else if cli.no_sort || sort == Some("none") {
            Some(Self::None)
        } else {
            None
        }
    }

    /// Get a potential `SortColumn` variant from a [Config].
    ///
    /// If the `Config::sorting::column` has value and is one of "time", "size" or "name",
    /// this returns the corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.sorting.as_ref().and_then(|s| s.column)
    }
}

/// The flag showing which sort order to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    Default,
    Reverse,
}

impl Configurable<Self> for SortOrder {
    /// Get a potential `SortOrder` variant from [Cli].
    ///
    /// If the "reverse" argument is passed, this returns [SortOrder::Reverse] in a [Some].
    /// Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.reverse {
            Some(Self::Reverse)
        } else {
            None
        }
    }

    /// Get a potential `SortOrder` variant from a [Config].
    ///
    /// If the `Config::sorting::reverse` has value,
    /// this returns a mapped variant in a [Some].
    /// Otherwise [None] is returned.
    /// A `true` maps to [SortOrder::Reverse] while `false` maps to [SortOrder::Default].
    fn from_config(config: &Config) -> Option<Self> {
        config.sorting.as_ref().and_then(|s| match s.reverse {
            Some(true) => Some(Self::Reverse),
            Some(false) => Some(Self::Default),
            None => None,
        })
    }
}

/// The flag showing where to place directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum DirGrouping {
    #[default]
    None,
    First,
    Last,
}

impl DirGrouping {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "first" => Self::First,
            "last" => Self::Last,
            "none" => Self::None,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'group-dirs'"),
        }
    }
}
impl Configurable<Self> for DirGrouping {
    /// Get a potential `DirGrouping` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [DirGrouping::None] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            return Some(Self::None);
        }

        if cli.group_directories_first {
            return Some(Self::First);
        }

        if let Some(mode) = &cli.group_dirs {
            return Some(Self::from_arg_str(mode));
        }

        None
    }

    /// Get a potential `DirGrouping` variant from a [Config].
    ///
    /// If the `Config::classic` has value and is `true`,
    /// then this returns the the [DirGrouping::None] variant in a [Some].
    /// Otherwise if `Config::sorting::dir-grouping` has value and
    /// is one of "first", "last" or "none", this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::None)
        } else {
            config.sorting.as_ref().and_then(|s| s.dir_grouping)
        }
    }
}

#[cfg(test)]
mod test_sort_column {
    use clap::Parser;

    use super::SortColumn;

    use crate::app::Cli;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_extension() {
        let argv = ["lsd", "--extensionsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_time() {
        let argv = ["lsd", "--timesort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_size() {
        let argv = ["lsd", "--sizesort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Size), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_git() {
        let argv = ["lsd", "--gitsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_version() {
        let argv = ["lsd", "--versionsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Version), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_no_sort() {
        let argv = ["lsd", "--no-sort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::None), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_sort() {
        let argv = ["lsd", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "size"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Size), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "extension"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "version"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Version), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "none"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::None), SortColumn::from_cli(&cli));
    }

    #[cfg(not(feature = "no-git"))]
    #[test]
    fn test_from_arg_cli_sort_git() {
        let argv = ["lsd", "--sort", "git"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_multi_sort() {
        let argv = ["lsd", "--sort", "size", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_multi_sort_use_last() {
        let argv = ["lsd", "--sort", "size", "-t", "-S", "-X", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortColumn::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty_column() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });

        assert_eq!(None, SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_extension() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Extension),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_name() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Name),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Name), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_time() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Time),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Time), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_size() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Size),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Size), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_version() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Version),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Version), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_git_status() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::GitStatus),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_config(&c));
    }
}

#[cfg(test)]
mod test_sort_order {
    use clap::Parser;

    use super::SortOrder;

    use crate::app::Cli;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SortOrder::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_reverse() {
        let argv = ["lsd", "--reverse"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortOrder::Reverse), SortOrder::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortOrder::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_default_config() {
        assert_eq!(
            Some(SortOrder::default()),
            SortOrder::from_config(&Config::builtin())
        );
    }

    #[test]
    fn test_from_config_empty_reverse() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_true() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(true),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Reverse), SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_false() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(false),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Default), SortOrder::from_config(&c));
    }
}

#[cfg(test)]
mod test_dir_grouping {
    use clap::Parser;

    use super::DirGrouping;

    use crate::app::Cli;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    #[should_panic]
    fn test_from_str_bad_value() {
        DirGrouping::from_arg_str("bad value");
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_first() {
        let argv = ["lsd", "--group-dirs", "first"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_last() {
        let argv = ["lsd", "--group-dirs", "last"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_explicit_none() {
        let argv = ["lsd", "--group-dirs", "none"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--group-dirs", "first", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_group_dirs_multi() {
        let argv = ["lsd", "--group-dirs", "first", "--group-dirs", "last"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_group_directories_first() {
        let argv = ["lsd", "--group-directories-first"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, DirGrouping::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_first() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::First),
        });
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_last() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_explicit_empty() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        c.classic = Some(true);
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_config(&c));
    }
}
