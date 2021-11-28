///! This module provides methods to create theme from files and operations related to
///! this.
use crate::config_file;
use crate::print_error;

use crossterm::style::Color;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// A struct holding the theme configuration
/// Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.avg
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub default: Color,
    pub user: Option<Color>,
    pub group: Option<Color>,
    pub permission: Option<Permission>,
    pub date: Option<Date>,
    pub size: Option<Size>,
    pub inode: Option<INode>,
    pub tree_edge: Option<Color>,
    pub links: Option<Links>,

    #[serde(skip)]
    pub file_type: FileType,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Permission {
    pub read: Option<Color>,
    pub write: Option<Color>,
    pub exec: Option<Color>,
    pub exec_sticky: Option<Color>,
    pub no_access: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct FileType {
    pub file: Option<File>,
    pub dir: Option<Dir>,
    pub pipe: Option<Color>,
    pub symlink: Option<Symlink>,
    pub block_device: Option<Color>,
    pub char_device: Option<Color>,
    pub socket: Option<Color>,
    pub special: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct File {
    pub exec_uid: Option<Color>,
    pub uid_no_exec: Option<Color>,
    pub exec_no_uid: Option<Color>,
    pub no_exec_no_uid: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Dir {
    pub uid: Option<Color>,
    pub no_uid: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Symlink {
    pub default: Option<Color>,
    pub broken: Option<Color>,
    pub missing_target: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Date {
    pub hour_old: Option<Color>,
    pub day_old: Option<Color>,
    pub older: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub none: Option<Color>,
    pub small: Option<Color>,
    pub medium: Option<Color>,
    pub large: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct INode {
    pub valid: Option<Color>,
    pub invalid: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Links {
    pub valid: Option<Color>,
    pub invalid: Option<Color>,
}

impl Default for FileType {
    fn default() -> Self {
        Theme::default_dark().file_type
    }
}

impl Default for Theme {
    fn default() -> Self {
        // TODO(zwpaper): check terminal color and return light or dark
        Self::default_dark()
    }
}

impl Theme {
    /// This read theme from file,
    /// use the file path if it is absolute
    /// prefix the config_file dir to it if it is not
    pub fn from_path(file: &str) -> Option<Self> {
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
    fn with_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(yaml)
    }

    pub fn default_dark() -> Self {
        Theme {
            default: Color::AnsiValue(245),
            user: Some(Color::AnsiValue(230)),  // Cornsilk1
            group: Some(Color::AnsiValue(187)), // LightYellow3
            permission: Some(Permission {
                read: Some(Color::DarkGreen),
                write: Some(Color::DarkYellow),
                exec: Some(Color::DarkRed),
                exec_sticky: Some(Color::AnsiValue(5)),
                no_access: Some(Color::AnsiValue(245)), // Grey
            }),
            file_type: FileType {
                file: Some(File {
                    exec_uid: Some(Color::AnsiValue(40)),        // Green3
                    uid_no_exec: Some(Color::AnsiValue(184)),    // Yellow3
                    exec_no_uid: Some(Color::AnsiValue(40)),     // Green3
                    no_exec_no_uid: Some(Color::AnsiValue(184)), // Yellow3
                }),
                dir: Some(Dir {
                    uid: Some(Color::AnsiValue(33)),    // DodgerBlue1
                    no_uid: Some(Color::AnsiValue(33)), // DodgerBlue1
                }),
                pipe: Some(Color::AnsiValue(44)), // DarkTurquoise
                symlink: Some(Symlink {
                    default: Some(Color::AnsiValue(44)),         // DarkTurquoise
                    broken: Some(Color::AnsiValue(124)),         // Red3
                    missing_target: Some(Color::AnsiValue(124)), // Red3
                }),
                block_device: Some(Color::AnsiValue(44)), // DarkTurquoise
                char_device: Some(Color::AnsiValue(172)), // Orange3
                socket: Some(Color::AnsiValue(44)),       // DarkTurquoise
                special: Some(Color::AnsiValue(44)),      // DarkTurquoise
            },
            date: Some(Date {
                hour_old: Some(Color::AnsiValue(40)), // Green3
                day_old: Some(Color::AnsiValue(42)),  // SpringGreen2
                older: Some(Color::AnsiValue(36)),    // DarkCyan
            }),
            size: Some(Size {
                none: Some(Color::AnsiValue(245)),   // Grey
                small: Some(Color::AnsiValue(229)),  // Wheat1
                medium: Some(Color::AnsiValue(216)), // LightSalmon1
                large: Some(Color::AnsiValue(172)),  // Orange3
            }),
            inode: Some(INode {
                valid: Some(Color::AnsiValue(13)),    // Pink
                invalid: Some(Color::AnsiValue(245)), // Grey
            }),
            links: Some(Links {
                valid: Some(Color::AnsiValue(13)),    // Pink
                invalid: Some(Color::AnsiValue(245)), // Grey
            }),
            tree_edge: Some(Color::AnsiValue(245)), // Grey
        }
    }

    #[cfg(test)]
    pub fn default_yaml() -> &'static str {
        r#"---
default: 245
user: 230
group: 187
permission:
  read: dark_green
  write: dark_yellow
  exec: dark_red
  exec-sticky: 5
  no-access: 245
date:
  hour-old: 40
  day-old: 42
  older: 36
size:
  none: 245
  small: 229
  medium: 216
  large: 172
inode:
  valid: 13
  invalid: 245
links:
  valid: 13
  invalid: 245
tree-edge: 245
"#
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn test_default_theme() {
        assert_eq!(
            Theme::default_dark(),
            Theme::with_yaml(Theme::default_yaml()).unwrap()
        );
    }

    #[test]
    fn test_default_theme_file() {
        use std::fs::File;
        use std::io::Write;
        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("theme.yaml");
        let mut file = File::create(&theme).unwrap();
        writeln!(file, "{}", Theme::default_yaml()).unwrap();

        assert_eq!(
            Theme::default_dark(),
            Theme::from_path(theme.to_str().unwrap()).unwrap()
        );
    }
}
