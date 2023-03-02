//! This module defines the [Indicators] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to print file type indicators.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Indicators(pub bool);

impl Configurable<Self> for Indicators {
    /// Get a potential `Indicators` value from [Cli].
    ///
    /// If the "indicators" argument is passed, this returns an `Indicators` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.indicators {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Indicators` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Indicators`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.indicators.as_ref().map(|ind| Self(*ind))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Indicators;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Indicators::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_true() {
        let argv = ["lsd", "--classify"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Indicators(true)), Indicators::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Indicators::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.indicators = Some(true);
        assert_eq!(Some(Indicators(true)), Indicators::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.indicators = Some(false);
        assert_eq!(Some(Indicators(false)), Indicators::from_config(&c));
    }
}
