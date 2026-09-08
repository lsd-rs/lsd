//! This module defines the [Color]. To set it up from [Cli], a [Config] and its [Default]
//! value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};
use std::env;
use std::fmt;

/// A collection of flags on how to use colors.
#[derive(Clone, Debug, Default)]
pub struct Color {
    /// When to use color.
    pub when: ColorOption,
    pub theme: ThemeOption,
}

impl Color {
    /// Get a `Color` struct from [Cli], a [Config] or the [Default] values.
    ///
    /// The [ColorOption] is configured with their respective [Configurable] implementation.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let when = ColorOption::configure_from(cli, config);
        let theme = ThemeOption::from_config(config);
        Self { when, theme }
    }
}

/// ThemeOption could be one of the following:
/// Custom(*.yaml): use the YAML theme file as theme file
/// if error happened, use the default theme
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum ThemeOption {
    NoColor,
    #[default]
    Default,
    #[allow(dead_code)]
    NoLscolors,
    CustomLegacy(String),
    Custom,
}

impl ThemeOption {
    fn from_config(config: &Config) -> ThemeOption {
        if config.classic == Some(true) {
            ThemeOption::NoColor
        } else {
            config
                .color
                .as_ref()
                .and_then(|c| c.theme.clone())
                .unwrap_or_default()
        }
    }
}

impl<'de> de::Deserialize<'de> for ThemeOption {
    fn deserialize<D>(deserializer: D) -> Result<ThemeOption, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeOptionVisitor;

        impl Visitor<'_> for ThemeOptionVisitor {
            type Value = ThemeOption;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`default` or <theme-file-path>")
            }

            fn visit_str<E>(self, value: &str) -> Result<ThemeOption, E>
            where
                E: de::Error,
            {
                match value {
                    "default" => Ok(ThemeOption::Default),
                    "custom" => Ok(ThemeOption::Custom),
                    str => Ok(ThemeOption::CustomLegacy(str.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ThemeOptionVisitor)
    }
}

/// The flag showing when to use colors in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ColorOption {
    Always,
    #[default]
    Auto,
    Never,
}

impl ColorOption {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "never" => Self::Never,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'color'"),
        }
    }
}

impl Configurable<Self> for ColorOption {
    fn configure_from(cli: &Cli, config: &Config) -> Self {
        let configured = Self::from_config(config);
        // The built-in config uses auto, which must still honor NO_COLOR.
        Self::from_cli(cli)
            .or(configured.filter(|when| *when != Self::Auto))
            .or_else(Self::from_environment)
            .or(configured)
            .unwrap_or_default()
    }

    /// Get a potential `ColorOption` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [ColorOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Never)
        } else {
            cli.color.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `ColorOption` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(ColorOption::Never),
    /// Otherwise if the `Config::color::when` has value and is one of "always", "auto" or "never"
    /// this returns its corresponding variant in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Never)
        } else {
            config.color.as_ref().and_then(|c| c.when)
        }
    }

    fn from_environment() -> Option<Self> {
        env::var_os("NO_COLOR")
            .filter(|value| !value.is_empty())
            .map(|_| Self::Never)
    }
}

#[cfg(test)]
mod test_color_option {
    use clap::Parser;

    use super::ColorOption;

    use crate::app::Cli;
    use crate::config_file::{self, Config};
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_always() {
        let argv = ["lsd", "--color", "always"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(ColorOption::Always), ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_auto() {
        let argv = ["lsd", "--color", "auto"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_never() {
        let argv = ["lsd", "--color", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(ColorOption::Never), ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_env_no_color() {
        for (value, expected) in [
            (None, None),
            (Some(""), None),
            (Some("true"), Some(ColorOption::Never)),
            (Some("false"), Some(ColorOption::Never)),
            (Some("0"), Some(ColorOption::Never)),
            (Some(" "), Some(ColorOption::Never)),
        ] {
            temp_env::with_var("NO_COLOR", value, || {
                assert_eq!(expected, ColorOption::from_environment(), "{value:?}");
            });
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_from_env_no_color_non_unicode() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        temp_env::with_var("NO_COLOR", Some(OsString::from_vec(vec![0xff])), || {
            assert_eq!(Some(ColorOption::Never), ColorOption::from_environment());
        });
    }

    #[test]
    fn test_configure_from_no_color_explicit_config() {
        let cli = Cli::try_parse_from(["lsd"]).unwrap();
        for (when, expected) in [
            (Some(ColorOption::Always), ColorOption::Always),
            (Some(ColorOption::Never), ColorOption::Never),
            (Some(ColorOption::Auto), ColorOption::Never),
            (None, ColorOption::Never),
        ] {
            let mut config = Config::with_none();
            config.color = Some(config_file::Color { when, theme: None });
            temp_env::with_var("NO_COLOR", Some("1"), || {
                assert_eq!(
                    expected,
                    ColorOption::configure_from(&cli, &config),
                    "{when:?}"
                );
            });
        }
    }

    #[test]
    fn test_configure_from_no_color_builtin_auto() {
        let config: Config = serde_yaml::from_str(config_file::DEFAULT_CONFIG).unwrap();
        let cli = Cli::try_parse_from(["lsd"]).unwrap();
        for (value, expected) in [
            (None, ColorOption::Auto),
            (Some(""), ColorOption::Auto),
            (Some("1"), ColorOption::Never),
        ] {
            temp_env::with_var("NO_COLOR", value, || {
                assert_eq!(
                    expected,
                    ColorOption::configure_from(&cli, &config),
                    "{value:?}"
                );
            });
        }
    }

    #[test]
    fn test_configure_from_no_color_cli_override() {
        for when in [ColorOption::Always, ColorOption::Never, ColorOption::Auto] {
            let mut config = Config::with_none();
            config.color = Some(config_file::Color {
                when: Some(when),
                theme: None,
            });
            for (args, expected) in [
                (vec!["lsd", "--color", "always"], ColorOption::Always),
                (vec!["lsd", "--color", "never"], ColorOption::Never),
                (vec!["lsd", "--color", "auto"], ColorOption::Auto),
                (vec!["lsd", "--classic"], ColorOption::Never),
            ] {
                let cli = Cli::try_parse_from(&args).unwrap();
                temp_env::with_var("NO_COLOR", Some("1"), || {
                    assert_eq!(
                        expected,
                        ColorOption::configure_from(&cli, &config),
                        "{when:?}, {args:?}"
                    );
                });
            }
        }
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--color", "always", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(ColorOption::Never), ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_color_multiple() {
        let argv = ["lsd", "--color", "always", "--color", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(ColorOption::Never), ColorOption::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, ColorOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });

        assert_eq!(Some(ColorOption::Always), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Auto),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Never),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });
        c.classic = Some(true);
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }
}

#[cfg(test)]
mod test_theme_option {
    use super::ThemeOption;
    use crate::config_file::{self, Config};

    #[test]
    fn test_from_config_none_default() {
        assert_eq!(
            ThemeOption::Default,
            ThemeOption::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });

        assert_eq!(ThemeOption::Default, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_no_color() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoColor),
        });
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_no_lscolor() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoLscolors),
        });
        assert_eq!(ThemeOption::NoLscolors, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_bad_file_flag() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::CustomLegacy("not-existed".to_string())),
        });
        assert_eq!(
            ThemeOption::CustomLegacy("not-existed".to_string()),
            ThemeOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });
        c.classic = Some(true);
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(&c));
    }
}
