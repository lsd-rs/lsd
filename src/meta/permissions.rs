use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, PermissionFlag};
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

impl From<&Metadata> for Permissions {
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
    fn bits_to_octal(r: bool, w: bool, x: bool) -> u8 {
        (r as u8) * 4 + (w as u8) * 2 + (x as u8)
    }

    pub fn render(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let bit = |bit, chr: &'static str, elem: &Elem| {
            if bit {
                colors.colorize(chr, elem)
            } else {
                colors.colorize('-', &Elem::NoAccess)
            }
        };

        let res = match flags.permission {
            PermissionFlag::Rwx => [
                // User permissions
                bit(self.user_read, "r", &Elem::Read),
                bit(self.user_write, "w", &Elem::Write),
                match (self.user_execute, self.setuid) {
                    (false, false) => colors.colorize('-', &Elem::NoAccess),
                    (true, false) => colors.colorize('x', &Elem::Exec),
                    (false, true) => colors.colorize('S', &Elem::ExecSticky),
                    (true, true) => colors.colorize('s', &Elem::ExecSticky),
                },
                // Group permissions
                bit(self.group_read, "r", &Elem::Read),
                bit(self.group_write, "w", &Elem::Write),
                match (self.group_execute, self.setgid) {
                    (false, false) => colors.colorize('-', &Elem::NoAccess),
                    (true, false) => colors.colorize('x', &Elem::Exec),
                    (false, true) => colors.colorize('S', &Elem::ExecSticky),
                    (true, true) => colors.colorize('s', &Elem::ExecSticky),
                },
                // Other permissions
                bit(self.other_read, "r", &Elem::Read),
                bit(self.other_write, "w", &Elem::Write),
                match (self.other_execute, self.sticky) {
                    (false, false) => colors.colorize('-', &Elem::NoAccess),
                    (true, false) => colors.colorize('x', &Elem::Exec),
                    (false, true) => colors.colorize('T', &Elem::ExecSticky),
                    (true, true) => colors.colorize('t', &Elem::ExecSticky),
                },
            ]
            .into_iter()
            // From the experiment, the maximum string size is 153 bytes
            .fold(String::with_capacity(160), |mut acc, x| {
                acc.push_str(&x.to_string());
                acc
            }),
            PermissionFlag::Octal => {
                let octals = [
                    Self::bits_to_octal(self.setuid, self.setgid, self.sticky),
                    Self::bits_to_octal(self.user_read, self.user_write, self.user_execute),
                    Self::bits_to_octal(self.group_read, self.group_write, self.group_execute),
                    Self::bits_to_octal(self.other_read, self.other_write, self.other_execute),
                ]
                .into_iter()
                .fold(String::with_capacity(4), |mut acc, x| {
                    acc.push(
                        char::from_digit(x as u32, 8)
                            .expect("octal value of permission should not be greater than 7"),
                    );
                    acc
                });

                colors.colorize(octals, &Elem::Octal).to_string()
            }
        };

        ColoredString::new(Colors::default_style(), res)
    }

    pub fn is_executable(&self) -> bool {
        self.user_execute || self.group_execute || self.other_execute
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
#[cfg(unix)]
mod modes {
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

#[cfg(unix)]
#[cfg(test)]
mod test {
    use super::Flags;
    use super::{PermissionFlag, Permissions};
    use crate::color::{Colors, ThemeOption};
    use std::fs;
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    #[test]
    fn permission_rwx() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o655))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let perms = Permissions::from(&meta);

        assert_eq!(
            "rw-r-xr-x",
            perms.render(&colors, &Flags::default()).content()
        );
    }

    #[test]
    fn permission_rwx2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o777))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let perms = Permissions::from(&meta);

        assert_eq!(
            "rwxrwxrwx",
            perms.render(&colors, &Flags::default()).content()
        );
    }

    #[test]
    fn permission_rwx_sticky() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o1777))
            .expect("unable to set permissions to file");

        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let flags = Flags {
            permission: PermissionFlag::Rwx,
            ..Default::default()
        };
        let perms = Permissions::from(&meta);

        assert_eq!("rwxrwxrwt", perms.render(&colors, &flags).content());
    }

    #[test]
    fn permission_octal() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o655))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let flags = Flags {
            permission: PermissionFlag::Octal,
            ..Default::default()
        };
        let perms = Permissions::from(&meta);

        assert_eq!("0655", perms.render(&colors, &flags).content());
    }

    #[test]
    fn permission_octal2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o777))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let flags = Flags {
            permission: PermissionFlag::Octal,
            ..Default::default()
        };
        let perms = Permissions::from(&meta);

        assert_eq!("0777", perms.render(&colors, &flags).content());
    }

    #[test]
    fn permission_octal_sticky() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o1777))
            .expect("unable to set permissions to file");

        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let flags = Flags {
            permission: PermissionFlag::Octal,
            ..Default::default()
        };
        let perms = Permissions::from(&meta);

        assert_eq!("1777", perms.render(&colors, &flags).content());
    }
}
