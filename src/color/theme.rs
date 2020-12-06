///! This module provides methods to create theme from files and operations related to
///! this.
use crate::config_file;
use crate::print_error;

use ansi_term::Colour;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// A struct holding the theme configuration
/// Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.avg
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub user: Colour,
    pub group: Colour,
    pub permissions: Permissions,
    pub file_type: FileType,
    pub modified: Modified,
    pub size: Size,
    pub inode: INode,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Permissions {
    pub read: Colour,
    pub write: Colour,
    pub exec: Colour,
    pub exec_sticky: Colour,
    pub no_access: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct FileType {
    pub file: File,
    pub dir: Dir,
    pub pipe: Colour,
    pub symlink: Symlink,
    pub block_device: Colour,
    pub char_device: Colour,
    pub socket: Colour,
    pub special: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct File {
    pub exec_uid: Colour,
    pub uid_no_exec: Colour,
    pub exec_no_uid: Colour,
    pub no_exec_no_uid: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Dir {
    pub uid: Colour,
    pub no_uid: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Symlink {
    pub default: Colour,
    pub broken: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Modified {
    pub hour_old: Colour,
    pub day_old: Colour,
    pub older: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub none: Colour,
    pub small: Colour,
    pub medium: Colour,
    pub large: Colour,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct INode {
    pub valid: Colour,
    pub invalid: Colour,
}

impl Default for Theme {
    fn default() -> Self {
        // TODO: check terminal color and return light or dark
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
            config_file::Config::config_file_path().unwrap().join(real)
        };
        match fs::read(&path) {
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

    /// This constructs a Theme struct with a passed [Yaml] str.
    fn with_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(yaml)
    }

    pub fn default_dark() -> Self {
        Theme {
            user: Colour::Fixed(230),  // Cornsilk1
            group: Colour::Fixed(187), // LightYellow3
            permissions: Permissions {
                read: Colour::Green,
                write: Colour::Yellow,
                exec: Colour::Red,
                exec_sticky: Colour::Purple,
                no_access: Colour::Fixed(245), // Grey
            },
            file_type: FileType {
                file: File {
                    exec_uid: Colour::Fixed(40),        // Green3
                    uid_no_exec: Colour::Fixed(184),    // Yellow3
                    exec_no_uid: Colour::Fixed(40),     // Green3
                    no_exec_no_uid: Colour::Fixed(184), // Yellow3
                },
                dir: Dir {
                    uid: Colour::Fixed(33),    // DodgerBlue1
                    no_uid: Colour::Fixed(33), // DodgerBlue1
                },
                pipe: Colour::Fixed(44), // DarkTurquoise
                symlink: Symlink {
                    default: Colour::Fixed(44), // DarkTurquoise
                    broken: Colour::Fixed(124), // Red3
                },
                block_device: Colour::Fixed(44), // DarkTurquoise
                char_device: Colour::Fixed(172), // Orange3
                socket: Colour::Fixed(44),       // DarkTurquoise
                special: Colour::Fixed(44),      // DarkTurquoise
            },
            modified: Modified {
                hour_old: Colour::Fixed(40), // Green3
                day_old: Colour::Fixed(42),  // SpringGreen2
                older: Colour::Fixed(36),    // DarkCyan
            },
            size: Size {
                none: Colour::Fixed(245),   // Grey
                small: Colour::Fixed(229),  // Wheat1
                medium: Colour::Fixed(216), // LightSalmon1
                large: Colour::Fixed(172),  // Orange3
            },
            inode: INode {
                valid: Colour::Fixed(13),    // Pink
                invalid: Colour::Fixed(245), // Grey
            },
        }
    }

    #[cfg(test)]
    pub fn default_yaml() -> &'static str {
        r#"---
user:
  Fixed: 230
group:
  Fixed: 187
permissions:
  read: Green
  write: Yellow
  exec: Red
  exec-sticky: Purple
  no-access:
    Fixed: 245
file-type:
  file:
    exec-uid:
      Fixed: 40
    uid-no-exec:
      Fixed: 184
    exec-no-uid:
      Fixed: 40
    no-exec-no-uid:
      Fixed: 184
  dir:
    uid:
      Fixed: 33
    no-uid:
      Fixed: 33
  pipe:
    Fixed: 44
  symlink:
    default:
      Fixed: 44
    broken:
      Fixed: 124
  block-device:
    Fixed: 44
  char-device:
    Fixed: 172
  socket:
    Fixed: 44
  special:
    Fixed: 44
modified:
  hour-old:
    Fixed: 40
  day-old:
    Fixed: 42
  older:
    Fixed: 36
size:
  none:
    Fixed: 245
  small:
    Fixed: 229
  medium:
    Fixed: 216
  large:
    Fixed: 172
inode:
  valid:
    Fixed: 13
  invalid:
    Fixed: 245
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
