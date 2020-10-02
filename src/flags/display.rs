//! This module defines the [Display] flag. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Display {
    All,
    AlmostAll,
    AutoAll(u32),
    DirectoryItself,
    DisplayOnlyVisible,
}

impl Display {
    /// Get a value from a [Yaml] string. The [Config] is used to log warnings about wrong values
    /// in a Yaml.
    fn from_yaml_string(value: &str, config: &Config) -> Option<Self> {
        match value {
            "all" => Some(Self::All),
            "almost-all" => Some(Self::AlmostAll),
            "directory-only" => Some(Self::DirectoryItself),
            _ => {
                config.print_invalid_value_warning("display", &value);
                None
            }
        }
    }
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [ArgMatches].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("all") {
            Some(Self::All)
        } else if matches.is_present("almost-all") {
            Some(Self::AlmostAll)
        } else if matches.is_present("auto-all") {
            // the value is pre-validated
            Some(Self::AutoAll(
                matches
                    .value_of("auto-all")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            ))
        } else if matches.is_present("directory-only") {
            Some(Self::DirectoryItself)
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [String](Yaml::String) value pointed to by "display" and
    /// it is either "all", "almost-all" or "directory-only", this returns the corresponding
    /// `Display` variant in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["display"] {
                Yaml::BadValue => None,
                Yaml::String(value) => Self::from_yaml_string(&value, &config),
                _ => {
                    config.print_wrong_type_warning("display", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `Display` is [Display::DisplayOnlyVisible].
impl Default for Display {
    fn default() -> Self {
        Self::DisplayOnlyVisible
    }
}

#[cfg(test)]
mod test {
    use super::Display;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_all() {
        let argv = vec!["lsd", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Display::All), Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_almost_all() {
        let argv = vec!["lsd", "--almost-all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::AlmostAll),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_directory_only() {
        let argv = vec!["lsd", "--directory-only"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::DirectoryItself),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Display::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Display::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_all() {
        let yaml_string = "display: all";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Display::All),
            Display::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_almost_all() {
        let yaml_string = "display: almost-all";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Display::AlmostAll),
            Display::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_directory_only() {
        let yaml_string = "display: directory-only";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Display::DirectoryItself),
            Display::from_config(&Config::with_yaml(yaml))
        );
    }
}
