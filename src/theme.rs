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
    #[error("Can not read the theme file")]
    ReadFailed(#[from] io::Error),
    #[error("Theme file format invalid")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("Theme file path invalid {0}")]
    InvalidPath(String),
}

impl Theme {
    /// Read theme from a file path
    /// use the file path as-is if it is absolute
    /// search the config paths folders for it if not
    pub fn from_path<D>(file: &str) -> Result<D, Error>
    where
        D: DeserializeOwned + Default,
    {
        let real = if let Some(path) = config_file::expand_home(file) {
            path
        } else {
            print_error!("Not a valid theme file path: {}.", &file);
            return Err(Error::InvalidPath(file.to_string()));
        };

        let mut paths = if Path::new(&real).is_absolute() {
            vec![real].into_iter()
        } else {
            config_file::Config::config_paths()
                .map(|p| p.join(real.clone()))
                .collect::<Vec<_>>()
                .into_iter()
        };

        let Some(valid) = paths.find_map(|p| {
            let yaml = p.with_extension("yaml");
            let yml = p.with_extension("yml");
            if yaml.is_file() {
                Some(yaml)
            } else if yml.is_file() {
                Some(yml)
            } else {
                None
            }
        }) else {
            return Err(Error::InvalidPath("No valid theme file found".to_string()));
        };

        match fs::read_to_string(valid) {
            Ok(yaml) => match Self::with_yaml(&yaml) {
                Ok(t) => Ok(t),
                Err(e) => Err(Error::InvalidFormat(e)),
            },
            Err(e) => Err(Error::ReadFailed(e)),
        }
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
