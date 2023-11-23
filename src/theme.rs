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
    #[error("Cannot read theme file. {0}")]
    NotExisted(#[from] io::Error),
    #[error("Theme file format invalid. {0}")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("Theme file path invalid. {0}")]
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
                        break;
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

#[cfg(test)]
mod tests {

    use super::Error;
    use super::Theme;

    #[test]
    fn test_can_deserialize_yaml() {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        map.insert("user".to_string(), "1".to_string());
        map.insert("group".to_string(), "2".to_string());
        assert_eq!(
            map,
            Theme::with_yaml(
                r#"---
                    user: 1
                    group: 2
                "#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_ioerror() {
        use super::ColorTheme;

        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("does-not-exist.yaml");

        let res = Theme::from_path::<ColorTheme>(theme.to_str().unwrap());
        assert!(res.is_err());
        let the_error = res.unwrap_err();
        assert!(matches!(&the_error, Error::NotExisted(_)));
        if let Error::NotExisted(some_err) = &the_error {
            assert_eq!(some_err.kind(), std::io::ErrorKind::NotFound);
        }

        // There are many reasons why we could get an IoError, not just "file not found".
        // Here we test that we actually get informations about the underlying io error.
        assert_eq!(
            "Cannot read theme file. No such file or directory (os error 2)".to_string(),
            the_error.to_string()
        );
    }

    #[test]
    fn test_invalid_format() {
        use super::ColorTheme;
        use std::fs::File;
        use std::io::Write;

        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("does-not-exist.yaml");
        let mut file = File::create(&theme).unwrap();
        // Write a purposefully bogus file
        writeln!(file, "bogus-field: 1").unwrap();

        let res = Theme::from_path::<ColorTheme>(theme.to_str().unwrap());
        assert!(res.is_err());
        // Just check the first part of serde_yaml output so that we don't break the test just adding new fields.
        assert!(res.unwrap_err().to_string().starts_with(
            "Theme file format invalid. unknown field `bogus-field`, expected one of"
        ));
    }
}
