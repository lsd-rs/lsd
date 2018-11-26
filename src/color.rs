use ansi_term::Colour;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Elem {
    /// Node type
    File,
    SymLink,
    Dir,

    /// Permissions
    Read,
    Write,
    Exec,
    NoAccess,

    /// Last Time Modified
    DayOld,
    HourOld,
    Older,

    /// User / Group Name
    User,
    Group,

    /// File Size
    FileLarge,
    FileMedium,
    FileSmall,
}

// You can find the table for each color, code, and display at:
//
//https://jonasjacek.github.io/colors/
lazy_static! {
    pub static ref Colors: HashMap<Elem, Colour> = {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Colour::Fixed(230)); // Cornsilk1
        m.insert(Elem::Group, Colour::Fixed(187)); // LightYellow3

        // Permissions
        m.insert(Elem::Read, Colour::Fixed(40)); // Green3
        m.insert(Elem::Write, Colour::Fixed(192)); // DarkOliveGreen1
        m.insert(Elem::Exec, Colour::Fixed(124)); // Red3
        m.insert(Elem::NoAccess, Colour::Fixed(168)); // HotPink3

        // Path Kind
        m.insert(Elem::File , Colour::Fixed(184)); // Yellow3
        m.insert(Elem::Dir, Colour::Fixed(33)); // DodgerBlue1
        m.insert(Elem::SymLink, Colour::Fixed(44)); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, Colour::Fixed(40)); // Green3
        m.insert(Elem::DayOld, Colour::Fixed(42)); // SpringGreen2
        m.insert(Elem::Older, Colour::Fixed(36)); // DarkCyan

        // Last Time Modified
        m.insert(Elem::FileSmall, Colour::Fixed(229)); // Wheat1
        m.insert(Elem::FileMedium, Colour::Fixed(216)); // LightSalmon1
        m.insert(Elem::FileLarge, Colour::Fixed(172)); // Orange3

        m
    };
}

lazy_static! {
    pub static ref PrecomputedElems : HashMap<Elem, String> = {
        let mut m = HashMap::new();

        // Permissions
        m.insert(Elem::Read, Colors[&Elem::Read].paint(String::from("r")).to_string());
        m.insert(Elem::Write, Colors[&Elem::Write].paint(String::from("w")).to_string());
        m.insert(Elem::Exec, Colors[&Elem::Exec].paint(String::from("x")).to_string());
        m.insert(Elem::NoAccess, Colors[&Elem::NoAccess].paint(String::from("-")).to_string());

        // Note types
        m.insert(Elem::File , Colors[&Elem::File].paint(String::from(".")).to_string());
        m.insert(Elem::Dir, Colors[&Elem::Dir].paint(String::from("d")).to_string());
        m.insert(Elem::SymLink, Colors[&Elem::SymLink].paint(String::from("l")).to_string());

        m
    };
}
