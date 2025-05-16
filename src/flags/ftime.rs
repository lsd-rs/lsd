//! This module defines the [FileTimeFlag]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;
use crate::print_error;

/// The flag showing which kind of time stamps to display.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum FileTimeFlag {
    #[default]
    Modification,
    Birth,
    Access,
}

impl FileTimeFlag {
    /// Get a value from a str.
    fn from_str<S: AsRef<str>>(value: S) -> Option<Self> {
        let value = value.as_ref();
        match value {
            "mtime" => Some(Self::Modification),
            "btime" => Some(Self::Birth),
            "atime" => Some(Self::Access),
            _ => {
                print_error!("Not a valid ftime value: {}.", value);
                None
            }
        }
    }
}

impl Configurable<Self> for FileTimeFlag {
    /// Get a potential `FileTimeFlag` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [FileTimeFlag::ModificationTime] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Modification)
        } else {
            cli.ftime.as_deref().and_then(Self::from_str)
        }
    }

    /// Get a potential `FileTimeFlag` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(FileTimeFlag::Date),
    /// Otherwise if the `Config::date` has value and is one of "date", "locale" or "relative",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Modification)
        } else {
            config.ftime.as_ref().and_then(Self::from_str)
        }
    }

    /// Get a potential `FileTimeFlag` variant from the environment.
    fn from_environment() -> Option<Self> {
        if let Ok(value) = std::env::var("LSD_FILE_TIME") {
            match value.as_str() {
                "atime" => Some(Self::Access),
                "btime" => Some(Self::Birth),
                "mtime" => Some(Self::Modification),
                _ => {
                    print_error!("Not a valid date value: {}.", value);
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::FileTimeFlag;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, FileTimeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_mtime() {
        let argv = ["lsd", "--ftime", "mtime"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(FileTimeFlag::Modification),
            FileTimeFlag::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_atime() {
        let argv = ["lsd", "--ftime", "atime"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(FileTimeFlag::Access), FileTimeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_btime() {
        let argv = ["lsd", "--ftime", "btime"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(FileTimeFlag::Birth), FileTimeFlag::from_cli(&cli));
    }

    #[test]
    #[should_panic = "possible values: mtime, btime, atime"]
    fn test_from_cli_invalid() {
        let argv = ["lsd", "--ftime", "foo"];
        let cli = Cli::try_parse_from(argv).unwrap();
        dbg!(&cli);
        FileTimeFlag::from_cli(&cli);
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--ftime", "mtime", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(FileTimeFlag::Modification),
            FileTimeFlag::from_cli(&cli)
        );
    }

    #[test]
    fn test_from_cli_ftime_multi() {
        let argv = ["lsd", "--ftime", "mtime", "--ftime", "atime"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(FileTimeFlag::Access), FileTimeFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, FileTimeFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_mtime() {
        let mut c = Config::with_none();
        c.ftime = Some("mtime".into());

        assert_eq!(
            Some(FileTimeFlag::Modification),
            FileTimeFlag::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_atime() {
        let mut c = Config::with_none();
        c.ftime = Some("atime".into());
        assert_eq!(Some(FileTimeFlag::Access), FileTimeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_btime() {
        let mut c = Config::with_none();
        c.ftime = Some("btime".into());
        assert_eq!(Some(FileTimeFlag::Birth), FileTimeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.ftime = Some("atime".into());
        c.classic = Some(true);
        assert_eq!(
            Some(FileTimeFlag::Modification),
            FileTimeFlag::from_config(&c)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_none() {
        std::env::set_var("LSD_FILE_TIME", "");
        assert_eq!(None, FileTimeFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_mtime() {
        std::env::set_var("LSD_FILE_TIME", "mtime");
        assert_eq!(
            Some(FileTimeFlag::Modification),
            FileTimeFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_atime() {
        std::env::set_var("LSD_FILE_TIME", "atime");
        assert_eq!(Some(FileTimeFlag::Access), FileTimeFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_btime() {
        std::env::set_var("LSD_FILE_TIME", "btime");
        assert_eq!(Some(FileTimeFlag::Birth), FileTimeFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_arg() {
        std::env::set_var("LSD_FILE_TIME", "mtime");
        let argv = ["lsd", "--ftime", "atime"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.ftime = Some("btime".into());
        assert_eq!(
            FileTimeFlag::Access,
            FileTimeFlag::configure_from(&cli, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_env() {
        std::env::set_var("LSD_FILE_TIME", "mtime");
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.ftime = Some("btime".into());
        assert_eq!(
            FileTimeFlag::Modification,
            FileTimeFlag::configure_from(&cli, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_config() {
        std::env::set_var("LSD_FILE_TIME", "");
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.ftime = Some("btime".into());
        assert_eq!(
            FileTimeFlag::Birth,
            FileTimeFlag::configure_from(&cli, &config)
        );
    }
}
