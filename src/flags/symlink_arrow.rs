use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing how to display symbolic arrow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymlinkArrow(String);

impl Configurable<Self> for SymlinkArrow {
    /// `SymlinkArrow` can be indirectly configured by [Cli] when the classic option is used.
    ///
    /// If classic is used, returns `->` in a [Some];
    /// otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(SymlinkArrow("->".to_string()))
        } else {
            None
        }
    }

    /// Get a potential `SymlinkArrow` value from a [Config].
    ///
    /// If the `Config::symlink_arrow` has value,
    /// returns its value as the value of the `SymlinkArrow`, in a [Some].
    /// If no arrow is configured and classic is enabled,
    /// returns `->` in a [Some]; otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        match config.symlink_arrow.as_ref() {
            Some(arrow) => Some(SymlinkArrow(arrow.to_string())),
            None => {
                if config.classic == Some(true) {
                    Some(SymlinkArrow("->".to_string()))
                } else {
                    None
                }
            }
        }
    }
}

/// The default value for the `SymlinkArrow` is `\u{21d2}(⇒)`
impl Default for SymlinkArrow {
    fn default() -> Self {
        Self(String::from("\u{21d2}")) // ⇒
    }
}

use std::fmt;
impl fmt::Display for SymlinkArrow {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use super::SymlinkArrow;
    #[test]
    fn test_symlink_arrow_from_config_utf8() {
        let mut c = Config::with_none();
        c.symlink_arrow = Some("↹".into());
        assert_eq!(
            Some(SymlinkArrow(String::from("\u{21B9}"))),
            SymlinkArrow::from_config(&c)
        );
    }

    #[test]
    fn test_symlink_arrow_config_none_classic() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(
            Some(SymlinkArrow(String::from("->"))),
            SymlinkArrow::from_config(&c)
        );
    }

    #[test]
    fn test_symlink_arrow_config_some_classic() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.symlink_arrow = Some("↹".into());
        // the configured arrow gets precedence over the classic arrow
        assert_eq!(
            Some(SymlinkArrow(String::from("\u{21B9}"))),
            SymlinkArrow::from_config(&c)
        );
    }

    #[test]
    fn test_symlink_arrow_from_args_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SymlinkArrow::from_cli(&cli));
    }

    #[test]
    fn test_symlink_arrow_from_args_classic() {
        let argv = ["lsd", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(SymlinkArrow("->".to_string())),
            SymlinkArrow::from_cli(&cli)
        );
    }

    #[test]
    fn test_symlink_arrow_default() {
        assert_eq!(
            SymlinkArrow(String::from("\u{21d2}")),
            SymlinkArrow::default()
        );
    }

    #[test]
    fn test_symlink_display() {
        assert_eq!("⇒", format!("{}", SymlinkArrow::default()));
    }
}
