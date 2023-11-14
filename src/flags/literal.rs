//! This module defines the [Literal]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag to set in order to show literal file names without quotes.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Literal(pub bool);

impl Configurable<Self> for Literal {
    /// Get a potential `Literal` value from [Cli].
    ///
    /// If the "literal" argument is passed, this returns a `Literal` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.literal {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Literal` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Literal`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.literal.map(Self)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Literal;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Literal::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_literal() {
        let argv = ["lsd", "--literal"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Literal(true)), Literal::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Literal::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.literal = Some(true);
        assert_eq!(Some(Literal(true)), Literal::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.literal = Some(false);
        assert_eq!(Some(Literal(false)), Literal::from_config(&c));
    }
}
