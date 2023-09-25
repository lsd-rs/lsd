//! This module defines the [Recursion] options. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Recursion::configure_from) method.

use crate::app::Cli;
use crate::config_file::Config;

/// The options relating to recursion.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Recursion {
    /// Whether the recursion into directories is enabled.
    pub enabled: bool,
    /// The depth for how far to recurse into directories.
    pub depth: usize,
}

impl Recursion {
    /// Get the Recursion from either [Cli], a [Config] or the [Default] value.
    ///
    /// The "enabled" value is determined by [enabled_from](Recursion::enabled_from) and the depth
    /// value is determined by [depth_from](Recursion::depth_from).
    ///
    /// # Errors
    ///
    /// If [depth_from](Recursion::depth_from) returns an [Error], this returns it.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let enabled = Self::enabled_from(cli, config);
        let depth = Self::depth_from(cli, config);
        Self { enabled, depth }
    }

    /// Get the "enabled" boolean from [Cli], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [enabled_from_cli](Recursion::enabled_from_cli)
    /// - [Config.recursion.enabled]
    /// - [Default::default]
    fn enabled_from(cli: &Cli, config: &Config) -> bool {
        if let Some(value) = Self::enabled_from_cli(cli) {
            return value;
        }
        if let Some(recursion) = &config.recursion {
            if let Some(enabled) = recursion.enabled {
                return enabled;
            }
        }

        Default::default()
    }

    /// Get a potential "enabled" boolean from [Cli].
    ///
    /// If the "recursive" argument is passed, this returns `true` in a [Some]. Otherwise this
    /// returns [None].
    fn enabled_from_cli(cli: &Cli) -> Option<bool> {
        if cli.recursive {
            Some(true)
        } else {
            None
        }
    }

    /// Get the "depth" integer from [Cli], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - Cli::depth
    /// - [Config.recursion.depth]
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// If both configuration file and Args is error, this will return a Max-Uint value.
    fn depth_from(cli: &Cli, config: &Config) -> usize {
        if let Some(value) = cli.depth {
            return value;
        }

        use crate::config_file::Recursion;
        if let Some(Recursion {
            depth: Some(value), ..
        }) = &config.recursion
        {
            return *value;
        }

        usize::MAX
    }
}

/// The default values for `Recursion` are the boolean default and [prim@usize::max_value()].
impl Default for Recursion {
    fn default() -> Self {
        Self {
            depth: usize::MAX,
            enabled: false,
        }
    }
}

#[cfg(test)]
mod test {
    use clap::error::ErrorKind;
    use clap::Parser;

    use super::Recursion;

    use crate::app::Cli;
    use crate::config_file::{self, Config};

    #[test]
    fn test_enabled_from_cli_empty() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Recursion::enabled_from_cli(&cli));
    }

    #[test]
    fn test_enabled_from_cli_true() {
        let argv = ["lsd", "--recursive"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(true), Recursion::enabled_from_cli(&cli));
    }

    #[test]
    fn test_enabled_from_empty_matches_and_config() {
        let argv = ["lsd"];
        assert!(!Recursion::enabled_from(
            &Cli::try_parse_from(argv).unwrap(),
            &Config::with_none()
        ));
    }

    #[test]
    fn test_enabled_from_matches_empty_and_config_true() {
        let argv = ["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: Some(true),
            depth: None,
        });
        assert!(Recursion::enabled_from(
            &Cli::try_parse_from(argv).unwrap(),
            &c
        ));
    }

    #[test]
    fn test_enabled_from_matches_empty_and_config_false() {
        let argv = ["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: Some(false),
            depth: None,
        });
        assert!(!Recursion::enabled_from(
            &Cli::try_parse_from(argv).unwrap(),
            &c
        ));
    }

    // The following depth_from_cli tests are implemented using match expressions instead
    // of the assert_eq macro, because clap::Error does not implement PartialEq.

    #[test]
    fn test_depth_from_cli_empty() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(cli.depth.is_none());
    }

    #[test]
    fn test_depth_from_cli_integer() {
        let argv = ["lsd", "--depth", "42"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(cli.depth, Some(42)));
    }

    #[test]
    fn test_depth_from_cli_depth_multi() {
        let argv = ["lsd", "--depth", "4", "--depth", "2"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(cli.depth, Some(2)));
    }

    #[test]
    fn test_depth_from_cli_neg_int() {
        let argv = ["lsd", "--depth", "\\-42"];
        let cli = Cli::try_parse_from(argv);
        assert!(matches!(cli, Err(e) if e.kind() == ErrorKind::ValueValidation));
    }

    #[test]
    fn test_depth_from_cli_non_int() {
        let argv = ["lsd", "--depth", "foo"];
        let cli = Cli::try_parse_from(argv);
        assert!(matches!(cli, Err(e) if e.kind() == ErrorKind::ValueValidation));
    }

    #[test]
    fn test_depth_from_config_none_max() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            usize::MAX,
            Recursion::depth_from(&cli, &Config::with_none())
        );
    }

    #[test]
    fn test_depth_from_config_pos_integer() {
        let argv = ["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: None,
            depth: Some(42),
        });
        assert_eq!(
            42,
            Recursion::depth_from(&Cli::try_parse_from(argv).unwrap(), &c)
        );
    }
}
