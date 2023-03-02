//! This module defines the [HyperlinkOption]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing when to use hyperlink in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum HyperlinkOption {
    Always,
    Auto,
    #[default]
    Never,
}

impl HyperlinkOption {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "never" => Self::Never,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'hyperlink'"),
        }
    }
}

impl Configurable<Self> for HyperlinkOption {
    /// Get a potential `HyperlinkOption` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [HyperlinkOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Never)
        } else {
            cli.hyperlink.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `HyperlinkOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(HyperlinkOption::Never).
    /// Otherwise if the `Config::hyperlink::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Never)
        } else {
            config.hyperlink
        }
    }
}

#[cfg(test)]
mod test_hyperlink_option {
    use clap::Parser;

    use super::HyperlinkOption;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, HyperlinkOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_always() {
        let argv = ["lsd", "--hyperlink", "always"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Always),
            HyperlinkOption::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_auto() {
        let argv = ["lsd", "--hyperlink", "auto"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(HyperlinkOption::Auto), HyperlinkOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_never() {
        let argv = ["lsd", "--hyperlink", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--hyperlink", "always", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_hyperlink_when_multi() {
        let argv = ["lsd", "--hyperlink", "always", "--hyperlink", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, HyperlinkOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Always);
        assert_eq!(
            Some(HyperlinkOption::Always),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Auto);
        assert_eq!(
            Some(HyperlinkOption::Auto),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Never);
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.hyperlink = Some(HyperlinkOption::Always);
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_config(&c)
        );
    }
}
