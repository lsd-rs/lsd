use ansi_term::Colour;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Elem {
    /// Node type
    File,
    UnrecognizedFile,
    RecognizedFile,
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

    /// Link
    DeadLink,
    Link,

    /// User / Group Name
    User,
    Group,

    /// File Size
    FileLarge,
    FileMedium,
    FileSmall,
}

lazy_static! {
    pub static ref Colors: HashMap<Elem, Colour> = {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Colour::RGB(0xFF, 0xFF, 0xD8));
        m.insert(Elem::Group, Colour::RGB(0xD9, 0xD9, 0x8F));

        // Permissions
        m.insert(Elem::Read, Colour::RGB(0x5f, 0xD7, 0x5F));
        m.insert(Elem::Write, Colour::RGB(0xD7, 0xD7, 0x87));
        m.insert(Elem::Exec, Colour::RGB(0xCD, 0x3A, 0x3A));
        m.insert(Elem::NoAccess, Colour::RGB(0xD7, 0x89, 0x89));

        // Path Kind
        m.insert(Elem::UnrecognizedFile, Colour::RGB(0xFF, 0xFF, 0x04));
        m.insert(Elem::RecognizedFile, Colour::RGB(0x04, 0xFF, 0x04));
        m.insert(Elem::Dir, Colour::RGB(0x00, 0xAF, 0xFF));
        m.insert(Elem::SymLink, Colour::RGB(0xFF, 0x00, 0x00));

        // Last Time Modified
        m.insert(Elem::HourOld, Colour::RGB(0x2C, 0xFF, 0x2C));
        m.insert(Elem::DayOld, Colour::RGB(0x1C, 0xFF, 0xB7));
        m.insert(Elem::Older, Colour::RGB(0x63, 0xB1, 0x8A));

        // Last Time Modified
        m.insert(Elem::FileSmall, Colour::RGB(0xFF, 0xFF, 0xD9));
        m.insert(Elem::FileMedium, Colour::RGB(0x1C, 0xFF, 0xB7));
        m.insert(Elem::FileLarge, Colour::RGB(0xFF, 0xB0, 0x00));

        // Link
        m.insert(Elem::Link, Colour::RGB(0x3B, 0xCE, 0xCE));

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
        m.insert(Elem::File , Colors[&Elem::UnrecognizedFile].paint(String::from(".")).to_string());
        m.insert(Elem::Dir, Colors[&Elem::Dir].paint(String::from("d")).to_string());
        m.insert(Elem::SymLink, Colors[&Elem::SymLink].paint(String::from("l")).to_string());

        m
    };
}
