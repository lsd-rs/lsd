//! This module defines the [SizeFlag]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file size units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SizeFlag {
    /// The variant to show file size with SI unit prefix and a B for bytes.
    #[default]
    Default,
    /// The variant to show file size with only the SI unit prefix.
    Short,
    /// The variant to show file size in bytes.
    Bytes,
}

impl SizeFlag {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "short" => Self::Short,
            "bytes" => Self::Bytes,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'size'"),
        }
    }
}

impl Configurable<Self> for SizeFlag {
    /// Get a potential `SizeFlag` variant from [Cli].
    ///
    /// If any of the "default", "short" or "bytes" arguments is passed, the corresponding
    /// `SizeFlag` variant is returned in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Bytes)
        } else {
            cli.size.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `SizeFlag` variant from a [Config].
    ///
    /// If the `Config::size` has value and is one of "default", "short" or "bytes",
    /// this returns the corresponding `SizeFlag` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Bytes)
        } else {
            config.size
        }
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::SizeFlag;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_default() {
        assert_eq!(SizeFlag::Default, SizeFlag::default());
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SizeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_default() {
        let argv = ["lsd", "--size", "default"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SizeFlag::Default), SizeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_short() {
        let argv = ["lsd", "--size", "short"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_bytes() {
        let argv = ["lsd", "--size", "bytes"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_cli(&cli));
    }

    #[test]
    #[should_panic]
    fn test_from_cli_unknown() {
        let argv = ["lsd", "--size", "unknown"];
        let _ = Cli::try_parse_from(argv).unwrap();
    }
    #[test]
    fn test_from_cli_size_multi() {
        let argv = ["lsd", "--size", "bytes", "--size", "short"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_size_classic() {
        let argv = ["lsd", "--size", "short", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SizeFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Default);
        assert_eq!(Some(SizeFlag::Default), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_short() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Short);
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_bytes() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Bytes);
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
    }
}
