//! This module defines the [Header] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to display block headers.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Header(pub bool);

impl Configurable<Self> for Header {
    /// Get a potential `Header` value from [Cli].
    ///
    /// If the "header" argument is passed, this returns a `Header` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.header {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Header` value from a [Config].
    ///
    /// If the `Config::header` has value,
    /// this returns it as the value of the `Header`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.header.map(Self)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Header;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Header::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_true() {
        let argv = ["lsd", "--header"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Header(true)), Header::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Header::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.header = Some(true);
        assert_eq!(Some(Header(true)), Header::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.header = Some(false);
        assert_eq!(Some(Header(false)), Header::from_config(&c));
    }
}
