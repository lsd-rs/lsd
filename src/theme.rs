pub mod color;
pub mod git;
pub mod icon;

use std::path::Path;
use std::{fs, io};

use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;

use crate::config_file;
use crate::print_error;

use color::ColorTheme;
use git::GitThemeSymbols;
use icon::IconTheme;

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Theme {
    pub color: ColorTheme,
    pub icon: IconTheme,
    pub git_theme: GitThemeSymbols,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Theme file not existed")]
    NotExisted(#[from] io::Error),
    #[error("Theme file format invalid")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("Theme file path invalid {0}")]
    InvalidPath(String),
    #[error("Unknown Theme error")]
    Unknown(),
}

impl Theme {
    /// This read theme from file,
    /// use the file path if it is absolute
    /// prefix the config_file dir to it if it is not
    pub fn from_path<D>(file: &str) -> Result<D, Error>
    where
        D: DeserializeOwned + Default,
    {
        let real = if let Some(path) = config_file::Config::expand_home(file) {
            path
        } else {
            print_error!("Not a valid theme file path: {}.", &file);
            return Err(Error::InvalidPath(file.to_string()));
        };
        let path = if Path::new(&real).is_absolute() {
            real
        } else {
            match config_file::Config::config_file_path() {
                Some(p) => p.join(real),
                None => return Err(Error::InvalidPath("config home not existed".into())),
            }
        };

        // try `yml` if `yaml` extension file not found or error
        let mut err: Error = Error::Unknown();
        for ext in ["yaml", "yml"] {
            match fs::read(&path.with_extension(ext)) {
                Ok(f) => match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                    Ok(t) => return Ok(t),
                    Err(e) => {
                        err = Error::from(e);
                    }
                },
                Err(e) => err = Error::from(e),
            }
        }

        Err(err)
    }

    /// This constructs a Theme struct with a passed [Yaml] str.
    fn with_yaml<D>(yaml: &str) -> Result<D, serde_yaml::Error>
    where
        D: DeserializeOwned + Default,
    {
        if yaml.trim() == "" {
            return Ok(D::default());
        }
        serde_yaml::from_str::<D>(yaml)
    }
}
