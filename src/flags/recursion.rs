//! This module defines the [Recursion] options. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use the [configure_from](Recursion::configure_from) method.

use crate::config_file::Config;

use clap::{ArgMatches, Error, ErrorKind};

/// The options relating to recursion.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Recursion {
    /// Whether the recursion into directories is enabled.
    pub enabled: bool,
    /// The depth for how far to recurse into directories.
    pub depth: usize,
}

impl Recursion {
    /// Get the Recursion from either [ArgMatches], a [Config] or the [Default] value.
    ///
    /// The "enabled" value is determined by [enabled_from](Recursion::enabled_from) and the depth
    /// value is determined by [depth_from](Recursion::depth_from).
    ///
    /// # Errors
    ///
    /// If [depth_from](Recursion::depth_from) returns an [Error], this returns it.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        let enabled = Self::enabled_from(matches, config);
        let depth = Self::depth_from(matches, config)?;
        Ok(Self { enabled, depth })
    }

    /// Get the "enabled" boolean from [ArgMatches], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [enabled_from_arg_matches](Recursion::enabled_from_arg_matches)
    /// - [enabled_from_config](Recursion::enabled_from_config)
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// The configuration file's Yaml is read in any case, to be able to check for errors and print
    /// out warnings.
    fn enabled_from(matches: &ArgMatches, config: &Config) -> bool {
        let mut result: bool = Default::default();

        // if config.has_yaml() {
        //     if let Some(value) = Self::enabled_from_config(config) {
        //         result = value;
        //     }
        // }

        if let Some(value) = Self::enabled_from_arg_matches(matches) {
            result = value;
        }

        result
    }

    /// Get a potential "enabled" boolean from [ArgMatches].
    ///
    /// If the "recursive" argument is passed, this returns `true` in a [Some]. Otherwise this
    /// returns [None].
    fn enabled_from_arg_matches(matches: &ArgMatches) -> Option<bool> {
        if matches.is_present("recursive") {
            Some(true)
        } else {
            None
        }
    }

    /// Get a potential "enabled" boolean from a [Config].
    ///
    /// If the Config's [Yaml] contains a [Boolean](Yaml::Boolean) value pointed to by "recursion"
    /// -> "enabled", this returns its value in a [Some]. Otherwise this returns [None].
    fn enabled_from_config(config: &Config) -> Option<bool> {
        // TODO(zhangwei):
        None
    }

    /// Get the "depth" integer from [ArgMatches], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [depth_from_arg_matches](Recursion::depth_from_arg_matches)
    /// - [depth_from_config](Recursion::depth_from_config)
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// The configuration file's Yaml is read in any case, to be able to check for errors and print
    /// out warnings.
    ///
    /// # Errors
    ///
    /// If [depth_from_arg_matches](Recursion::depth_from_arg_matches) returns an [Error], this
    /// returns it.
    fn depth_from(matches: &ArgMatches, config: &Config) -> Result<usize, Error> {
        let mut result: Result<usize, Error> = Ok(usize::max_value());

        // if config.has_yaml() {
        //     if let Some(value) = Self::depth_from_config(config) {
        //         result = Ok(value);
        //     }
        // }

        if let Some(value) = Self::depth_from_arg_matches(matches) {
            result = value;
        }

        result
    }

    /// Get a potential "depth" value from [ArgMatches].
    ///
    /// If the "depth" argument is passed, its parameter is evaluated. If it can be parsed into a
    /// [usize], the [Result] is returned in the [Some]. If it can not be parsed an [Error] is
    /// returned in the [Some]. If the argument has not been passed, a [None] is returned.
    ///
    /// # Errors
    ///
    /// If the parameter to the "depth" argument can not be parsed, this returns an [Error] in a
    /// [Some].
    fn depth_from_arg_matches(matches: &ArgMatches) -> Option<Result<usize, Error>> {
        if let Some(str) = matches.value_of("depth") {
            match str.parse::<usize>() {
                Ok(value) => return Some(Ok(value)),
                Err(_) => {
                    return Some(Err(Error::with_description(
                        "The argument '--depth' requires a valid positive number.",
                        ErrorKind::ValueValidation,
                    )))
                }
            }
        }
        None
    }

    /// Get a potential "depth" value from a [Config].
    ///
    /// If the Config's [Yaml] contains a positive [Integer](Yaml::Integer) value pointed to by
    /// "recursion" -> "depth", this returns its value in a [Some]. Otherwise this returns [None].
    fn depth_from_config(config: &Config) -> Option<usize> {
        // TODO(zhangwei):
        None
    }
}

/// The default values for `Recursion` are the boolean default and [prim@usize::max_value()].
impl Default for Recursion {
    fn default() -> Self {
        Self {
            depth: usize::max_value(),
            enabled: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Recursion;

    use crate::app;
    use crate::config_file::Config;

    use clap::ErrorKind;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_enabled_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_arg_matches_true() {
        let argv = vec!["lsd", "--recursive"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(true), Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_config_none() {
        assert_eq!(None, Recursion::enabled_from_config(&Config::with_none()));
    }

    #[test]
    fn test_enabled_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            None,
            Recursion::enabled_from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_enabled_from_config_true() {
        let yaml_string = "recursion:\n  enabled: true";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(true),
            Recursion::enabled_from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_enabled_from_config_false() {
        let yaml_string = "recursion:\n  enabled: false";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(false),
            Recursion::enabled_from_config(&Config::with_none())
        );
    }

    // The following depth_from_arg_matches tests are implemented using match expressions instead
    // of the assert_eq macro, because clap::Error does not implement PartialEq.

    #[test]
    fn test_depth_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_depth_from_arg_matches_integer() {
        let argv = vec!["lsd", "--depth", "42"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(value) => value == 42,
                    Err(_) => false,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_arg_matches_neg_int() {
        let argv = vec!["lsd", "--depth", "\\-42"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(_) => false,
                    Err(error) => error.kind == ErrorKind::ValueValidation,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_arg_matches_non_int() {
        let argv = vec!["lsd", "--depth", "foo"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(_) => false,
                    Err(error) => error.kind == ErrorKind::ValueValidation,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_config_none() {
        assert_eq!(None, Recursion::depth_from_config(&Config::with_none()));
    }

    #[test]
    fn test_depth_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Recursion::depth_from_config(&Config::with_none()));
    }

    #[test]
    fn test_depth_from_config_pos_integer() {
        let yaml_string = "recursion:\n  depth: 42";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(42),
            Recursion::depth_from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_depth_from_config_neg_integer() {
        let yaml_string = "recursion:\n  depth: -42";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Recursion::depth_from_config(&Config::with_none()));
    }

    #[test]
    fn test_depth_from_config_string() {
        let yaml_string = "recursion:\n  depth: foo";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Recursion::depth_from_config(&Config::with_none()));
    }
}
