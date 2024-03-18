//! This module defines the [DateFlag]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::{self, Cli};
use crate::config_file::Config;
use crate::print_error;

/// The flag showing which kind of time stamps to display.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum DateFlag {
    #[default]
    Date,
    Locale,
    Relative,
    Iso,
    Formatted(String),
}

impl DateFlag {
    /// Get a value from a date format string
    fn from_format_string(value: &str) -> Option<Self> {
        if app::validate_time_format(value).is_ok() {
            Some(Self::Formatted(value[1..].to_string()))
        } else {
            print_error!("Not a valid date format: {}.", value);
            None
        }
    }

    /// Get a value from a str.
    fn from_str<S: AsRef<str>>(value: S) -> Option<Self> {
        let value = value.as_ref();
        match value {
            "date" => Some(Self::Date),
            "locale" => Some(Self::Locale),
            "relative" => Some(Self::Relative),
            _ if value.starts_with('+') => Self::from_format_string(value),
            _ => {
                print_error!("Not a valid date value: {}.", value);
                None
            }
        }
    }
}

impl Configurable<Self> for DateFlag {
    /// Get a potential `DateFlag` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [DateFlag::Date] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Date)
        } else {
            cli.date.as_deref().and_then(Self::from_str)
        }
    }

    /// Get a potential `DateFlag` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(DateFlag::Date),
    /// Otherwise if the `Config::date` has value and is one of "date", "locale" or "relative",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Date)
        } else {
            config.date.as_ref().and_then(Self::from_str)
        }
    }

    /// Get a potential `DateFlag` variant from the environment.
    fn from_environment() -> Option<Self> {
        if let Ok(value) = std::env::var("TIME_STYLE") {
            match value.as_str() {
                "full-iso" => Some(Self::Formatted("%F %T.%f %z".into())),
                "long-iso" => Some(Self::Formatted("%F %R".into())),
                "locale" => Some(Self::Locale),
                "iso" => Some(Self::Iso),
                _ if value.starts_with('+') => Self::from_format_string(&value),
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

    use super::DateFlag;

    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_date() {
        let argv = ["lsd", "--date", "date"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_locale() {
        let argv = ["lsd", "--date", "locale"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DateFlag::Locale), DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_relative() {
        let argv = ["lsd", "--date", "relative"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DateFlag::Relative), DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_format() {
        let argv = ["lsd", "--date", "+%F"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_cli(&cli)
        );
    }

    #[test]
    #[should_panic(expected = "invalid format specifier: %J")]
    fn test_from_cli_format_invalid() {
        let argv = ["lsd", "--date", "+%J"];
        let cli = Cli::try_parse_from(argv).unwrap();
        DateFlag::from_cli(&cli);
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--date", "date", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_date_multi() {
        let argv = ["lsd", "--date", "relative", "--date", "date"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, DateFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_date() {
        let mut c = Config::with_none();
        c.date = Some("date".into());

        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_relative() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        assert_eq!(Some(DateFlag::Relative), DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_format() {
        let mut c = Config::with_none();
        c.date = Some("+%F".into());
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_format_invalid() {
        let mut c = Config::with_none();
        c.date = Some("+%J".into());
        assert_eq!(None, DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        c.classic = Some(true);
        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(&c));
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_none() {
        std::env::set_var("TIME_STYLE", "");
        assert_eq!(None, DateFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_full_iso() {
        std::env::set_var("TIME_STYLE", "full-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %T.%f %z".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_long_iso() {
        std::env::set_var("TIME_STYLE", "long-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %R".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_iso() {
        std::env::set_var("TIME_STYLE", "iso");
        assert_eq!(Some(DateFlag::Iso), DateFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_format() {
        std::env::set_var("TIME_STYLE", "+%F");
        assert_eq!(
            Some(DateFlag::Formatted("%F".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_arg() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = ["lsd", "--date", "+%F"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%F".into()),
            DateFlag::configure_from(&cli, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_env() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%R".into()),
            DateFlag::configure_from(&cli, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_config() {
        std::env::set_var("TIME_STYLE", "");
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%c".into()),
            DateFlag::configure_from(&cli, &config)
        );
    }
}
