use crate::color::{ColoredString, Colors, Elem};
use crate::meta::Permissions;
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(windows, allow(dead_code))]
pub enum FileType {
    BlockDevice,
    CharDevice,
    Directory { uid: bool },
    File { uid: bool, exec: bool },
    SymLink,
    Pipe,
    Socket,
    Special,
}

impl FileType {
    #[cfg(unix)]
    pub fn new(meta: &Metadata, permissions: &Permissions) -> Self {
        use std::os::unix::fs::FileTypeExt;

        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File {
                exec: permissions.is_executable(),
                uid: permissions.setuid,
            }
        } else if file_type.is_dir() {
            FileType::Directory {
                uid: permissions.setuid,
            }
        } else if file_type.is_fifo() {
            FileType::Pipe
        } else if file_type.is_symlink() {
            FileType::SymLink
        } else if file_type.is_char_device() {
            FileType::CharDevice
        } else if file_type.is_block_device() {
            FileType::BlockDevice
        } else if file_type.is_socket() {
            FileType::Socket
        } else {
            FileType::Special
        }
    }

    #[cfg(windows)]
    pub fn new(meta: &Metadata, permissions: &Permissions) -> Self {
        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File {
                exec: permissions.is_executable(),
                uid: permissions.setuid,
            }
        } else if file_type.is_dir() {
            FileType::Directory {
                uid: permissions.setuid,
            }
        } else if file_type.is_symlink() {
            FileType::SymLink
        } else {
            FileType::Special
        }
    }
}

impl FileType {
    pub fn render(self, colors: &Colors) -> ColoredString {
        match self {
            FileType::File { exec, .. } => {
                colors.colorize(String::from("."), &Elem::File { exec, uid: false })
            }
            FileType::Directory { .. } => {
                colors.colorize(String::from("d"), &Elem::Dir { uid: false })
            }
            FileType::Pipe => colors.colorize(String::from("|"), &Elem::Pipe),
            FileType::SymLink => colors.colorize(String::from("l"), &Elem::SymLink),
            FileType::BlockDevice => colors.colorize(String::from("b"), &Elem::BlockDevice),
            FileType::CharDevice => colors.colorize(String::from("c"), &Elem::CharDevice),
            FileType::Socket => colors.colorize(String::from("s"), &Elem::Socket),
            FileType::Special => colors.colorize(String::from("?"), &Elem::Special),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FileType;
    use crate::color::{Colors, Theme};
    use crate::meta::Meta;
    #[cfg(unix)]
    use crate::meta::Permissions;
    use ansi_term::Colour;
    #[cfg(unix)]
    use std::fs::File;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(unix)]
    use std::os::unix::net::UnixListener;
    #[cfg(unix)]
    use std::process::Command;
    use tempdir::TempDir;

    #[test]
    #[cfg(unix)] // Windows uses different default permissions
    fn test_file_type() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));

        assert_eq!(Colour::Fixed(184).paint("."), file_type.render(&colors));
    }

    #[test]
    fn test_dir_type() {
        let tmp_dir = TempDir::new("test_dir_type").expect("failed to create temp dir");
        let meta =
            Meta::from_path(&tmp_dir.path().to_path_buf()).expect("failed to get tempdir path");
        let metadata = tmp_dir.path().metadata().expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&metadata, &meta.permissions);

        assert_eq!(Colour::Fixed(33).paint("d"), file_type.render(&colors));
    }

    #[test]
    #[cfg(unix)] // Symlink support is *hard* on Windows
    fn test_symlink_type() {
        let tmp_dir = TempDir::new("test_symlink_type").expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");

        // Create the symlink
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));

        assert_eq!(Colour::Fixed(44).paint("l"), file_type.render(&colors));
    }

    #[test]
    #[cfg(unix)] // Windows pipes aren't like Unix pipes
    fn test_pipe_type() {
        let tmp_dir = TempDir::new("test_pipe_type").expect("failed to create temp dir");

        // Create the pipe;
        let pipe_path = tmp_dir.path().join("pipe.tmp");
        let success = Command::new("mkfifo")
            .arg(&pipe_path)
            .status()
            .expect("failed to exec mkfifo")
            .success();
        assert_eq!(true, success, "failed to exec mkfifo");
        let meta = pipe_path.metadata().expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));

        assert_eq!(Colour::Fixed(44).paint("|"), file_type.render(&colors));
    }

    #[test]
    #[cfg(feature = "sudo")]
    fn test_char_device_type() {
        let tmp_dir = TempDir::new("test_char_device_type").expect("failed to create temp dir");

        // Create the char device;
        let char_device_path = tmp_dir.path().join("char-device.tmp");
        let success = Command::new("sudo")
            .arg("mknod")
            .arg(&char_device_path)
            .arg("c")
            .arg("89")
            .arg("1")
            .status()
            .expect("failed to exec mknod")
            .success();
        assert_eq!(true, success, "failed to exec mknod");
        let meta = char_device_path.metadata().expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));

        assert_eq!(Colour::Fixed(44).paint("c"), file_type.render(&colors));
    }

    #[test]
    #[cfg(unix)] // Sockets don't work the same way on Windows
    fn test_socket_type() {
        let tmp_dir = TempDir::new("test_socket_type").expect("failed to create temp dir");

        // Create the socket;
        let socket_path = tmp_dir.path().join("socket.tmp");
        UnixListener::bind(&socket_path).expect("failed to create the socket");
        let meta = socket_path.metadata().expect("failed to get metas");

        let colors = Colors::new(Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));

        assert_eq!(Colour::Fixed(44).paint("s"), file_type.render(&colors));
    }
}
