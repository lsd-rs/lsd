//! This module defines the [Dereference] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to dereference symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Dereference(pub bool);

impl Configurable<Self> for Dereference {
    /// Get a potential `Dereference` value from [Cli].
    ///
    /// If the "dereference" argument is passed, this returns a `Dereference` with value `true` in
    /// a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.dereference {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Dereference` value from a [Config].
    ///
    /// If the `Config::dereference` has value, this returns its value
    /// as the value of the `Dereference`, in a [Some], Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.dereference.map(Self)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Dereference;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Dereference::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_true() {
        let argv = ["lsd", "--dereference"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Dereference(true)), Dereference::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Dereference::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.dereference = Some(true);
        assert_eq!(Some(Dereference(true)), Dereference::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.dereference = Some(false);
        assert_eq!(Some(Dereference(false)), Dereference::from_config(&c));
    }
}
