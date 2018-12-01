use color::{Elem, PrecomputedElems};
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
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
    fn from(meta: &Metadata) -> Self {
        let bits = meta.permissions().mode();
        let has_bit = |bit| bits & bit == bit;

        Permissions {
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
}

impl Permissions {
    pub fn render(&self) -> String {
        let mut res = String::with_capacity(11);

        res += &self.render_permission_set(self.user_read, self.user_write, self.user_execute);
        res += &self.render_permission_set(self.group_read, self.group_write, self.group_execute);
        res += &self.render_permission_set(self.other_read, self.other_write, self.other_execute);

        res
    }

    fn render_permission_set(&self, read: bool, write: bool, exec: bool) -> String {
        let mut res = String::with_capacity(3);

        // Read Permisssions
        if read {
            res += PrecomputedElems[&Elem::Read].as_str();
        } else {
            res += PrecomputedElems[&Elem::NoAccess].as_str();
        }

        // Write Permisssions
        if write {
            res += PrecomputedElems[&Elem::Write].as_str();
        } else {
            res += PrecomputedElems[&Elem::NoAccess].as_str();
        }

        // Exec Permisssions
        if exec {
            res += PrecomputedElems[&Elem::Exec].as_str();
        } else {
            res += PrecomputedElems[&Elem::NoAccess].as_str();
        }

        res
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
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
