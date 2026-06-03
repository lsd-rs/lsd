//! This module defines the [TreeFilter]. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](TreeFilter::configure_from) method.

use crate::app::Cli;
use crate::config_file::Config;

use super::glob_helpers::{create_glob, create_glob_set};
use clap::Error;
use globset::{GlobSet, GlobSetBuilder};

/// the struct holding a [GlobSet] for inclusive tree filtering
#[derive(Clone, Debug)]
pub struct TreeFilter(pub GlobSet);

impl TreeFilter {
    /// Returns a value from either [Cli], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_cli](TreeFilter::from_cli)
    /// - [from_config](TreeFilter::from_config)
    /// - [Default::default]
    ///
    /// # Errors
    ///
    /// If either of the [Glob::new] or [GlobSetBuilder::build] methods return an [Err].
    pub fn configure_from(cli: &Cli, config: &Config) -> Result<Self, Error> {
        if let Some(value) = Self::from_cli(cli) {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Ok(Default::default())
    }

    /// Get a potential [TreeFilter] from [Cli].
    ///
    /// If the "tree-filter" argument has been passed, this returns a [Result] in a [Some] with
    /// either the built [TreeFilter] or an [Error]. If the argument has not been passed, returns [None].
    fn from_cli(cli: &Cli) -> Option<Result<Self, Error>> {
        if cli.tree_filter.is_empty() {
            return None;
        }

        let mut builder = GlobSetBuilder::new();

        for value in &cli.tree_filter {
            match create_glob(value) {
                Ok(glob) => {
                    builder.add(glob);
                }
                Err(err) => return Some(Err(err)),
            }
        }

        Some(create_glob_set(&builder).map(Self))
    }

    /// Get a potential [TreeFilter] from a [Config].
    ///
    /// If `Config::tree_filter` contains an array of strings, each value is used to build
    /// the [GlobSet]. If the build succeeds, returns [TreeFilter] in a [Some]. If the
    /// config does not contain such a key, returns [None].
    fn from_config(config: &Config) -> Option<Result<Self, Error>> {
        let globs = config.tree_filter.as_ref()?;
        let mut builder = GlobSetBuilder::new();

        for glob in globs {
            match create_glob(glob) {
                Ok(glob) => {
                    builder.add(glob);
                }
                Err(err) => return Some(Err(err)),
            }
        }

        Some(create_glob_set(&builder).map(Self))
    }

}

/// the default value of `TreeFilter` is the empty [GlobSet], returned by [GlobSet::empty()].
impl Default for TreeFilter {
    fn default() -> Self {
        Self(GlobSet::empty())
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::TreeFilter;

    use crate::app::Cli;
    use crate::config_file::Config;

    // tests use match instead of assert_eq because clap::Error does not implement PartialEq.
    // no tests for actual GlobSet contents since GlobSet does not implement PartialEq.

    #[test]
    fn test_configuration_from_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            TreeFilter::configure_from(&cli, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = ["lsd", "--tree-filter", "*.rs"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            TreeFilter::configure_from(&cli, &Config::with_none()),
            Ok(..)
        ));
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.tree_filter = Some(vec!["*.rs".into()]);
        assert!(matches!(TreeFilter::configure_from(&cli, &c), Ok(..)));
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(TreeFilter::from_cli(&cli).is_none());
    }

    #[test]
    fn test_from_config_none() {
        assert!(TreeFilter::from_config(&Config::with_none()).is_none());
    }
}
