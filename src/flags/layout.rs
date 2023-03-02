//! This module defines the [Layout] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use crate::app::Cli;
use crate::config_file::Config;

use super::Configurable;

use serde::Deserialize;

/// The flag showing which output layout to print.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    #[default]
    Grid,
    Tree,
    OneLine,
}

impl Configurable<Layout> for Layout {
    /// Get a potential `Layout` variant from [Cli].
    ///
    /// If any of the "tree", "long" or "oneline" arguments is passed, this returns the
    /// corresponding `Layout` variant in a [Some]. Otherwise if the number of passed "blocks"
    /// arguments is greater than 1, this also returns the [OneLine](Layout::OneLine) variant.
    /// Finally if neither of them is passed, this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.tree {
            Some(Self::Tree)
        } else if cli.long || cli.oneline || cli.inode || cli.context || cli.blocks.len() > 1
        // TODO: handle this differently
        {
            Some(Self::OneLine)
        } else {
            None
        }
    }

    /// Get a potential Layout variant from a [Config].
    ///
    /// If the `Config::layout` has value and is one of "tree", "oneline" or "grid",
    /// this returns the corresponding `Layout` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.layout
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::Layout;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, Layout::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_tree() {
        let argv = ["lsd", "--tree"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Layout::Tree), Layout::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_oneline() {
        let argv = ["lsd", "--oneline"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_oneline_through_long() {
        let argv = ["lsd", "--long"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_oneline_through_blocks() {
        let argv = ["lsd", "--blocks", "permission,name"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Layout::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_tree() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Tree);
        assert_eq!(Some(Layout::Tree), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_oneline() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::OneLine);
        assert_eq!(Some(Layout::OneLine), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_grid() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Grid);
        assert_eq!(Some(Layout::Grid), Layout::from_config(&c));
    }
}
