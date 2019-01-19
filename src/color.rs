use ansi_term::{ANSIString, Colour, Style};
use std::collections::HashMap;

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
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        match self {
            Elem::Dir { uid: true } | Elem::File { uid: true, .. } => true,
            _ => false,
        }
    }
}

pub type ColoredString<'a> = ANSIString<'a>;

#[derive(Debug, Copy, Clone)]
pub enum Theme {
    NoColor,
    Default,
}

pub struct Colors {
    colors: Option<HashMap<Elem, Colour>>,
}

impl Colors {
    pub fn new(theme: Theme) -> Self {
        let colors = match theme {
            Theme::NoColor => None,
            Theme::Default => Some(Self::get_light_theme_colour_map()),
        };

        Self { colors }
    }

    pub fn colorize<'a>(&self, input: String, elem: &Elem) -> ColoredString<'a> {
        self.style(elem).paint(input)
    }

    fn style(&self, elem: &Elem) -> Style {
        if let Some(ref colors) = self.colors {
            let style_fg = Style::default().fg(colors[elem]);
            if elem.has_suid() {
                style_fg.on(Colour::Fixed(124)) // Red3
            } else {
                style_fg
            }
        } else {
            Style::default()
        }
    }

    // You can find the table for each color, code, and display at:
    //
    //https://jonasjacek.github.io/colors/
    fn get_light_theme_colour_map() -> HashMap<Elem, Colour> {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Colour::Fixed(230)); // Cornsilk1
        m.insert(Elem::Group, Colour::Fixed(187)); // LightYellow3

        // Permissions
        m.insert(Elem::Read, Colour::Fixed(40)); // Green3
        m.insert(Elem::Write, Colour::Fixed(192)); // DarkOliveGreen1
        m.insert(Elem::Exec, Colour::Fixed(124)); // Red3
        m.insert(Elem::ExecSticky, Colour::Fixed(13)); // Fuchsia
        m.insert(Elem::NoAccess, Colour::Fixed(168)); // HotPink3

        // File Types
        m.insert(
            Elem::File {
                exec: false,
                uid: false,
            },
            Colour::Fixed(184),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: false,
                uid: true,
            },
            Colour::Fixed(184),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: true,
                uid: false,
            },
            Colour::Fixed(40),
        ); // Green3
        m.insert(
            Elem::File {
                exec: true,
                uid: true,
            },
            Colour::Fixed(40),
        ); // Green3
        m.insert(Elem::Dir { uid: true }, Colour::Fixed(33)); // DodgerBlue1
        m.insert(Elem::Dir { uid: false }, Colour::Fixed(33)); // DodgerBlue1
        m.insert(Elem::Pipe, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::SymLink, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::BrokenSymLink, Colour::Fixed(124)); // Red3
        m.insert(Elem::BlockDevice, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::CharDevice, Colour::Fixed(172)); // Orange3
        m.insert(Elem::Socket, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::Special, Colour::Fixed(44)); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, Colour::Fixed(40)); // Green3
        m.insert(Elem::DayOld, Colour::Fixed(42)); // SpringGreen2
        m.insert(Elem::Older, Colour::Fixed(36)); // DarkCyan

        // Last Time Modified
        m.insert(Elem::NonFile, Colour::Fixed(15)); // White
        m.insert(Elem::FileSmall, Colour::Fixed(229)); // Wheat1
        m.insert(Elem::FileMedium, Colour::Fixed(216)); // LightSalmon1
        m.insert(Elem::FileLarge, Colour::Fixed(172)); // Orange3

        m
    }
}
