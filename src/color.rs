mod theme;

use theme::Theme;

pub use crate::flags::color::ThemeOption;

use ansi_term::{ANSIString, Colour, Style};
use lscolors::{Indicator, LsColors};
use std::path::Path;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Elem {
    /// Node type
    File {
        exec: bool,
        uid: bool,
    },
    SymLink,
    BrokenSymLink,
    Dir {
        uid: bool,
    },
    Pipe,
    BlockDevice,
    CharDevice,
    Socket,
    Special,

    /// Permissions
    Read,
    Write,
    Exec,
    ExecSticky,
    NoAccess,

    /// Last Time Modified
    DayOld,
    HourOld,
    Older,

    /// User / Group Name
    User,
    Group,

    /// File Size
    NonFile,
    FileLarge,
    FileMedium,
    FileSmall,

    /// INode
    INode {
        valid: bool,
    },

    Links {
        valid: bool,
    },

    TreeEdge,
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        matches!(self, Elem::Dir { uid: true } | Elem::File { uid: true, .. })
    }
    pub fn get_color(&self, theme: &theme::Theme) -> Colour {
        match self {
            Elem::File {
                exec: true,
                uid: true,
            } => theme.file_type.file.exec_uid,
            Elem::File {
                exec: false,
                uid: true,
            } => theme.file_type.file.uid_no_exec,
            Elem::File {
                exec: true,
                uid: false,
            } => theme.file_type.file.exec_no_uid,
            Elem::File {
                exec: false,
                uid: false,
            } => theme.file_type.file.no_exec_no_uid,
            Elem::SymLink => theme.file_type.symlink.default,
            Elem::BrokenSymLink => theme.file_type.symlink.broken,
            Elem::Dir { uid: true } => theme.file_type.dir.uid,
            Elem::Dir { uid: false } => theme.file_type.dir.no_uid,
            Elem::Pipe => theme.file_type.pipe,
            Elem::BlockDevice => theme.file_type.block_device,
            Elem::CharDevice => theme.file_type.char_device,
            Elem::Socket => theme.file_type.socket,
            Elem::Special => theme.file_type.special,

            Elem::Read => theme.permissions.read,
            Elem::Write => theme.permissions.write,
            Elem::Exec => theme.permissions.exec,
            Elem::ExecSticky => theme.permissions.exec_sticky,
            Elem::NoAccess => theme.permissions.no_access,

            Elem::DayOld => theme.modified.day_old,
            Elem::HourOld => theme.modified.hour_old,
            Elem::Older => theme.modified.older,

            Elem::User => theme.user,
            Elem::Group => theme.group,

            Elem::NonFile => theme.size.none,
            Elem::FileLarge => theme.size.large,
            Elem::FileMedium => theme.size.medium,
            Elem::FileSmall => theme.size.small,

            Elem::INode { valid: false } => theme.inode.valid,
            Elem::INode { valid: true } => theme.inode.invalid,
        }
    }
}

pub type ColoredString<'a> = ANSIString<'a>;

pub struct Colors {
    theme: Option<Theme>,
    lscolors: Option<LsColors>,
}

impl Colors {
    pub fn new(t: ThemeOption) -> Self {
        let theme = match t {
            ThemeOption::NoColor => None,
            ThemeOption::Default => Some(Theme::default()),
            ThemeOption::NoLscolors => Some(Theme::default()),
            ThemeOption::Custom(ref file) => {
                Some(Theme::from_path(file).unwrap_or_else(Theme::default))
            }
        };
        let lscolors = match t {
            ThemeOption::Default => Some(LsColors::from_env().unwrap_or_default()),
            _ => None,
        };

        Self { theme, lscolors }
    }

    pub fn colorize<'a>(&self, input: String, elem: &Elem) -> ColoredString<'a> {
        self.style(elem).paint(input)
    }

    pub fn colorize_using_path<'a>(
        &self,
        input: String,
        path: &Path,
        elem: &Elem,
    ) -> ColoredString<'a> {
        let style_from_path = self.style_from_path(path);
        match style_from_path {
            Some(style_from_path) => style_from_path.paint(input),
            None => self.colorize(input, elem),
        }
    }

    fn style_from_path(&self, path: &Path) -> Option<Style> {
        match &self.lscolors {
            Some(lscolors) => lscolors
                .style_for_path(path)
                .map(lscolors::Style::to_ansi_term_style),
            None => None,
        }
    }

    fn style(&self, elem: &Elem) -> Style {
        match &self.lscolors {
            Some(lscolors) => match self.get_indicator_from_elem(elem) {
                Some(style) => {
                    let style = lscolors.style_for_indicator(style);
                    style
                        .map(lscolors::Style::to_ansi_term_style)
                        .unwrap_or_default()
                }
                None => self.style_default(elem),
            },
            None => self.style_default(elem),
        }
    }

    fn style_default(&self, elem: &Elem) -> Style {
        if let Some(t) = &self.theme {
            let style_fg = Style::default().fg(elem.get_color(&t));
            if elem.has_suid() {
                style_fg.on(Colour::Fixed(124)) // Red3
            } else {
                style_fg
            }
        } else {
            Style::default()
        }
    }

    fn get_indicator_from_elem(&self, elem: &Elem) -> Option<Indicator> {
        let indicator_string = match elem {
            Elem::File { exec, uid } => match (exec, uid) {
                (_, true) => None,
                (true, false) => Some("ex"),
                (false, false) => Some("fi"),
            },
            Elem::Dir { uid } => {
                if *uid {
                    None
                } else {
                    Some("di")
                }
            }
            Elem::SymLink => Some("ln"),
            Elem::Pipe => Some("pi"),
            Elem::Socket => Some("so"),
            Elem::BlockDevice => Some("bd"),
            Elem::CharDevice => Some("cd"),
            Elem::BrokenSymLink => Some("or"),
            Elem::INode { valid } => match valid {
                true => Some("so"),
                false => Some("no"),
            },
            Elem::Links { valid } => match valid {
                true => Some("so"),
                false => Some("no"),
            },
            _ => None,
        };

        match indicator_string {
            Some(ids) => Indicator::from(ids),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Colors;
    use crate::color::Theme;
    use crate::color::ThemeOption;
    #[test]
    fn test_color_new_no_color_theme() {
        assert!(Colors::new(ThemeOption::NoColor).theme.is_none());
    }

    #[test]
    fn test_color_new_default_theme() {
        assert_eq!(
            Colors::new(ThemeOption::Default).theme,
            Some(Theme::default_dark()),
        );
    }

    #[test]
    fn test_color_new_bad_custom_theme() {
        assert_eq!(
            Colors::new(ThemeOption::Custom("not-existed".to_string())).theme,
            Some(Theme::default_dark()),
        );
    }
}

#[cfg(test)]
mod elem {
    use super::Elem;
    use crate::color::{theme, Theme};
    use ansi_term::Colour;

    #[cfg(test)]
    fn test_theme() -> Theme {
        Theme {
            user: Colour::Fixed(230),  // Cornsilk1
            group: Colour::Fixed(187), // LightYellow3
            permissions: theme::Permissions {
                read: Colour::Green,
                write: Colour::Yellow,
                exec: Colour::Red,
                exec_sticky: Colour::Purple,
                no_access: Colour::Fixed(245), // Grey
            },
            file_type: theme::FileType {
                file: theme::File {
                    exec_uid: Colour::Fixed(40),        // Green3
                    uid_no_exec: Colour::Fixed(184),    // Yellow3
                    exec_no_uid: Colour::Fixed(40),     // Green3
                    no_exec_no_uid: Colour::Fixed(184), // Yellow3
                },
                dir: theme::Dir {
                    uid: Colour::Fixed(33),    // DodgerBlue1
                    no_uid: Colour::Fixed(33), // DodgerBlue1
                },
                pipe: Colour::Fixed(44), // DarkTurquoise
                symlink: theme::Symlink {
                    default: Colour::Fixed(44), // DarkTurquoise
                    broken: Colour::Fixed(124), // Red3
                },
                block_device: Colour::Fixed(44), // DarkTurquoise
                char_device: Colour::Fixed(172), // Orange3
                socket: Colour::Fixed(44),       // DarkTurquoise
                special: Colour::Fixed(44),      // DarkTurquoise
            },
            modified: theme::Modified {
                hour_old: Colour::Fixed(40), // Green3
                day_old: Colour::Fixed(42),  // SpringGreen2
                older: Colour::Fixed(36),    // DarkCyan
            },
            size: theme::Size {
                none: Colour::Fixed(245),   // Grey
                small: Colour::Fixed(229),  // Wheat1
                medium: Colour::Fixed(216), // LightSalmon1
                large: Colour::Fixed(172),  // Orange3
            },
            inode: theme::INode {
                valid: Colour::Fixed(13),    // Pink
                invalid: Colour::Fixed(245), // Grey
            },
        }
    }

    #[test]
    fn test_default_file() {
        assert_eq!(
            Elem::File {
                exec: true,
                uid: true
            }
            .get_color(&test_theme()),
            Colour::Fixed(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: true
            }
            .get_color(&test_theme()),
            Colour::Fixed(184),
        );
        assert_eq!(
            Elem::File {
                exec: true,
                uid: false
            }
            .get_color(&test_theme()),
            Colour::Fixed(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: false
            }
            .get_color(&test_theme()),
            Colour::Fixed(184),
        );
    }
}
