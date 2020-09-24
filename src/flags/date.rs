//! This module defines the [DateFlag]. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app;
use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing which kind of time stamps to display.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFlag {
    Date,
    Relative,
    Formatted(String),
}

impl DateFlag {
    /// Get a value from a date format string. The [Config] is used to log warnings about wrong
    /// values in a Yaml.
    fn from_format_string(value: &str, config: &Config) -> Option<Self> {
        match app::validate_time_format(&value) {
            Ok(()) => Some(Self::Formatted(value[1..].to_string())),
            _ => {
                config.print_warning(&format!("Not a valid date format: {}", value));
                None
            }
        }
    }

    /// Get a value from a [Yaml] string. The [Config] is used to log warnings about wrong values
    /// in a Yaml.
    fn from_yaml_string(value: &str, config: &Config) -> Option<Self> {
        match value {
            "date" => Some(Self::Date),
            "relative" => Some(Self::Relative),
            _ if value.starts_with('+') => Self::from_format_string(&value, &config),
            _ => {
                config.print_warning(&format!("Not a valid date value: {}", value));
                None
            }
        }
    }
}

impl Configurable<Self> for DateFlag {
    /// Get a potential `DateFlag` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [DateFlag::Date] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Date)
        } else if matches.occurrences_of("date") > 0 {
            match matches.value_of("date") {
                Some("date") => Some(Self::Date),
                Some("relative") => Some(Self::Relative),
                Some(format) if format.starts_with('+') => {
                    Some(Self::Formatted(format[1..].to_owned()))
                }
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `DateFlag` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [Boolean](Yaml::Boolean) value pointed to by "classic"
    /// and its value is `true`, then this returns the [DateFlag::Date] variant in a [Some].
    /// Otherwise if the Yaml contains a [String](Yaml::String) value pointed to by "date" and it
    /// is one of "date" or "relative", this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            if let Yaml::Boolean(true) = &yaml["classic"] {
                Some(Self::Date)
            } else {
                match &yaml["date"] {
                    Yaml::BadValue => None,
                    Yaml::String(value) => Self::from_yaml_string(&value, &config),
                    _ => {
                        config.print_wrong_type_warning("date", "string");
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `DateFlag` is [DateFlag::Date].
impl Default for DateFlag {
    fn default() -> Self {
        Self::Date
    }
}

#[cfg(test)]
mod test {
    use super::DateFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_date() {
        let argv = vec!["lsd", "--date", "date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_relative() {
        let argv = vec!["lsd", "--date", "relative"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Relative),
            DateFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_format() {
        let argv = vec!["lsd", "--date", "+%F"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    #[should_panic(expected = "invalid format specifier: %J")]
    fn test_from_arg_matches_format_invalid() {
        let argv = vec!["lsd", "--date", "+%J"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        DateFlag::from_arg_matches(&matches);
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--date", "date", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, DateFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, DateFlag::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_date() {
        let yaml_string = "date: date";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DateFlag::Date),
            DateFlag::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_relative() {
        let yaml_string = "date: relative";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DateFlag::Relative),
            DateFlag::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_format() {
        let yaml_string = "date: +%F";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_format_invalid() {
        let yaml_string = "date: +%J";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, DateFlag::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let yaml_string = "classic: true\ndate: relative";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DateFlag::Date),
            DateFlag::from_config(&Config::with_yaml(yaml))
        );
    }
}
