use crate::color::{ColoredString, Colors, Elem};
use ansi_term::ANSIStrings;
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Permissions {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,

    pub sticky: bool,
    pub setgid: bool,
    pub setuid: bool,
}

impl<'a> From<&'a Metadata> for Permissions {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let bits = meta.permissions().mode();
        let has_bit = |bit| bits & bit == bit;

        Self {
            user_read: has_bit(modes::USER_READ),
            user_write: has_bit(modes::USER_WRITE),
            user_execute: has_bit(modes::USER_EXECUTE),

            group_read: has_bit(modes::GROUP_READ),
            group_write: has_bit(modes::GROUP_WRITE),
            group_execute: has_bit(modes::GROUP_EXECUTE),

            other_read: has_bit(modes::OTHER_READ),
            other_write: has_bit(modes::OTHER_WRITE),
            other_execute: has_bit(modes::OTHER_EXECUTE),

            sticky: has_bit(modes::STICKY),
            setgid: has_bit(modes::SETGID),
            setuid: has_bit(modes::SETUID),
        }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        panic!("Cannot get permissions from metadata on Windows")
    }
}

impl Permissions {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        let bit = |bit, chr: &'static str, elem: &Elem| {
            if bit {
                colors.colorize(String::from(chr), elem)
            } else {
                colors.colorize(String::from("-"), &Elem::NoAccess)
            }
        };

        let strings: &[ColoredString] = &[
            // User permissions
            bit(self.user_read, "r", &Elem::Read),
            bit(self.user_write, "w", &Elem::Write),
            match (self.user_execute, self.setuid) {
                (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                (false, true) => colors.colorize(String::from("S"), &Elem::ExecSticky),
                (true, true) => colors.colorize(String::from("s"), &Elem::ExecSticky),
            },
            // Group permissions
            bit(self.group_read, "r", &Elem::Read),
            bit(self.group_write, "w", &Elem::Write),
            match (self.group_execute, self.setuid) {
                (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                (false, true) => colors.colorize(String::from("S"), &Elem::ExecSticky),
                (true, true) => colors.colorize(String::from("s"), &Elem::ExecSticky),
            },
            // Other permissions
            bit(self.other_read, "r", &Elem::Read),
            bit(self.other_write, "w", &Elem::Write),
            match (self.other_execute, self.sticky) {
                (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                (false, true) => colors.colorize(String::from("T"), &Elem::ExecSticky),
                (true, true) => colors.colorize(String::from("t"), &Elem::ExecSticky),
            },
        ];

        let res = ANSIStrings(strings).to_string();
        ColoredString::from(res)
    }

    pub fn is_executable(&self) -> bool {
        self.user_execute || self.group_execute || self.other_execute
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
#[cfg(unix)]
mod modes {
    use libc;

    pub type Mode = u32;
    // The `libc::mode_t` type’s actual type varies, but the value returned
    // from `metadata.permissions().mode()` is always `u32`.

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const USER_WRITE: Mode = libc::S_IWUSR as Mode;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;

    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const GROUP_WRITE: Mode = libc::S_IWGRP as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;

    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
    pub const OTHER_WRITE: Mode = libc::S_IWOTH as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const STICKY: Mode = libc::S_ISVTX as Mode;
    pub const SETGID: Mode = libc::S_ISGID as Mode;
    pub const SETUID: Mode = libc::S_ISUID as Mode;
}
