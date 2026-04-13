//! This module defines the [MaxShown] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](MaxShown::configure_from) method via [Configurable].

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// max number of items to show per directory level in tree layout
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct MaxShown(pub Option<usize>);

impl Configurable<Self> for MaxShown {
    /// Get a potential `MaxShown` value from [Cli].
    ///
    /// If the "max-shown" argument has been passed, this returns a `MaxShown` with `Some(n)`
    /// in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.max_shown.map(|n| Self(Some(n)))
    }

    /// Get a potential `MaxShown` value from a [Config].
    ///
    /// If `Config::max_shown` has a value, this returns it wrapped in `MaxShown(Some(n))`
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.max_shown.map(|n| Self(Some(n)))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::MaxShown;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, MaxShown::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_some() {
        let argv = ["lsd", "--max-shown", "3"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(MaxShown(Some(3))), MaxShown::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, MaxShown::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_some() {
        let mut c = Config::with_none();
        c.max_shown = Some(5);
        assert_eq!(Some(MaxShown(Some(5))), MaxShown::from_config(&c));
    }

    #[test]
    fn test_default() {
        assert_eq!(MaxShown(None), MaxShown::default());
    }

    #[test]
    fn test_configure_from_cli_takes_precedence() {
        let argv = ["lsd", "--max-shown", "7"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.max_shown = Some(2);
        assert_eq!(MaxShown(Some(7)), MaxShown::configure_from(&cli, &c));
    }

    #[test]
    fn test_configure_from_config_fallback() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.max_shown = Some(4);
        assert_eq!(MaxShown(Some(4)), MaxShown::configure_from(&cli, &c));
    }
}
