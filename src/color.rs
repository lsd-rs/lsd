use ansi_term::{ANSIString, Colour, Style};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Elem {
    /// Node type
    File,
    SymLink,
    BrokenSymLink,
    Dir,
    ExecutableFile,
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

    /// Misc
    ExecutableFile,
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
            Theme::Default => Some(Colors::get_light_theme_colour_map()),
        };

        Colors { colors }
    }

    pub fn colorize<'a>(&self, input: String, elem: &Elem) -> ColoredString<'a> {
        if let Some(ref colors) = self.colors {
            colors[elem].paint(input)
        } else {
            Style::default().paint(input)
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
        m.insert(Elem::ExecSticky, Colour::Fixed(13)); // Fuschsia
        m.insert(Elem::NoAccess, Colour::Fixed(168)); // HotPink3

        // File Types
        m.insert(Elem::File, Colour::Fixed(184)); // Yellow3
        m.insert(Elem::Dir, Colour::Fixed(33)); // DodgerBlue1
        m.insert(Elem::ExecutableFile, Colour::Fixed(40)); // Green3
        m.insert(Elem::Pipe, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::SymLink, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::BrokenSymLink, Colour::Fixed(124)); // Red3
        m.insert(Elem::BlockDevice, Colour::Fixed(44)); // DarkTurquoise
        m.insert(Elem::CharDevice, Colour::Fixed(44)); // DarkTurquoise
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
        m.insert(Elem::ExecutableFile, Colour::Fixed(47)); // Green

        m
    }
}
