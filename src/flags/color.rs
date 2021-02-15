//! This module defines the [Color]. To set it up from [ArgMatches], a [Config] and its [Default]
//! value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;
use crate::print_error;

use clap::ArgMatches;
use serde::Deserialize;

/// A collection of flags on how to use colors.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Color {
    /// When to use color.
    pub when: ColorOption,
}

impl Color {
    /// Get a `Color` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [ColorOption] is configured with their respective [Configurable] implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let when = ColorOption::configure_from(matches, config);
        Self { when }
    }
}

/// The flag showing when to use colors in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorOption {
    Always,
    Auto,
    Never,
}

impl ColorOption {
    /// Get a Color value from a [String].
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "always" => Some(Self::Always),
            "auto" => Some(Self::Auto),
            "never" => Some(Self::Never),
            _ => {
                print_error!(
                    "Config color.when could only be one of auto, always and never, got {}.",
                    &value
                );
                None
            }
        }
    }
}

impl Configurable<Self> for ColorOption {
    /// Get a potential `ColorOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [ColorOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("color") > 0 {
            if let Some(color) = matches.values_of("color")?.last() {
                Self::from_str(&color)
            } else {
                panic!("Bad color args. This should not be reachable!");
            }
        } else {
            None
        }
    }

    /// Get a potential `ColorOption` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(ColorOption::Never),
    /// Otherwise if the `Config::color::when` has value and is one of "always", "auto" or "never"
    /// this returns its corresponding variant in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            return Some(Self::Never);
        }

        if let Some(color) = &config.color {
            Some(color.when)
        } else {
            None
        }
    }
}

/// The default value for `ColorOption` is [ColorOption::Auto].
impl Default for ColorOption {
    fn default() -> Self {
        Self::Auto
    }
}

#[cfg(test)]
mod test_color_option {
    use super::ColorOption;

    use crate::app;
    use crate::config_file::{self, Config};
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, ColorOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--color", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Always),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_auto() {
        let argv = vec!["lsd", "--color", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Auto),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--color", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_color_multiple() {
        let argv = vec!["lsd", "--color", "always", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, ColorOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: ColorOption::Always,
        });

        assert_eq!(Some(ColorOption::Always), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: ColorOption::Auto,
        });
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: ColorOption::Never,
        });
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: ColorOption::Always,
        });
        c.classic = Some(true);
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }
}
