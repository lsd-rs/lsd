//! This module defines the [Git] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to display block gits.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Git(pub bool);

impl Configurable<Self> for Git {
    /// Get a potential `Git` value from [ArgMatches].
    ///
    /// If the "git" argument is passed, this returns a `Git` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if !cfg!(feature = "no-git") && cli.git {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Git` value from a [Config].
    ///
    /// If the `Config::git` has value,
    /// this returns it as the value of the `Git`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.git.map(Self)
    }
}

#[cfg(not(feature = "no-git"))]
#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Git;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Git::from_cli(&cli));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = ["lsd", "--git"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Git(true)), Git::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Git::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.git = Some(true);
        assert_eq!(Some(Git(true)), Git::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.git = Some(false);
        assert_eq!(Some(Git(false)), Git::from_config(&c));
    }
}
