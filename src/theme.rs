pub mod color;
pub mod icon;

use serde::{de::DeserializeOwned, Deserialize};
use std::fs;
use std::path::Path;

use crate::config_file;
use crate::print_error;

use color::ColorTheme;
use icon::IconTheme;

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Theme {
    pub color: ColorTheme,
    pub icon: IconTheme,
}

impl Theme {
    /// This read theme from file,
    /// use the file path if it is absolute
    /// prefix the config_file dir to it if it is not
    pub fn from_path<D: DeserializeOwned>(file: &str) -> Option<D> {
        let real = if let Some(path) = config_file::Config::expand_home(file) {
            path
        } else {
            print_error!("Not a valid theme file path: {}.", &file);
            return None;
        };
        let path = if Path::new(&real).is_absolute() {
            real
        } else {
            config_file::Config::config_file_path()?
                .join("themes")
                .join(real)
        };
        match fs::read(&path.with_extension("yaml")) {
            Ok(f) => match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                Ok(t) => Some(t),
                Err(e) => {
                    print_error!("Theme file {} format error: {}.", &file, e);
                    None
                }
            },
            Err(_) => {
                // try `yml` if `yaml` extension file not found
                match fs::read(&path.with_extension("yml")) {
                    Ok(f) => match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                        Ok(t) => Some(t),
                        Err(e) => {
                            print_error!("Theme file {} format error: {}.", &file, e);
                            None
                        }
                    },
                    Err(e) => {
                        print_error!("Not a valid theme: {}, {}.", path.to_string_lossy(), e);
                        None
                    }
                }
            }
        }
    }

    /// This constructs a Theme struct with a passed [Yaml] str.
    fn with_yaml<D: DeserializeOwned>(yaml: &str) -> Result<D, serde_yaml::Error> {
        serde_yaml::from_str::<D>(yaml)
    }
}
