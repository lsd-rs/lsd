//! This module defines the [PermissionFlag]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file permissions units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionFlag {
    /// The variant to show file permissions in rwx format
    #[cfg_attr(not(target_os = "windows"), default)]
    Rwx,
    /// The variant to show file permissions in octal format
    Octal,
    /// (windows only): Attributes from powershell's `Get-ChildItem`
    #[cfg_attr(target_os = "windows", default)]
    Attributes,
    /// Disable the display of owner and permissions, may be used to speed up in Windows
    Disable,
}

impl PermissionFlag {
    fn from_arg_str(value: &str) -> Self {
        match value {
            "rwx" => Self::Rwx,
            "octal" => Self::Octal,
            "attributes" => Self::Attributes,
            "disable" => Self::Disable,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'permission'"),
        }
    }
}

impl Configurable<Self> for PermissionFlag {
    /// Get a potential `PermissionFlag` variant from [Cli].
    ///
    /// If any of the "rwx" or "octal" arguments is passed, the corresponding
    /// `PermissionFlag` variant is returned in a [Some]. If neither of them is passed,
    /// this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Rwx)
        } else {
            cli.permission.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `PermissionFlag` variant from a [Config].
    ///
    /// If the `Config::permissions` has value and is one of "rwx" or "octal",
    /// this returns the corresponding `PermissionFlag` variant in a [Some].
    /// Otherwise this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Rwx)
        } else {
            config.permission
        }
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::PermissionFlag;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_default() {
        let expected = if cfg!(target_os = "windows") {
            PermissionFlag::Attributes
        } else {
            PermissionFlag::Rwx
        };
        assert_eq!(expected, PermissionFlag::default());
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, PermissionFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_default() {
        let argv = ["lsd", "--permission", "rwx"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_short() {
        let argv = ["lsd", "--permission", "octal"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_attributes() {
        let argv = ["lsd", "--permission", "attributes"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(PermissionFlag::Attributes),
            PermissionFlag::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_permissions_disable() {
        let argv = ["lsd", "--permission", "disable"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(PermissionFlag::Disable),
            PermissionFlag::from_cli(&cli)
        );
    }

    #[test]
    #[should_panic]
    fn test_from_cli_unknown() {
        let argv = ["lsd", "--permission", "unknown"];
        let _ = Cli::try_parse_from(argv).unwrap();
    }
    #[test]
    fn test_from_cli_permissions_multi() {
        let argv = ["lsd", "--permission", "octal", "--permission", "rwx"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_permissions_classic() {
        let argv = ["lsd", "--permission", "rwx", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, PermissionFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_rwx() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Rwx);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_octal() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Octal);
        assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }
}
