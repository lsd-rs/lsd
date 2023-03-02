//! This module defines the [Display] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
    /// windows only, used to show files with system protected flag
    SystemProtected,
    All,
    AlmostAll,
    DirectoryOnly,
    #[default]
    VisibleOnly,
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [Cli].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.directory_only {
            Some(Self::DirectoryOnly)
        } else if cli.almost_all {
            Some(Self::AlmostAll)
        } else if cli.all {
            Some(Self::All)
        } else if cli.system_protected {
            #[cfg(windows)]
            return Some(Self::SystemProtected);

            #[cfg(not(windows))]
            return Some(Self::All);
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the `Config::display` has value and is one of
    /// "all", "almost-all", "directory-only" or `visible-only`,
    /// this returns the corresponding `Display` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.display
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Display;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Display::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_system_protected() {
        let argv = ["lsd", "--system-protected"];
        let cli = Cli::try_parse_from(argv).unwrap();
        #[cfg(windows)]
        assert_eq!(Some(Display::SystemProtected), Display::from_cli(&cli));

        #[cfg(not(windows))]
        assert_eq!(Some(Display::All), Display::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_all() {
        let argv = ["lsd", "--all"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Display::All), Display::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_almost_all() {
        let argv = ["lsd", "--almost-all"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Display::AlmostAll), Display::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_directory_only() {
        let argv = ["lsd", "--directory-only"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Display::DirectoryOnly), Display::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Display::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::All);
        assert_eq!(Some(Display::All), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_almost_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::AlmostAll);
        assert_eq!(Some(Display::AlmostAll), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_directory_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::DirectoryOnly);
        assert_eq!(Some(Display::DirectoryOnly), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_visible_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::VisibleOnly);
        assert_eq!(Some(Display::VisibleOnly), Display::from_config(&c));
    }
}
