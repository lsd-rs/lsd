use serde::Deserialize;
use serde_yaml::Sequence;
use std::error::Error;
///! This module provides methods to handle the program's config files and operations related to
///! this.
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use xdg::BaseDirectories;

use crate::print_error;

const CONF_DIR: &str = "lsd";
const CONF_FILE_NAME: &str = "config";
const YAML_LONG_EXT: &str = "yaml";

/// A struct to hold an optional file path [String] and an optional [Yaml], and provides methods
/// around error handling in a config file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub classic: Option<bool>,
    pub blocks: Option<Sequence>,
    pub color: Option<Color>,
    pub date: Option<String>, // enum?
    pub dereference: Option<bool>,
    pub display: Option<String>, // enum?
    pub icons: Option<Icons>,
    pub ignore_globs: Option<Sequence>,
    pub indicators: Option<bool>,
    pub layout: Option<String>, // enum?
    pub recursion: Option<Recursion>,
    pub size: Option<String>, // enum?
    pub sorting: Option<Sorting>,
    pub no_symlink: Option<bool>,
    pub total_size: Option<bool>,
    pub styling: Option<Styling>,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub when: String, // enum?
}

#[derive(Debug, Deserialize)]
pub struct Icons {
    pub when: String, // enum?
    pub theme: String,
}

#[derive(Debug, Deserialize)]
pub struct Recursion {
    pub enabled: bool,
    pub depth: i32,
}

#[derive(Debug, Deserialize)]
pub struct Sorting {
    pub column: String, // enum?
    pub reverse: bool,
    pub dir_grouping: String, // enum?
}

#[derive(Debug, Deserialize)]
pub struct Styling {
    pub symlink_arrow: bool,
}

impl Config {
    /// This constructs a Config struct without a file [String] and without a [Yaml].
    pub fn with_none() -> Self {
        Self::default()
    }

    /// This constructs a Config struct with a passed file [String] and without a [Yaml].
    // TODO(zhangwei) Box<Error>
    pub fn with_file(file: String) -> Option<Self> {
        match fs::read(&file) {
            Ok(f) => Self::with_yaml(&String::from_utf8_lossy(&f)),
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => print_error!("bad config file: {}, {}\n", &file, e),
                };
                None
            }
        }
    }

    /// This constructs a Config struct with a passed [Yaml] and without a file [String].
    fn with_yaml(yaml: &str) -> Option<Self> {
        match serde_yaml::from_str(yaml) {
            Ok(c) => Some(c),
            Err(e) => {
                print_error!("configuration file format error, {}\n\n", e);
                None
            }
        }
    }

    /// This provides the path for a configuration file, according to the XDG_BASE_DIRS specification.
    /// not checking the error because this is static
    #[cfg(not(windows))]
    fn config_file_path() -> PathBuf {
        BaseDirectories::with_prefix(CONF_DIR)
            .unwrap()
            .place_config_file([CONF_FILE_NAME, YAML_LONG_EXT].join("."))
            .unwrap()
    }

    /// This provides the path for a configuration file, inside the %APPDATA% directory.
    /// not checking the error because this is static
    #[cfg(windows)]
    fn config_file_path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join(CONF_DIR)
            .join(CONF_FILE_NAME)
            .set_extension(YAML_LONG_EXT)
    }
}

impl Default for Config {
    fn default() -> Self {
        if let Some(c) = Self::with_file(Self::config_file_path().to_string_lossy().to_string()) {
            c
        } else {
            Config {
                classic: Some(false),
                blocks: None,
                color: None,
                date: None,
                dereference: None,
                display: None,
                icons: None,
                ignore_globs: None,
                indicators: None,
                layout: None,
                recursion: None,
                size: None,
                sorting: None,
                no_symlink: None,
                total_size: None,
                styling: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn test_read_config_ok() {
        let c = Config::with_yaml("classic: true").unwrap();
        println!("{:?}", c);
        assert!(c.classic.unwrap())
    }

    #[test]
    fn test_read_config_bad_bool() {
        let c = Config::with_yaml("classic: notbool");
        println!("{:?}", c);
        assert!(c.is_some())
    }

    #[test]
    fn test_read_config_file_not_found() {
        let c = Config::with_file("not-existed".to_string());
        assert!(c.is_none())
    }
}
