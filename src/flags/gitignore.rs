//! This module defines the [Gitignore]. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Gitignore::configure_from) method.

use crate::app::Cli;
use crate::config_file::Config;

use super::Configurable;
/// The struct holding whether or not to use the gitignore and methods to build it.
/// A value of `true` means to use the gitignore, and filter out gitignored files
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Gitignore(pub bool);

impl Gitignore {
    pub fn use_gitignore(&self) -> bool {
        self.0
    }
}

impl Configurable<Self> for Gitignore {
    /// Returns a value from either [Cli], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_cli](Gitignore::from_cli)
    /// - [from_config](Gitignore::from_config)
    /// - [Default::default]
    fn configure_from(cli: &Cli, config: &Config) -> Self {
        if let Some(value) = Self::from_cli(cli) {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Default::default()
    }

    /// Get a potential [Gitignore] from [Cli].
    ///
    /// If the "gitignore" argument has been passed, this returns a [Gitignore] set to `true` in a [Some]
    /// If the argument has not been passed, this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.gitignore.then_some(Self(true))
    }

    /// Get a potential [Gitignore] from a [Config].
    ///
    /// If the `Config::gitignore` contains an boolean value,
    /// this returns a [Gitignore] set to the value of `Config::gitignore` in a [Some].
    /// Otherwise, returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.gitignore.map(Self)
    }
}

/// The default value of `Gitignore` is false.
impl Default for Gitignore {
    fn default() -> Self {
        Self(false)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Gitignore;

    use super::super::Configurable;

    use crate::app::Cli;
    use crate::config_file::Config;

    #[test]
    fn test_configuration_from_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            Gitignore::configure_from(&cli, &Config::with_none()),
            Gitignore(false)
        ));
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = ["lsd", "--gitignore"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(matches!(
            Gitignore::configure_from(&cli, &Config::with_none()),
            Gitignore(true)
        ));
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.gitignore = Some(true);
        assert!(matches!(
            Gitignore::configure_from(&cli, &c),
            Gitignore(true)
        ));
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert!(Gitignore::from_cli(&cli).is_none());
    }

    #[test]
    fn test_from_config_none() {
        assert!(Gitignore::from_config(&Config::with_none()).is_none());
    }
}
