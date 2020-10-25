///! This module provides methods to handle the program's config files and operations related to
///! this.
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::print_error;

#[cfg(not(windows))]
use xdg::BaseDirectories;
use yaml_rust::{Yaml, YamlLoader};

const CONF_DIR: &str = "lsd";
const CONF_FILE_NAME: &str = "config";
const YAML_LONG_EXT: &str = "yaml";
const YAML_SHORT_EXT: &str = "yml";

/// A struct to hold an optional file path [String] and an optional [Yaml], and provides methods
/// around error handling in a config file.
#[derive(Clone, Debug)]
pub struct Config {
    pub file: Option<String>,
    pub yaml: Option<Yaml>,
}

impl Config {
    /// This constructs a Config struct without a file [String] and without a [Yaml].
    pub fn with_none() -> Self {
        Self {
            file: None,
            yaml: None,
        }
    }

    /// This constructs a Config struct with a passed file [String] and without a [Yaml].
    pub fn with_file(file: String) -> Self {
        Self {
            file: Some(file),
            yaml: None,
        }
    }

    /// This constructs a Config struct with a passed [Yaml] and without a file [String].
    #[cfg(test)]
    pub fn with_yaml(yaml: Yaml) -> Self {
        Self {
            file: None,
            yaml: Some(yaml),
        }
    }

    /// This tries to read a configuration file like in the XDG_BASE_DIRS specification and returns
    /// the contents of the YAML config file.
    pub fn read_config(name: &str) -> Self {
        let config_file_long_path;
        let config_file_short_path;
        match Self::config_file_paths(name) {
            Some((long, short)) => {
                config_file_long_path = long;
                config_file_short_path = short;
            }
            _ => return Self::with_none(),
        }

        let mut out_config;
        let mut config_file;
        match File::open(&config_file_long_path) {
            Ok(result) => {
                config_file = result;
                out_config = Self::with_file(config_file_long_path.as_path().display().to_string());
            }
            Err(_) => match File::open(&config_file_short_path) {
                Ok(result) => {
                    config_file = result;
                    out_config =
                        Self::with_file(config_file_short_path.as_path().display().to_string());
                }
                Err(_) => return Self::with_none(),
            },
        }

        let mut config_content = String::new();
        if let Err(error) = config_file.read_to_string(&mut config_content) {
            print_error!("Found a config file, but could not read it: {}", error);
            return out_config;
        }

        match YamlLoader::load_from_str(&config_content) {
            Ok(result) => {
                if !result.is_empty() {
                    out_config.yaml = Some(result[0].clone());
                }
                out_config
            }
            Err(error) => {
                print_error!("Error parsing config: {}\n", error);
                out_config
            }
        }
    }

    /// This provides two paths for a configuration file (the first with the long yaml extension,
    /// the second with the short yml extension), according to the XDG_BASE_DIRS specification.
    #[cfg(not(windows))]
    pub fn config_file_paths(name: &str) -> Option<(PathBuf, PathBuf)> {
        let base_dirs;
        match BaseDirectories::with_prefix(CONF_DIR) {
            Ok(result) => base_dirs = result,
            _ => return None,
        }

        let config_file_long_path;
        match base_dirs.place_config_file([name, YAML_LONG_EXT].join(".")) {
            Ok(result) => config_file_long_path = result,
            _ => return None,
        }

        let config_file_short_path;
        match base_dirs.place_config_file([name, YAML_SHORT_EXT].join(".")) {
            Ok(result) => config_file_short_path = result,
            _ => return None,
        }

        Some((config_file_long_path, config_file_short_path))
    }

    /// This provides two paths for a configuration file (the first with the long yaml extension,
    /// the second with the short yml extension) inside the %APPDATA% directory.
    #[cfg(windows)]
    pub fn config_file_paths(name: &str) -> Option<(PathBuf, PathBuf)> {
        let mut config_file_long_path;
        match dirs::config_dir() {
            Some(path) => config_file_long_path = path,
            _ => return None,
        }

        config_file_long_path.push(CONF_DIR);
        let mut config_file_short_path = config_file_long_path.clone();

        config_file_long_path.push([name, YAML_LONG_EXT].join("."));
        config_file_short_path.push([name, YAML_SHORT_EXT].join("."));

        Some((config_file_long_path, config_file_short_path))
    }

    /// Returns whether the Config has a [Yaml].
    pub fn has_yaml(&self) -> bool {
        self.yaml.is_some()
    }

    /// This prints the provided warning message to stderr, prepending the executable name and the
    /// configuration file path that likely caused the warning.
    pub fn print_warning(&self, message: &str) {
        print_error!(
            "lsd: {} - {}\n",
            self.file.as_ref().unwrap_or(&String::from("")),
            message
        );
    }

    /// This prints a predetermined warning message to stderr, warning about an invalid value for a
    /// configuration element.
    pub fn print_invalid_value_warning(&self, name: &str, value: &str) {
        self.print_warning(&format!("Not a valid {} value: {}", name, value));
    }

    /// This prints a predetermined warning message to stderr, warning about a wrong [Yaml] data
    /// type for a configuration value.
    pub fn print_wrong_type_warning(&self, name: &str, type_name: &str) {
        self.print_warning(&format!(
            "The {} config value has to be a {}.",
            name, type_name
        ));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::read_config(CONF_FILE_NAME)
    }
}
