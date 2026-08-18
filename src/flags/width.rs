//! This module defines the [Width] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The width to assume for the terminal when laying out the grid.
///
/// A value of [None] means the width is auto-detected from the terminal, which is the
/// default behavior. Setting an explicit width forces the grid (multi-column) layout even
/// when the output is not a terminal, mirroring `ls -w`.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Width(pub Option<usize>);

impl Configurable<Self> for Width {
    /// Get a potential `Width` value from [Cli].
    ///
    /// If the "width" argument is passed, this returns its value in a [Some]. Otherwise this
    /// returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.width.map(|w| Self(Some(w)))
    }

    /// Get a potential `Width` value from a [Config].
    ///
    /// If the `Config::width` has value, this returns it in a [Some]. Otherwise this returns
    /// [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.width.map(|w| Self(Some(w)))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Width;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Width::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_width() {
        let argv = ["lsd", "--width", "80"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Width(Some(80))), Width::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_short() {
        let argv = ["lsd", "-w", "120"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Width(Some(120))), Width::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Width::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_width() {
        let mut c = Config::with_none();
        c.width = Some(100);
        assert_eq!(Some(Width(Some(100))), Width::from_config(&c));
    }
}
