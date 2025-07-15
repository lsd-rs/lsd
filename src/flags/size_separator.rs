//! This module defines the [SizeSeparator]. To set it up from [Cli], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;
use crate::print_error;

use num_format::Locale;
use serde::Deserialize;

/// The flag showing which separator to use for file sizes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Default)]
pub struct SizeSeparator {
    locale: Option<String>,
}

impl SizeSeparator {
    pub fn new(locale: Option<String>) -> Self {
        Self { locale }
    }

    pub fn get_locale(&self) -> Option<Locale> {
        self.locale.as_ref().and_then(|s| {
            Locale::from_name(s).ok().or_else(|| {
                print_error!("Invalid locale: {s}");
                None
            })
        })
    }
}

impl Configurable<Self> for SizeSeparator {
    fn from_cli(cli: &Cli) -> Option<Self> {
        cli.size_separator
            .as_ref()
            .map(|s| Self::new(Some(s.clone())))
    }

    fn from_config(config: &Config) -> Option<Self> {
        config
            .size_separator
            .as_ref()
            .map(|s| Self::new(Some(s.clone())))
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::SizeSeparator;

    use crate::app::Cli;
    use crate::flags::Configurable;

    #[test]
    fn test_from_cli_bytes_with_separator() {
        let argv = ["lsd", "--size-separator", "en"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(
            Some(SizeSeparator::new(Some("en".to_string()))),
            SizeSeparator::from_cli(&cli)
        );
    }
}
