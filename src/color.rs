mod theme;

use crossterm::style::{Attribute, ContentStyle, StyledContent, Stylize};
use theme::Theme;

pub use crate::flags::color::ThemeOption;

use crossterm::style::Color;
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
    MissingSymLinkTarget,
    Dir {
        uid: bool,
    },
    Pipe,
    BlockDevice,
    CharDevice,
    Socket,
    Special,

    /// Permission
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

macro_rules! color_or_default {
    ($elem:expr, $color:tt, $default:expr) => {
        if let Some(color) = $elem {
            color.$color.unwrap_or($default)
        } else {
            $default
        }
    };
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        matches!(self, Elem::Dir { uid: true } | Elem::File { uid: true, .. })
    }

    pub fn get_color(&self, theme: &theme::Theme) -> Color {
        match self {
            Elem::File {
                exec: true,
                uid: true,
            } => color_or_default!(&theme.file_type.file, exec_uid, theme.default),
            Elem::File {
                exec: false,
                uid: true,
            } => color_or_default!(&theme.file_type.file, uid_no_exec, theme.default),
            Elem::File {
                exec: true,
                uid: false,
            } => color_or_default!(&theme.file_type.file, exec_no_uid, theme.default),
            Elem::File {
                exec: false,
                uid: false,
            } => color_or_default!(&theme.file_type.file, no_exec_no_uid, theme.default),
            Elem::SymLink => color_or_default!(&theme.file_type.symlink, default, theme.default),
            Elem::BrokenSymLink => {
                color_or_default!(&theme.file_type.symlink, broken, theme.default)
            }
            Elem::MissingSymLinkTarget => {
                color_or_default!(&theme.file_type.symlink, missing_target, theme.default)
            }
            Elem::Dir { uid: true } => color_or_default!(&theme.file_type.dir, uid, theme.default),
            Elem::Dir { uid: false } => {
                color_or_default!(&theme.file_type.dir, no_uid, theme.default)
            }
            Elem::Pipe => theme.file_type.pipe.unwrap_or(theme.default),
            Elem::BlockDevice => theme.file_type.block_device.unwrap_or(theme.default),
            Elem::CharDevice => theme.file_type.char_device.unwrap_or(theme.default),
            Elem::Socket => theme.file_type.socket.unwrap_or(theme.default),
            Elem::Special => theme.file_type.special.unwrap_or(theme.default),

            Elem::Read => color_or_default!(&theme.permission, read, theme.default),
            Elem::Write => color_or_default!(&theme.permission, write, theme.default),
            Elem::Exec => color_or_default!(&theme.permission, exec, theme.default),
            Elem::ExecSticky => color_or_default!(&theme.permission, exec_sticky, theme.default),
            Elem::NoAccess => color_or_default!(&theme.permission, no_access, theme.default),

            Elem::DayOld => color_or_default!(&theme.date, day_old, theme.default),
            Elem::HourOld => color_or_default!(&theme.date, hour_old, theme.default),
            Elem::Older => color_or_default!(&theme.date, older, theme.default),

            Elem::User => theme.user.unwrap_or(theme.default),
            Elem::Group => theme.group.unwrap_or(theme.default),

            Elem::NonFile => color_or_default!(&theme.size, none, theme.default),
            Elem::FileLarge => color_or_default!(&theme.size, large, theme.default),
            Elem::FileMedium => color_or_default!(&theme.size, medium, theme.default),
            Elem::FileSmall => color_or_default!(&theme.size, small, theme.default),

            Elem::INode { valid: false } => color_or_default!(&theme.inode, valid, theme.default),
            Elem::INode { valid: true } => color_or_default!(&theme.inode, invalid, theme.default),

            Elem::TreeEdge => theme.tree_edge.unwrap_or(theme.default),
            Elem::Links { valid: false } => color_or_default!(&theme.links, invalid, theme.default),
            Elem::Links { valid: true } => color_or_default!(&theme.links, valid, theme.default),
        }
    }
}

pub type ColoredString = StyledContent<String>;

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
            ThemeOption::Custom(ref file) => Some(Theme::from_path(file).unwrap_or_default()),
        };
        let lscolors = match t {
            ThemeOption::Default => Some(LsColors::from_env().unwrap_or_default()),
            ThemeOption::Custom(_) => Some(LsColors::from_env().unwrap_or_default()),
            _ => None,
        };

        Self { theme, lscolors }
    }

    pub fn colorize(&self, input: String, elem: &Elem) -> ColoredString {
        self.style(elem).apply(input)
    }

    pub fn colorize_using_path(&self, input: String, path: &Path, elem: &Elem) -> ColoredString {
        let style_from_path = self.style_from_path(path);
        match style_from_path {
            Some(style_from_path) => style_from_path.apply(input),
            None => self.colorize(input, elem),
        }
    }

    pub fn default_style() -> ContentStyle {
        ContentStyle::default()
    }

    fn style_from_path(&self, path: &Path) -> Option<ContentStyle> {
        match &self.lscolors {
            Some(lscolors) => lscolors.style_for_path(path).map(to_content_style),
            None => None,
        }
    }

    fn style(&self, elem: &Elem) -> ContentStyle {
        match &self.lscolors {
            Some(lscolors) => match self.get_indicator_from_elem(elem) {
                Some(style) => {
                    let style = lscolors.style_for_indicator(style);
                    style.map(to_content_style).unwrap_or_default()
                }
                None => self.style_default(elem),
            },
            None => self.style_default(elem),
        }
    }

    fn style_default(&self, elem: &Elem) -> ContentStyle {
        if let Some(t) = &self.theme {
            let style_fg = ContentStyle::default().with(elem.get_color(&t));
            if elem.has_suid() {
                style_fg.on(Color::AnsiValue(124)) // Red3
            } else {
                style_fg
            }
        } else {
            ContentStyle::default()
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
            Elem::MissingSymLinkTarget => Some("mi"),
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

fn to_content_style(ls: &lscolors::Style) -> ContentStyle {
    let to_crossterm_color = |c: &lscolors::Color| match c {
        lscolors::style::Color::RGB(r, g, b) => Color::Rgb {
            r: *r,
            g: *g,
            b: *b,
        },
        lscolors::style::Color::Fixed(n) => Color::AnsiValue(*n),
        lscolors::style::Color::Black => Color::Black,
        lscolors::style::Color::Red => Color::DarkRed,
        lscolors::style::Color::Green => Color::DarkGreen,
        lscolors::style::Color::Yellow => Color::DarkYellow,
        lscolors::style::Color::Blue => Color::DarkBlue,
        lscolors::style::Color::Magenta => Color::DarkMagenta,
        lscolors::style::Color::Cyan => Color::DarkCyan,
        lscolors::style::Color::White => Color::White,
    };
    let mut style = ContentStyle::default();

    style.foreground_color = ls.foreground.as_ref().map(to_crossterm_color);
    style.background_color = ls.background.as_ref().map(to_crossterm_color);

    if ls.font_style.bold {
        style.attributes.set(Attribute::Bold);
    }
    if ls.font_style.dimmed {
        style.attributes.set(Attribute::Dim);
    }
    if ls.font_style.italic {
        style.attributes.set(Attribute::Italic);
    }
    if ls.font_style.underline {
        style.attributes.set(Attribute::Underlined);
    }
    if ls.font_style.rapid_blink {
        style.attributes.set(Attribute::RapidBlink);
    }
    if ls.font_style.slow_blink {
        style.attributes.set(Attribute::SlowBlink);
    }
    if ls.font_style.reverse {
        style.attributes.set(Attribute::Reverse);
    }
    if ls.font_style.hidden {
        style.attributes.set(Attribute::Hidden);
    }
    if ls.font_style.strikethrough {
        style.attributes.set(Attribute::CrossedOut);
    }

    style
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
    use crossterm::style::Color;

    #[cfg(test)]
    fn test_theme() -> Theme {
        Theme {
            default: Color::AnsiValue(245),
            user: Some(Color::AnsiValue(230)),  // Cornsilk1
            group: Some(Color::AnsiValue(187)), // LightYellow3
            permission: Some(theme::Permission {
                read: Some(Color::Green),
                write: Some(Color::Yellow),
                exec: Some(Color::Red),
                exec_sticky: Some(Color::Magenta),
                no_access: Some(Color::AnsiValue(245)), // Grey
            }),
            file_type: theme::FileType {
                file: Some(theme::File {
                    exec_uid: Some(Color::AnsiValue(40)),        // Green3
                    uid_no_exec: Some(Color::AnsiValue(184)),    // Yellow3
                    exec_no_uid: Some(Color::AnsiValue(40)),     // Green3
                    no_exec_no_uid: Some(Color::AnsiValue(184)), // Yellow3
                }),
                dir: Some(theme::Dir {
                    uid: Some(Color::AnsiValue(33)),    // DodgerBlue1
                    no_uid: Some(Color::AnsiValue(33)), // DodgerBlue1
                }),
                pipe: Some(Color::AnsiValue(44)), // DarkTurquoise
                symlink: Some(theme::Symlink {
                    default: Some(Color::AnsiValue(44)),         // DarkTurquoise
                    broken: Some(Color::AnsiValue(124)),         // Red3
                    missing_target: Some(Color::AnsiValue(124)), // Red3
                }),
                block_device: Some(Color::AnsiValue(44)), // DarkTurquoise
                char_device: Some(Color::AnsiValue(172)), // Orange3
                socket: Some(Color::AnsiValue(44)),       // DarkTurquoise
                special: Some(Color::AnsiValue(44)),      // DarkTurquoise
            },
            date: Some(theme::Date {
                hour_old: Some(Color::AnsiValue(40)), // Green3
                day_old: Some(Color::AnsiValue(42)),  // SpringGreen2
                older: Some(Color::AnsiValue(36)),    // DarkCyan
            }),
            size: Some(theme::Size {
                none: Some(Color::AnsiValue(245)),   // Grey
                small: Some(Color::AnsiValue(229)),  // Wheat1
                medium: Some(Color::AnsiValue(216)), // LightSalmon1
                large: Some(Color::AnsiValue(172)),  // Orange3
            }),
            inode: Some(theme::INode {
                valid: Some(Color::AnsiValue(13)),    // Pink
                invalid: Some(Color::AnsiValue(245)), // Grey
            }),
            links: Some(theme::Links {
                valid: Some(Color::AnsiValue(13)),    // Pink
                invalid: Some(Color::AnsiValue(245)), // Grey
            }),
            tree_edge: Some(Color::AnsiValue(245)), // Grey
        }
    }

    fn none_theme() -> Theme {
        Theme {
            default: Color::Green,
            file_type: theme::FileType {
                file: None,
                dir: None,
                pipe: None,
                symlink: None,
                block_device: None,
                char_device: None,
                socket: None,
                special: None,
            },
            group: None,
            user: None,
            permission: None,
            date: None,
            size: None,
            inode: None,
            links: None,
            tree_edge: None,
        }
    }

    #[test]
    fn test_default_theme_color() {
        assert_eq!(
            Elem::File {
                exec: true,
                uid: true
            }
            .get_color(&test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: true
            }
            .get_color(&test_theme()),
            Color::AnsiValue(184),
        );
        assert_eq!(
            Elem::File {
                exec: true,
                uid: false
            }
            .get_color(&test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: false
            }
            .get_color(&test_theme()),
            Color::AnsiValue(184),
        );
    }

    #[test]
    fn test_default_theme_default() {
        assert_eq!(
            Elem::User.get_color(&none_theme()),
            none_theme().default,
        );
        assert_eq!(
            Elem::Group.get_color(&none_theme()),
            none_theme().default,
        );
        assert_eq!(
            Elem::INode {
                valid: false
            }
            .get_color(&none_theme()),
            none_theme().default,
        );
        assert_eq!(
            Elem::Links {
                valid: true
            }
            .get_color(&none_theme()),
            none_theme().default,
        );
    }
}
