use ansi_term::Colour;
use path_lister::Path;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::Timespec;

const HOURE: u64 = 3600; // 1 HOURE == 3600 seconds
const DAY: u64 = HOURE * 24; // 1 DAY == 25 HOURE

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum PathKind {
    UnrecognizedFile,
    RecognizedFile,
    Dir,
}

lazy_static! {
    pub static ref PathKindColor: HashMap<PathKind, (u8, u8, u8)> = {
        let mut m = HashMap::new();

        m.insert(PathKind::UnrecognizedFile, (0xFF, 0xFF, 0x04)); // gold
        m.insert(PathKind::RecognizedFile, (0x04, 0xFF, 0x04)); // limon
        m.insert(PathKind::Dir, (0x00, 0xAF, 0xFF)); // dodgerblue

        m
    };
}

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum LastTimeModified {
    DayOld,
    HourOld,
    Older,
}

lazy_static! {
    pub static ref LastTimeModifiedColor: HashMap<LastTimeModified, (u8, u8, u8)> = {
        let mut m = HashMap::new();

        m.insert(LastTimeModified::HourOld, (0x2C, 0xFF, 0x2C));
        m.insert(LastTimeModified::DayOld, (0x1C, 0xFF, 0xB7));
        m.insert(LastTimeModified::Older, (0x63, 0xB1, 0x8A));

        m
    };
}

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Elem {
    /// Link
    DeadLink,
    Link,

    /// File Size
    FileLarge,
    FileMedium,
    FileSmall,

    /// Random
    Report,
    User,
    Tree,
    Empty,
    Error,
    Normal,

    /// Git
    Addition,
    Modification,
    Deletion,
    Untracked,
    Unchanged,
}

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {}
    }

    pub fn format_path(&self, content: &str, path: &Path) -> String {
        let color = match path.metadata.is_dir() {
            true => PathKindColor[&PathKind::Dir],
            false => PathKindColor[&PathKind::UnrecognizedFile],
        };

        Colour::RGB(color.0, color.1, color.2)
            .paint(content)
            .to_string()
    }

    pub fn format_date(&self, path: &Path) -> String {
        let modified_time = path.metadata.modified().unwrap();

        let now = SystemTime::now();

        let color;
        if modified_time > now - Duration::new(HOURE, 0) {
            color = LastTimeModifiedColor[&LastTimeModified::HourOld];
        } else if modified_time > now - Duration::new(DAY, 0) {
            color = LastTimeModifiedColor[&LastTimeModified::DayOld];
        } else {
            color = LastTimeModifiedColor[&LastTimeModified::Older];
        }

        let modified_time_since_epoch = modified_time.duration_since(UNIX_EPOCH).unwrap();
        let time = time::at(Timespec::new(
            modified_time_since_epoch.as_secs() as i64,
            modified_time_since_epoch.subsec_nanos() as i32,
        ));

        Colour::RGB(color.0, color.1, color.2)
            .paint(time.ctime().to_string())
            .to_string()
    }

    pub fn format_permissions(&self, path: &Path) -> String {
        let mut res = String::with_capacity(10);

        let mode = path.metadata.permissions().mode();

        let read_perm = Colour::RGB(0x5f, 0xD7, 0x5F)
            .paint(String::from("r"))
            .to_string();
        let write_perm = Colour::RGB(0xD7, 0xD7, 0x87)
            .paint(String::from("w"))
            .to_string();
        let exec_perm = Colour::RGB(0xCD, 0x3A, 0x3A)
            .paint(String::from("x"))
            .to_string();
        let no_access = Colour::RGB(0xD7, 0x89, 0x89)
            .paint(String::from("-"))
            .to_string();

        // User Read Permisssions
        match mode & 0o400 {
            0 => res = res + no_access.as_str(),
            _ => res = res + read_perm.as_str(),
        }

        // User Write Permisssions
        match mode & 0o200 {
            0 => res = res + no_access.as_str(),
            _ => res = res + write_perm.as_str(),
        }

        // User Exec Permisssions
        match mode & 0o100 {
            0 => res = res + no_access.as_str(),
            _ => res = res + exec_perm.as_str(),
        }

        // Group Read Permisssions
        match mode & 0o040 {
            0 => res = res + no_access.as_str(),
            _ => res = res + read_perm.as_str(),
        }

        // Group Write Permisssions
        match mode & 0o020 {
            0 => res = res + no_access.as_str(),
            _ => res = res + write_perm.as_str(),
        }

        // Group Exec Permisssions
        match mode & 0o010 {
            0 => res = res + no_access.as_str(),
            _ => res = res + exec_perm.as_str(),
        }

        // Other Read Permisssions
        match mode & 0o040 {
            0 => res = res + no_access.as_str(),
            _ => res = res + read_perm.as_str(),
        }

        // Other Write Permisssions
        match mode & 0o020 {
            0 => res = res + no_access.as_str(),
            _ => res = res + write_perm.as_str(),
        }

        // Other Exec Permisssions
        match mode & 0o010 {
            0 => res = res + no_access.as_str(),
            _ => res = res + exec_perm.as_str(),
        }

        res.to_string()
    }
}
