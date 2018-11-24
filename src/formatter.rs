use ansi_term::Colour;
use logo::Logo;
use meta::Meta;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::Timespec;

const HOURE: u64 = 3600; // 1 HOURE == 3600 seconds
const DAY: u64 = HOURE * 24; // 1 DAY == 25 HOURE

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Elem {
    /// Path Kind
    UnrecognizedFile,
    RecognizedFile,
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
        m.insert(Elem::UnrecognizedFile, Colour::RGB(0xFF, 0xFF, 0x04)); // gold
        m.insert(Elem::RecognizedFile, Colour::RGB(0x04, 0xFF, 0x04)); // limon
        m.insert(Elem::Dir, Colour::RGB(0x00, 0xAF, 0xFF)); // dodgerblue

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

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {}
    }

    pub fn format_name(&self, meta: &Meta) -> String {
        let mut content = String::new();

        let color = if meta.metadata.is_dir() {
            content = content + Logo::folder().as_str() + " " + &meta.name;
            Colors[&Elem::Dir]
        } else {
            content = content + Logo::from_pathbuf(&meta.path).as_str() + "  " + &meta.name;
            Colors[&Elem::UnrecognizedFile]
        };

        content = color.paint(content).to_string();

        let color = Colors[&Elem::Link];
        if let Some(ref link) = meta.symlink_target {
            content =
                content + &color.paint(String::from(" ⇒ ") + &color.paint(link).to_string());
        }

        content
    }

    pub fn format_date(&self, meta: &Meta) -> String {
        let modified_time = meta
            .metadata
            .modified()
            .expect("failed to retrieve modified date");

        let now = SystemTime::now();

        let color;
        if modified_time > now - Duration::new(HOURE, 0) {
            color = Colors[&Elem::HourOld];
        } else if modified_time > now - Duration::new(DAY, 0) {
            color = Colors[&Elem::DayOld];
        } else {
            color = Colors[&Elem::Older];
        }

        let modified_time_since_epoch = modified_time
            .duration_since(UNIX_EPOCH)
            .expect("failed to convert modified time to timestamp");
        let time = time::at(Timespec::new(
            modified_time_since_epoch.as_secs() as i64,
            modified_time_since_epoch.subsec_nanos() as i32,
        ));

        color.paint(time.ctime().to_string()).to_string()
    }

    pub fn format_permissions(&self, meta: &Meta) -> String {
        let mut res = String::with_capacity(10);

        let mode = meta.metadata.permissions().mode();

        let read_perm = Colors[&Elem::Read].paint(String::from("r")).to_string();
        let write_perm = Colors[&Elem::Write].paint(String::from("w")).to_string();
        let exec_perm = Colors[&Elem::Exec].paint(String::from("x")).to_string();
        let no_access = Colors[&Elem::NoAccess].paint(String::from("-")).to_string();

        // User Read Permisssions
        match mode & 0o400 {
            0 => res += no_access.as_str(),
            _ => res += read_perm.as_str(),
        }

        // User Write Permisssions
        match mode & 0o200 {
            0 => res += no_access.as_str(),
            _ => res += write_perm.as_str(),
        }

        // User Exec Permisssions
        match mode & 0o100 {
            0 => res += no_access.as_str(),
            _ => res += exec_perm.as_str(),
        }

        // Group Read Permisssions
        match mode & 0o040 {
            0 => res += no_access.as_str(),
            _ => res += read_perm.as_str(),
        }

        // Group Write Permisssions
        match mode & 0o020 {
            0 => res += no_access.as_str(),
            _ => res += write_perm.as_str(),
        }

        // Group Exec Permisssions
        match mode & 0o010 {
            0 => res += no_access.as_str(),
            _ => res += exec_perm.as_str(),
        }

        // Other Read Permisssions
        match mode & 0o040 {
            0 => res += no_access.as_str(),
            _ => res += read_perm.as_str(),
        }

        // Other Write Permisssions
        match mode & 0o020 {
            0 => res += no_access.as_str(),
            _ => res += write_perm.as_str(),
        }

        // Other Exec Permisssions
        match mode & 0o010 {
            0 => res += no_access.as_str(),
            _ => res += exec_perm.as_str(),
        }

        res.to_string()
    }

    pub fn format_user(&self, user_name: &str, max_user_size: usize) -> String {
        if user_name.len() == max_user_size {
            return Colors[&Elem::User].paint(user_name).to_string();
        }

        let mut content = String::with_capacity(max_user_size);

        content += user_name;

        for _ in 0..(max_user_size - user_name.len()) {
            content.push(' ');
        }

        content
    }

    pub fn format_group(&self, group_name: &str, max_group_size: usize) -> String {
        if group_name.len() == max_group_size {
            return Colors[&Elem::Group].paint(group_name).to_string();
        }

        let mut content = String::with_capacity(max_group_size);
        content += group_name;

        for _ in 0..(max_group_size - group_name.len()) {
            content.push(' ');
        }

        content
    }

    pub fn format_size(
        &self,
        meta: &Meta,
        max_value_length: usize,
        max_unit_size: usize,
    ) -> String {
        let mut content = String::with_capacity(max_value_length + max_unit_size + 1);

        for _ in 0..(max_value_length - meta.size_value.len()) {
            content.push(' ');
        }

        content += meta.size_value.as_str();
        content.push(' ');
        content += meta.size_unit.as_str();

        for _ in 0..(max_unit_size - meta.size_unit.len()) {
            content.push(' ');
        }

        if meta.metadata.len() < 10 * 1044 * 1024 {
            Colors[&Elem::FileSmall].paint(content).to_string()
        } else if meta.metadata.len() < 100 * 1044 * 1024 {
            Colors[&Elem::FileMedium].paint(content).to_string()
        } else {
            Colors[&Elem::FileLarge].paint(content).to_string()
        }
    }
}
