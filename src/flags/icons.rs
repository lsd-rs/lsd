//! This module defines the [IconOption]. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;
use crate::print_error;

use clap::ArgMatches;

/// A collection of flags on how to use icons.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Icons {
    /// When to use icons.
    pub when: IconOption,
    /// Which icon theme to use.
    pub theme: IconTheme,
}

impl Icons {
    /// Get an `Icons` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [IconOption] and [IconTheme] are configured with their respective [Configurable]
    /// implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let when = IconOption::configure_from(matches, config);
        let theme = IconTheme::configure_from(matches, config);
        Self { when, theme }
    }
}

/// The flag showing when to use icons in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum IconOption {
    Always,
    Auto,
    Never,
}

impl IconOption {
    /// Get a value from a str.
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "always" => Some(Self::Always),
            "auto" => Some(Self::Auto),
            "never" => Some(Self::Never),
            _ => {
                print_error!(
                    "icons/when can only be one of auto, always or never, but got {}",
                    &value
                );
                None
            }
        }
    }
}

impl Configurable<Self> for IconOption {
    /// Get a potential `IconOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [IconOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("icon") > 0 {
            match matches.value_of("icon") {
                Some("always") => Some(Self::Always),
                Some("auto") => Some(Self::Auto),
                Some("never") => Some(Self::Never),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `IconOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(IconOption::Never).
    /// Otherwise if the `Config::icon::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = &config.classic {
            return Some(Self::Never);
        }

        if let Some(icon) = &config.icons {
            if let Some(when) = &icon.when {
                Self::from_str(&when)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// The default value for the `IconOption` is [IconOption::Auto].
impl Default for IconOption {
    fn default() -> Self {
        Self::Auto
    }
}

/// The flag showing which icon theme to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum IconTheme {
    Unicode,
    Fancy,
}

impl IconTheme {
    /// Get a value from a [Yaml] string. The [Config] is used to log warnings about wrong values
    /// in a Yaml.
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "fancy" => Some(Self::Fancy),
            "unicode" => Some(Self::Unicode),
            _ => {
                print_error!("icons->theme: {}", &value);
                None
            }
        }
    }
}

impl Configurable<Self> for IconTheme {
    /// Get a potential `IconTheme` variant from [ArgMatches].
    ///
    /// If the argument is passed, this returns the variant corresponding to its parameter in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.occurrences_of("icon-theme") > 0 {
            match matches.value_of("icon-theme") {
                Some("fancy") => Some(Self::Fancy),
                Some("unicode") => Some(Self::Unicode),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `IconTheme` variant from a [Config].
    ///
    /// If the `Config::icons::theme` has value and is one of "fancy" or "unicode",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(icon) = &config.icons {
            if let Some(theme) = &icon.theme {
                Self::from_str(&theme)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// The default value for `IconTheme` is [IconTheme::Fancy].
impl Default for IconTheme {
    fn default() -> Self {
        Self::Fancy
    }
}

#[cfg(test)]
mod test_icon_option {
    use super::IconOption;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, IconOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--icon", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Always),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_autp() {
        let argv = vec!["lsd", "--icon", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Auto),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--icon", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--icon", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, IconOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let yaml_string = "icons:\n  when: always";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconOption::Always),
            IconOption::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_auto() {
        let yaml_string = "icons:\n  when: auto";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconOption::Auto),
            IconOption::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_never() {
        let yaml_string = "icons:\n  when: never";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let yaml_string = "classic: true\nicons:\n  when: always";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_config(&Config::with_none())
        );
    }
}

#[cfg(test)]
mod test_icon_theme {
    use super::IconTheme;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, IconTheme::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_fancy() {
        let argv = vec!["lsd", "--icon-theme", "fancy"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconTheme::Fancy),
            IconTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_unicode() {
        let argv = vec!["lsd", "--icon-theme", "unicode"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconTheme::Unicode),
            IconTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, IconTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_fancy() {
        let yaml_string = "icons:\n  theme: fancy";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconTheme::Fancy),
            IconTheme::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_unicode() {
        let yaml_string = "icons:\n  theme: unicode";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(IconTheme::Unicode),
            IconTheme::from_config(&Config::with_none())
        );
    }
}
