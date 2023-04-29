//! This module defines the [GitTheme] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;
use serde::de::{self, Deserializer, Visitor};

use crate::app::Cli;
use crate::config_file::Config;

/// Configure how to display git block
///
/// git block is enabled in block flags
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum GitTheme {
    /// Default ASCII theme
    #[default]
    Default,
    /// Custom theme using a YAML config file
    Custom(String),
}

impl Configurable<Self> for GitTheme {
    /// Get a potential `GitTheme` value from [ArgMatches].
    ///
    /// Git theme is not yet configurable from cli
    fn from_cli(_cli: &Cli) -> Option<Self> {
        None
    }

    /// Get a potential `GitTheme` value from a [Config].
    ///
    /// If the `Config::git_theme` has value,
    /// this returns it as the value of the `GitTheme`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.git_theme.clone()
    }
}

impl<'de> de::Deserialize<'de> for GitTheme {
    fn deserialize<D>(deserializer: D) -> Result<GitTheme, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeOptionVisitor;

        impl<'de> Visitor<'de> for ThemeOptionVisitor {
            type Value = GitTheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`default` or <theme-file-path>")
            }

            fn visit_str<E>(self, value: &str) -> Result<GitTheme, E>
            where
                E: de::Error,
            {
                match value {
                    "default" => Ok(GitTheme::Default),
                    str => Ok(GitTheme::Custom(str.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ThemeOptionVisitor)
    }
}

#[cfg(not(feature = "no-git"))]
#[cfg(test)]
mod test {
    use clap::Parser;

    use super::GitTheme;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, GitTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = ["lsd", "--git"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, GitTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, GitTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.git_theme = Some(GitTheme::Default);
        assert_eq!(Some(GitTheme::Default), GitTheme::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.git_theme = Some(GitTheme::Default);
        assert_eq!(Some(GitTheme::Default), GitTheme::from_config(&c));
    }

    #[test]
    fn test_from_default_config() {
        let mut c = Config::with_none();
        c.git_theme = None;
        assert_eq!(None, GitTheme::from_config(&c));
    }
}
