//! This module defines the [IconOption]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// A collection of flags on how to use icons.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Icons {
    /// When to use icons.
    pub when: IconOption,
    /// Which icon theme to use.
    pub theme: IconTheme,
    /// String between icon and name.
    pub separator: IconSeparator,
}

impl Icons {
    /// Get an `Icons` struct from [Cli], a [Config] or the [Default] values.
    ///
    /// The [IconOption] and [IconTheme] are configured with their respective [Configurable]
    /// implementation.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let when = IconOption::configure_from(cli, config);
        let theme = IconTheme::configure_from(cli, config);
        let separator = IconSeparator::configure_from(cli, config);
        Self {
            when,
            theme,
            separator,
        }
    }
}

/// The flag showing when to use icons in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum IconOption {
    Always,
    #[default]
    Auto,
    Never,
}

impl IconOption {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "never" => Self::Never,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'icon'"),
        }
    }
}

impl Configurable<Self> for IconOption {
    /// Get a potential `IconOption` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [IconOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Never)
        } else {
            cli.icon.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `IconOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(IconOption::Never).
    /// Otherwise if the `Config::icon::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Never)
        } else {
            config.icons.as_ref().and_then(|icon| icon.when)
        }
    }
}

/// The flag showing which icon theme to use.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconTheme {
    Unicode,
    #[default]
    Fancy,
}

impl IconTheme {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "fancy" => Self::Fancy,
            "unicode" => Self::Unicode,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'icon-theme'"),
        }
    }
}

impl Configurable<Self> for IconTheme {
    /// Get a potential `IconTheme` variant from [Cli].
    ///
    /// If the argument is passed, this returns the variant corresponding to its parameter in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.icon_theme.as_deref().map(Self::from_arg_str)
    }

    /// Get a potential `IconTheme` variant from a [Config].
    ///
    /// If the `Config::icons::theme` has value and is one of "fancy" or "unicode",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.icons.as_ref().and_then(|icon| icon.theme.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IconSeparator(pub String);

impl Configurable<Self> for IconSeparator {
    /// Get a potential `IconSeparator` variant from [Cli].
    ///
    /// If the argument is passed, this returns the variant corresponding to its parameter in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(_cli: &Cli) -> Option<Self> {
        None
    }

    /// Get a potential `IconSeparator` variant from a [Config].
    ///
    /// This returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(icon) = &config.icons {
            if let Some(separator) = icon.separator.clone() {
                return Some(IconSeparator(separator));
            }
        }
        None
    }
}

/// The default value for `IconSeparator` is [" "].
impl Default for IconSeparator {
    fn default() -> Self {
        IconSeparator(" ".to_string())
    }
}

#[cfg(test)]
mod test_icon_option {
    use clap::Parser;

    use super::IconOption;

    use crate::app::Cli;
    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_always() {
        let argv = ["lsd", "--icon", "always"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Always), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_auto() {
        let argv = ["lsd", "--icon", "auto"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Auto), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_never() {
        let argv = ["lsd", "--icon", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--icon", "always", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_icon_when_multi() {
        let argv = ["lsd", "--icon", "always", "--icon", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Always), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Auto),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Auto), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Never),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }
}

#[cfg(test)]
mod test_icon_theme {
    use clap::Parser;

    use super::IconTheme;

    use crate::app::Cli;
    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_fancy() {
        let argv = ["lsd", "--icon-theme", "fancy"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Fancy), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_unicode() {
        let argv = ["lsd", "--icon-theme", "unicode"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_icon_multi() {
        let argv = ["lsd", "--icon-theme", "fancy", "--icon-theme", "unicode"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_fancy() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Fancy),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Fancy), IconTheme::from_config(&c));
    }

    #[test]
    fn test_from_config_unicode() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Unicode),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_config(&c));
    }
}

#[cfg(test)]
mod test_icon_separator {
    use super::IconSeparator;

    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" ".to_string()),
        });
        let expected = Some(IconSeparator(" ".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }

    #[test]
    fn test_from_config_custom() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" |".to_string()),
        });
        let expected = Some(IconSeparator(" |".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }
}
