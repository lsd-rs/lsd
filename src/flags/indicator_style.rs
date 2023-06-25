//! This module defines the [IndicatorStyle] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The option to choose which indicator to print
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum IndicatorStyle {
    None,
    Slash,
    FileType,
    Classify,
}

impl std::convert::From<&str> for IndicatorStyle {
    fn from(input: &str) -> Self {
        match input {
            "none" => Self::None,
            "slash" => Self::Slash,
            "file-type" => Self::FileType,
            _ => Self::Classify,
        }
    }
}

impl std::default::Default for IndicatorStyle {
    fn default() -> Self {
        Self::Classify
    }
}

impl Configurable<Self> for IndicatorStyle {
    /// Get a potential `IndicatorStyle` value from [Cli].
    ///
    /// If the "indicator_style" argument is passed, this returns an `IndicatorStyle` with its value
    /// in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.indicator_style
            .as_ref()
            .map(|indicator_style| Self::from(indicator_style.as_str()))
    }

    /// Get a potential `IndicatorStyle` value from a [Config].
    ///
    /// If the `Config::indicator_style` has value,
    /// this returns its value as the value of the `IndicatorStyle`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config
            .indicator_style
            .clone()
            .map(|v| Self::from(v.as_str()))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::IndicatorStyle;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::{Configurable, Flags};

    #[test]
    fn test_from_cli_empty() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, IndicatorStyle::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd", "--indicator-style=none"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IndicatorStyle::None), IndicatorStyle::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_slash() {
        let argv = ["lsd", "--indicator-style=slash"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IndicatorStyle::Slash), IndicatorStyle::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_p() {
        let argv = ["lsd", "-p"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::default());
        assert_eq!(IndicatorStyle::Slash, flags.unwrap().indicator_style);
    }

    #[test]
    fn test_from_cli_no_p() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::default());
        assert_eq!(IndicatorStyle::Classify, flags.unwrap().indicator_style);
    }

    #[test]
    fn test_from_cli_file_style() {
        let argv = ["lsd", "--file-type"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::default());
        assert_eq!(IndicatorStyle::FileType, flags.unwrap().indicator_style);
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, IndicatorStyle::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_none() {
        let mut c = Config::with_none();
        c.indicator_style = Some("none".to_string());
        assert_eq!(Some(IndicatorStyle::None), IndicatorStyle::from_config(&c));
    }

    #[test]
    fn test_from_config_slash() {
        let mut c = Config::with_none();
        c.indicator_style = Some("slash".to_string());
        assert_eq!(Some(IndicatorStyle::Slash), IndicatorStyle::from_config(&c));
    }
}
