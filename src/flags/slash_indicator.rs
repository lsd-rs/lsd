//! This module defines the [SlashIndicator] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to print slash indicator for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct SlashIndicator(pub bool);    

impl Configurable<Self> for SlashIndicator {
    /// Get a potential `SlashIndicator` value from [Cli].
    ///
    /// If the "SlashIndicator" argument is passed, this returns an `SlashIndicator` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.slash_indicator {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `SlashIndicator` value from a [Config].
    ///
    /// If the `Config::slash_indicator` has value,
    /// this returns its value as the value of the `SlashIndicator`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.slash_indicator.as_ref().map(|ind| Self(*ind))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::SlashIndicator;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SlashIndicator::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_true() {
        let argv = ["lsd", "--slash-indicator"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SlashIndicator(true)), SlashIndicator::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SlashIndicator::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.slash_indicator = Some(true);
        assert_eq!(Some(SlashIndicator(true)), SlashIndicator::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.slash_indicator = Some(false);
        assert_eq!(Some(SlashIndicator(false)), SlashIndicator::from_config(&c));
    }
}
