//! This module defines the [IgnoreGlobs]. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](IgnoreGlobs::configure_from) method.

use crate::app::Cli;
use crate::config_file::Config;

use clap::error::ErrorKind;
use clap::Error;
use globset::{Glob, GlobSet, GlobSetBuilder};

/// The struct holding a [GlobSet] and methods to build it.
#[derive(Clone, Debug)]
pub struct IgnoreGlobs(pub GlobSet);

impl IgnoreGlobs {
    /// Returns a value from either [Cli], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_cli](IgnoreGlobs::from_cli)
    /// - [from_config](IgnoreGlobs::from_config)
    /// - [Default::default]
    ///
    /// # Errors
    ///
    /// If either of the [Glob::new] or [GlobSetBuilder.build] methods return an [Err].
    pub fn configure_from(cli: &Cli, config: &Config) -> Result<Self, Error> {
        if let Some(value) = Self::from_cli(cli) {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Ok(Default::default())
    }

    /// Get a potential [IgnoreGlobs] from [Cli].
    ///
    /// If the "ignore-glob" argument has been passed, this returns a [Result] in a [Some] with
    /// either the built [IgnoreGlobs] or an [Error], if any error was encountered while creating the
    /// [IgnoreGlobs]. If the argument has not been passed, this returns [None].
    fn from_cli(cli: &Cli) -> Option<Result<Self, Error>> {
        if cli.ignore_glob.is_empty() {
            return None;
        }

        let mut glob_set_builder = GlobSetBuilder::new();

        for value in &cli.ignore_glob {
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
    use clap::Parser;

    use super::IgnoreGlobs;

    use crate::app::Cli;
    use crate::config_file::Config;

    // The following tests are implemented using match expressions instead of the assert_eq macro,
    // because clap::Error does not implement PartialEq.
    //
    // Further no tests for actually returned GlobSets are implemented, because GlobSet does not
    // even implement PartialEq and thus can not be easily compared.

    #[test]
    fn test_configuration_from_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            IgnoreGlobs::configure_from(&cli, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = ["lsd", "--ignore-glob", ".git"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            IgnoreGlobs::configure_from(&cli, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.ignore_globs = Some(vec![".git".into()]);
        assert!(matches!(IgnoreGlobs::configure_from(&cli, &c), Ok(..)));
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(IgnoreGlobs::from_cli(&cli).is_none());
    }

    #[test]
    fn test_from_config_none() {
        assert!(IgnoreGlobs::from_config(&Config::with_none()).is_none());
    }
}
