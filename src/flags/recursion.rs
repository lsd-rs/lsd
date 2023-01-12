//! This module defines the [Recursion] options. To set it up from [ArgMatches], a [Config] and its
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
    /// - [Config.recursion.enabled]
    /// - [Default::default]
    fn enabled_from(matches: &ArgMatches, config: &Config) -> bool {
        if let Some(value) = Self::enabled_from_arg_matches(matches) {
            return value;
        }
        if let Some(recursion) = &config.recursion {
            if let Some(enabled) = recursion.enabled {
                return enabled;
            }
        }

        Default::default()
    }

    /// Get a potential "enabled" boolean from [ArgMatches].
    ///
    /// If the "recursive" argument is passed, this returns `true` in a [Some]. Otherwise this
    /// returns [None].
    fn enabled_from_arg_matches(matches: &ArgMatches) -> Option<bool> {
        if matches.get_one("recursive") == Some(&true) {
            Some(true)
        } else {
            None
        }
    }

    /// Get the "depth" integer from [ArgMatches], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [depth_from_arg_matches](Recursion::depth_from_arg_matches)
    /// - [Config.recursion.depth]
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// If both configuration file and Args is error, this will return a Max-Uint value.
    ///
    /// # Errors
    ///
    /// If [depth_from_arg_matches](Recursion::depth_from_arg_matches) returns an [Error], this
    /// returns it.
    fn depth_from(matches: &ArgMatches, config: &Config) -> Result<usize, Error> {
        if let Some(value) = Self::depth_from_arg_matches(matches) {
            return value;
        }

        if let Some(recursion) = &config.recursion {
            if let Some(depth) = recursion.depth {
                return Ok(depth);
            }
        }

        Ok(usize::MAX)
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
        let depth = match matches.get_many::<String>("depth") {
            Some(d) => d.last(),
            None => None,
        };
        if let Some(str) = depth {
            match str.parse::<usize>() {
                Ok(value) => return Some(Ok(value)),
                Err(_) => {
                    return Some(Err(Error::raw(
                        ErrorKind::ValueValidation,
                        "The argument '--depth' requires a valid positive number.",
                    )))
                }
            }
        }
        None
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
    use super::Recursion;

    use crate::app;
    use crate::config_file::{self, Config};

    use clap::ErrorKind;

    #[test]
    fn test_enabled_from_arg_matches_empty() {
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert_eq!(None, Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_arg_matches_true() {
        let argv = ["lsd", "--recursive"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert_eq!(Some(true), Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_empty_matches_and_config() {
        let argv = ["lsd"];
        assert!(!Recursion::enabled_from(
            &app::build().try_get_matches_from(argv).unwrap(),
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
            &app::build().try_get_matches_from(argv).unwrap(),
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
            &app::build().try_get_matches_from(argv).unwrap(),
            &c
        ));
    }

    // The following depth_from_arg_matches tests are implemented using match expressions instead
    // of the assert_eq macro, because clap::Error does not implement PartialEq.

    #[test]
    fn test_depth_from_arg_matches_empty() {
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(matches!(Recursion::depth_from_arg_matches(&matches), None));
    }

    #[test]
    fn test_depth_from_arg_matches_integer() {
        let argv = ["lsd", "--depth", "42"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(
            matches!(Recursion::depth_from_arg_matches(&matches), Some(Ok(value)) if value == 42)
        );
    }

    #[test]
    fn test_depth_from_arg_matches_depth_multi() {
        let argv = ["lsd", "--depth", "4", "--depth", "2"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(
            matches!(Recursion::depth_from_arg_matches(&matches), Some(Ok(value)) if value == 2)
        );
    }

    #[test]
    fn test_depth_from_arg_matches_neg_int() {
        let argv = ["lsd", "--depth", "\\-42"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(
            matches!(Recursion::depth_from_arg_matches(&matches), Some(Err(e)) if e.kind() == ErrorKind::ValueValidation)
        );
    }

    #[test]
    fn test_depth_from_arg_matches_non_int() {
        let argv = ["lsd", "--depth", "foo"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(
            matches!(Recursion::depth_from_arg_matches(&matches), Some(Err(e)) if e.kind() == ErrorKind::ValueValidation)
        );
    }

    #[test]
    fn test_depth_from_config_none_max() {
        let argv = ["lsd"];
        assert_eq!(
            usize::MAX,
            Recursion::depth_from(
                &app::build().try_get_matches_from(argv).unwrap(),
                &Config::with_none()
            )
            .unwrap()
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
            Recursion::depth_from(&app::build().try_get_matches_from(argv).unwrap(), &c).unwrap()
        );
    }
}
