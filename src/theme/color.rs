//! This module provides methods to create theme from files and operations related to
//! this.
use crossterm::style::Color;
use serde::{de::IntoDeserializer, Deserialize};
use std::fmt;

// Custom color deserialize
fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct ColorVisitor;
    impl<'de> serde::de::Visitor<'de> for ColorVisitor {
        type Value = Color;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                    "`black`, `blue`, `dark_blue`, `cyan`, `dark_cyan`, `green`, `dark_green`, `grey`, `dark_grey`, `magenta`, `dark_magenta`, `red`, `dark_red`, `white`, `yellow`, `dark_yellow`, `u8`, or `3 u8 array`",
                )
        }

        fn visit_str<E>(self, value: &str) -> Result<Color, E>
        where
            E: serde::de::Error,
        {
            Color::deserialize(value.into_deserializer())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Color, E>
        where
            E: serde::de::Error,
        {
            if value > 255 {
                return Err(E::invalid_value(
                    serde::de::Unexpected::Unsigned(value),
                    &self,
                ));
            }
            Ok(Color::AnsiValue(value as u8))
        }

        fn visit_seq<M>(self, mut seq: M) -> Result<Color, M::Error>
        where
            M: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            if let Some(size) = seq.size_hint() {
                if size != 3 {
                    return Err(serde::de::Error::invalid_length(
                        size,
                        &"a list of size 3(RGB)",
                    ));
                }
            }
            loop {
                match seq.next_element::<u8>() {
                    Ok(Some(x)) => {
                        values.push(x);
                    }
                    Ok(None) => break,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            // recheck as size_hint sometimes not working
            if values.len() != 3 {
                return Err(serde::de::Error::invalid_length(
                    values.len(),
                    &"a list of size 3(RGB)",
                ));
            }
            Ok(Color::from((values[0], values[1], values[2])))
        }
    }

    deserializer.deserialize_any(ColorVisitor)
}

/// A struct holding the theme configuration
/// Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ColorTheme {
    #[serde(deserialize_with = "deserialize_color")]
    pub user: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub group: Color,
    pub permission: Permission,
    pub attributes: Attributes,
    pub date: Date,
    pub size: Size,
    pub inode: INode,
    #[serde(deserialize_with = "deserialize_color")]
    pub tree_edge: Color,
    pub links: Links,
    pub git_status: GitStatus,

    #[serde(skip)]
    pub file_type: FileType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Permission {
    #[serde(deserialize_with = "deserialize_color")]
    pub read: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub write: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub exec: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub exec_sticky: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub no_access: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub octal: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub acl: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub context: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Attributes {
    #[serde(deserialize_with = "deserialize_color")]
    pub archive: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub read: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub hidden: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub system: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FileType {
    pub file: File,
    pub dir: Dir,
    #[serde(deserialize_with = "deserialize_color")]
    pub pipe: Color,
    pub symlink: Symlink,
    #[serde(deserialize_with = "deserialize_color")]
    pub block_device: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub char_device: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub socket: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub special: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct File {
    #[serde(deserialize_with = "deserialize_color")]
    pub exec_uid: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub uid_no_exec: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub exec_no_uid: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub no_exec_no_uid: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Dir {
    #[serde(deserialize_with = "deserialize_color")]
    pub uid: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub no_uid: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Symlink {
    #[serde(deserialize_with = "deserialize_color")]
    pub default: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub broken: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub missing_target: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Date {
    #[serde(deserialize_with = "deserialize_color")]
    pub hour_old: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub day_old: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub older: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Size {
    #[serde(deserialize_with = "deserialize_color")]
    pub none: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub small: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub medium: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub large: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct INode {
    #[serde(deserialize_with = "deserialize_color")]
    pub valid: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub invalid: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Links {
    #[serde(deserialize_with = "deserialize_color")]
    pub valid: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub invalid: Color,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct GitStatus {
    #[serde(deserialize_with = "deserialize_color")]
    pub default: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub unmodified: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub ignored: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub new_in_index: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub new_in_workdir: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub typechange: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub deleted: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub renamed: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub modified: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub conflicted: Color,
}

impl Default for Permission {
    fn default() -> Self {
        Permission {
            read: Color::DarkGreen,
            write: Color::DarkYellow,
            exec: Color::DarkRed,
            exec_sticky: Color::AnsiValue(5),
            no_access: Color::AnsiValue(245), // Grey
            octal: Color::AnsiValue(6),
            acl: Color::DarkCyan,
            context: Color::Cyan,
        }
    }
}
impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            archive: Color::DarkGreen,
            read: Color::DarkYellow,
            hidden: Color::AnsiValue(13), // Pink,
            system: Color::AnsiValue(13), // Pink,
        }
    }
}
impl Default for FileType {
    fn default() -> Self {
        FileType {
            file: File::default(),
            dir: Dir::default(),
            symlink: Symlink::default(),
            pipe: Color::AnsiValue(44),         // DarkTurquoise
            block_device: Color::AnsiValue(44), // DarkTurquoise
            char_device: Color::AnsiValue(172), // Orange3
            socket: Color::AnsiValue(44),       // DarkTurquoise
            special: Color::AnsiValue(44),      // DarkTurquoise
        }
    }
}
impl Default for File {
    fn default() -> Self {
        File {
            exec_uid: Color::AnsiValue(40),        // Green3
            uid_no_exec: Color::AnsiValue(184),    // Yellow3
            exec_no_uid: Color::AnsiValue(40),     // Green3
            no_exec_no_uid: Color::AnsiValue(184), // Yellow3
        }
    }
}
impl Default for Dir {
    fn default() -> Self {
        Dir {
            uid: Color::AnsiValue(33),    // DodgerBlue1
            no_uid: Color::AnsiValue(33), // DodgerBlue1
        }
    }
}
impl Default for Symlink {
    fn default() -> Self {
        Symlink {
            default: Color::AnsiValue(44),         // DarkTurquoise
            broken: Color::AnsiValue(124),         // Red3
            missing_target: Color::AnsiValue(124), // Red3
        }
    }
}
impl Default for Date {
    fn default() -> Self {
        Date {
            hour_old: Color::AnsiValue(40), // Green3
            day_old: Color::AnsiValue(42),  // SpringGreen2
            older: Color::AnsiValue(36),    // DarkCyan
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            none: Color::AnsiValue(245),   // Grey
            small: Color::AnsiValue(229),  // Wheat1
            medium: Color::AnsiValue(216), // LightSalmon1
            large: Color::AnsiValue(172),  // Orange3
        }
    }
}
impl Default for INode {
    fn default() -> Self {
        INode {
            valid: Color::AnsiValue(13),    // Pink
            invalid: Color::AnsiValue(245), // Grey
        }
    }
}
impl Default for Links {
    fn default() -> Self {
        Links {
            valid: Color::AnsiValue(13),    // Pink
            invalid: Color::AnsiValue(245), // Grey
        }
    }
}

impl Default for GitStatus {
    fn default() -> Self {
        GitStatus {
            default: Color::AnsiValue(245),    // Grey
            unmodified: Color::AnsiValue(245), // Grey
            ignored: Color::AnsiValue(245),    // Grey
            new_in_index: Color::DarkGreen,
            new_in_workdir: Color::DarkGreen,
            typechange: Color::DarkYellow,
            deleted: Color::DarkRed,
            renamed: Color::DarkGreen,
            modified: Color::DarkYellow,
            conflicted: Color::DarkRed,
        }
    }
}

impl Default for ColorTheme {
    fn default() -> Self {
        // TODO(zwpaper): check terminal color and return light or dark
        Self::default_dark()
    }
}

impl ColorTheme {
    pub fn default_dark() -> Self {
        ColorTheme {
            user: Color::AnsiValue(230),  // Cornsilk1
            group: Color::AnsiValue(187), // LightYellow3
            permission: Permission::default(),
            attributes: Attributes::default(),
            file_type: FileType::default(),
            date: Date::default(),
            size: Size::default(),
            inode: INode::default(),
            links: Links::default(),
            tree_edge: Color::AnsiValue(245), // Grey
            git_status: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColorTheme;
    use crate::theme::Theme;

    fn default_yaml() -> &'static str {
        r#"---
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

    #[test]
    fn test_default_theme() {
        assert_eq!(
            ColorTheme::default_dark(),
            Theme::with_yaml(default_yaml()).unwrap()
        );
    }

    #[test]
    fn test_default_theme_file() {
        use std::fs::File;
        use std::io::Write;
        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("theme.yaml");
        let mut file = File::create(&theme).unwrap();
        writeln!(file, "{}", default_yaml()).unwrap();

        assert_eq!(
            ColorTheme::default_dark(),
            Theme::from_path(theme.to_str().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_empty_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme: ColorTheme = Theme::with_yaml("user: 230").unwrap(); // 230 is the default value
        let default_theme = ColorTheme::default_dark();
        assert_eq!(empty_theme, default_theme);
    }

    #[test]
    fn test_first_level_theme_return_default_but_changed() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme: ColorTheme = Theme::with_yaml("user: 130").unwrap();
        let mut theme = ColorTheme::default_dark();
        use crossterm::style::Color;
        theme.user = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }

    #[test]
    fn test_hexadecimal_colors() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme: ColorTheme = Theme::with_yaml("user: \"#ff007f\"").unwrap();
        assert_eq!(
            empty_theme.user,
            crossterm::style::Color::Rgb {
                r: 255,
                g: 0,
                b: 127
            }
        );
    }

    #[test]
    fn test_second_level_theme_return_default_but_changed() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme: ColorTheme = Theme::with_yaml(
            r#"---
permission:
  read: 130"#,
        )
        .unwrap();
        let mut theme = ColorTheme::default_dark();
        use crossterm::style::Color;
        theme.permission.read = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }
}
