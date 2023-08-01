//! This module defines the [TruncateOwner] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;
use crate::app::Cli;

use crate::config_file::Config;

/// The flag showing how to truncate user and group names.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TruncateOwner {
    pub after: Option<usize>,
    pub marker: Option<String>,
}

impl Configurable<Self> for TruncateOwner {
    /// Get a potential `TruncateOwner` value from [Cli].
    ///
    /// If the "header" argument is passed, this returns a `TruncateOwner` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        match (cli.truncate_owner_after, cli.truncate_owner_marker.clone()) {
            (None, None) => None,
            (after, marker) => Some(Self { after, marker }),
        }
    }

    /// Get a potential `TruncateOwner` value from a [Config].
    ///
    /// If the `Config::truncate_owner` has value,
    /// this returns it as the value of the `TruncateOwner`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.truncate_owner.as_ref().map(|c| Self {
            after: c.after,
            marker: c.marker.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::TruncateOwner;

    use crate::app::Cli;
    use crate::config_file::{self, Config};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, TruncateOwner::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_after_some() {
        let argv = ["lsd", "--truncate-owner-after", "1"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(TruncateOwner {
                after: Some(1),
                marker: None,
            }),
            TruncateOwner::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_marker_some() {
        let argv = ["lsd", "--truncate-owner-marker", "…"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(TruncateOwner {
                after: None,
                marker: Some("…".to_string()),
            }),
            TruncateOwner::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, TruncateOwner::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_all_fields_none() {
        let mut c = Config::with_none();
        c.truncate_owner = Some(config_file::TruncateOwner {
            after: None,
            marker: None,
        });
        assert_eq!(
            Some(TruncateOwner {
                after: None,
                marker: None,
            }),
            TruncateOwner::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_all_fields_some() {
        let mut c = Config::with_none();
        c.truncate_owner = Some(config_file::TruncateOwner {
            after: Some(1),
            marker: Some(">".to_string()),
        });
        assert_eq!(
            Some(TruncateOwner {
                after: Some(1),
                marker: Some(">".to_string()),
            }),
            TruncateOwner::from_config(&c)
        );
    }
}
