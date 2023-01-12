//! This module defines the [IgnoreGlobs]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](IgnoreGlobs::configure_from) method.

use crate::config_file::Config;

use clap::{ArgMatches, Error, ErrorKind, ValueSource};
use globset::{Glob, GlobSet, GlobSetBuilder};

/// The struct holding a [GlobSet] and methods to build it.
#[derive(Clone, Debug)]
pub struct IgnoreGlobs(pub GlobSet);

impl IgnoreGlobs {
    /// Returns a value from either [ArgMatches], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_arg_matches](IgnoreGlobs::from_arg_matches)
    /// - [from_config](IgnoreGlobs::from_config)
    /// - [Default::default]
    ///
    /// # Errors
    ///
    /// If either of the [Glob::new] or [GlobSetBuilder.build] methods return an [Err].
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        if let Some(value) = Self::from_arg_matches(matches) {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Ok(Default::default())
    }

    /// Get a potential [IgnoreGlobs] from [ArgMatches].
    ///
    /// If the "ignore-glob" argument has been passed, this returns a [Result] in a [Some] with
    /// either the built [IgnoreGlobs] or an [Error], if any error was encountered while creating the
    /// [IgnoreGlobs]. If the argument has not been passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Result<Self, Error>> {
        if matches.value_source("ignore-glob") != Some(ValueSource::CommandLine) {
            return None;
        }

        let values = matches.get_many::<String>("ignore-glob")?;
        let mut glob_set_builder = GlobSetBuilder::new();

        for value in values {
            match Self::create_glob(value) {
                Ok(glob) => {
                    glob_set_builder.add(glob);
                }
                Err(err) => return Some(Err(err)),
            }
        }

        Some(Self::create_glob_set(&glob_set_builder).map(Self))
    }

    /// Get a potential [IgnoreGlobs] from a [Config].
    ///
    /// If the `Config::ignore-globs` contains an Array of Strings,
    /// each of its values is used to build the [GlobSet]. If the building
    /// succeeds, the [IgnoreGlobs] is returned in the [Result] in a [Some]. If any error is
    /// encountered while building, an [Error] is returned in the Result instead. If the Config does
    /// not contain such a key, this returns [None].
    fn from_config(config: &Config) -> Option<Result<Self, Error>> {
        let globs = config.ignore_globs.as_ref()?;
        let mut glob_set_builder = GlobSetBuilder::new();

        for glob in globs {
            match Self::create_glob(glob) {
                Ok(glob) => {
                    glob_set_builder.add(glob);
                }
                Err(err) => return Some(Err(err)),
            }
        }

        Some(Self::create_glob_set(&glob_set_builder).map(Self))
    }

    /// Create a [Glob] from a provided pattern.
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob(pattern: &str) -> Result<Glob, Error> {
        Glob::new(pattern).map_err(|err| Error::raw(ErrorKind::ValueValidation, err))
    }

    /// Create a [GlobSet] from a provided [GlobSetBuilder].
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob_set(builder: &GlobSetBuilder) -> Result<GlobSet, Error> {
        builder
            .build()
            .map_err(|err| Error::raw(ErrorKind::ValueValidation, err))
    }
}

/// The default value of `IgnoreGlobs` is the empty [GlobSet], returned by [GlobSet::empty()].
impl Default for IgnoreGlobs {
    fn default() -> Self {
        Self(GlobSet::empty())
    }
}

#[cfg(test)]
mod test {
    use super::IgnoreGlobs;

    use crate::app;
    use crate::config_file::Config;

    // The following tests are implemented using match expressions instead of the assert_eq macro,
    // because clap::Error does not implement PartialEq.
    //
    // Further no tests for actually returned GlobSets are implemented, because GlobSet does not
    // even implement PartialEq and thus can not be easily compared.

    #[test]
    fn test_configuration_from_none() {
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(matches!(
            IgnoreGlobs::configure_from(&matches, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = ["lsd", "--ignore-glob", ".git"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(matches!(
            IgnoreGlobs::configure_from(&matches, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        let mut c = Config::with_none();
        c.ignore_globs = Some(vec![".git".into()]);
        assert!(matches!(IgnoreGlobs::configure_from(&matches, &c), Ok(..)));
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert!(matches!(IgnoreGlobs::from_arg_matches(&matches), None));
    }

    #[test]
    fn test_from_config_none() {
        assert!(matches!(
            IgnoreGlobs::from_config(&Config::with_none()),
            None
        ));
    }
}
