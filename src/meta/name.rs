use crate::color::{ColoredString, Colors, Elem};
use crate::icon::Icons;
use crate::meta::filetype::FileType;
use std::cmp::{Ordering, PartialOrd};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub enum DisplayOption<'a> {
    Parent,
    Current,
    FileName,
    Relative{base_path: &'a Path},
}

#[derive(Clone, Debug, Eq)]
pub struct Name {
    path: PathBuf,
    extension: Option<String>,
    file_type: FileType,
}

impl Name {
    pub fn file_name(&self) -> &str {
        self.path.file_name().and_then(OsStr::to_str).unwrap_or("?")
    }

    fn relative_path(&self, base_path: &Path) -> std::borrow::Cow<'_, str> {
        if let Ok(relative_path) = self.path.strip_prefix(base_path) {
            relative_path.to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("?")
        }
    }

    pub fn new(path: &Path, file_type: FileType) -> Self {
        let mut extension = None;
        if let Some(res) = path.extension() {
            extension = Some(
                res.to_str()
                    .expect("failed to encode file name")
                    .to_string(),
            );
        }

        Self {
            path: PathBuf::from(path),
            extension,
            file_type,
        }
    }

    pub fn render(&self, colors: &Colors, icons: &Icons, display_option: &DisplayOption) -> ColoredString {
        let content = match display_option {
            DisplayOption::Parent => format!("{}..", icons.get(self)),
            DisplayOption::Current => format!("{}.", icons.get(self)),
            DisplayOption::FileName => format!("{}{}", icons.get(self), self.file_name()),
            DisplayOption::Relative{base_path} => format!("{}{}", icons.get(self), self.relative_path(base_path)),
        };

        let elem = match self.file_type {
            FileType::CharDevice => Elem::CharDevice,
            FileType::Directory { uid } => Elem::Dir { uid },
            FileType::SymLink => Elem::SymLink,
            FileType::File { uid, exec } => Elem::File { uid, exec },
            _ => Elem::File {
                exec: false,
                uid: false,
            },
        };

        colors.colorize_using_path(content, &self.path, &elem)
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension.as_ref().map(|string| string.as_str())
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use crate::color::{self, Colors};
    use crate::icon::{self, Icons};
    use crate::meta::FileType;
    use crate::meta::Meta;
    #[cfg(unix)]
    use crate::meta::Permissions;
    use ansi_term::Colour;
    use std::cmp::Ordering;
    use std::fs::{self, File};
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::path::Path;
    #[cfg(unix)]
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    #[cfg(unix)] // Windows uses different default permissions
    fn test_print_file_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy);

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(color::Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);

        assert_eq!(
            Colour::Fixed(184).paint(" file.txt"),
            name.render(&colors, &icons)
        );
    }

    #[test]
    fn test_print_dir_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy);

        // Chreate the directory
        let dir_path = tmp_dir.path().join("directory");
        fs::create_dir(&dir_path).expect("failed to create the dir");
        let meta = Meta::from_path(&dir_path).unwrap();

        let colors = Colors::new(color::Theme::NoLscolors);

        assert_eq!(
            Colour::Fixed(33).paint(" directory"),
            meta.name.render(&colors, &icons)
        );
    }

    #[test]
    #[cfg(unix)] // Symlinks are hard on Windows
    fn test_print_symlink_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy);

        // Create the file;
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");

        // Create the symlink
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");

        let colors = Colors::new(color::Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&tmp_dir.into_path(), &symlink_path, file_type);

        assert_eq!(
            Colour::Fixed(44).paint(" target.tmp"),
            name.render(&colors, &icons)
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_print_other_type_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy);

        // Create the pipe;
        let pipe_path = tmp_dir.path().join("pipe.tmp");
        let success = Command::new("mkfifo")
            .arg(&pipe_path)
            .status()
            .expect("failed to exec mkfifo")
            .success();
        assert_eq!(true, success, "failed to exec mkfifo");
        let meta = pipe_path.metadata().expect("failed to get metas");

        let colors = Colors::new(color::Theme::NoLscolors);
        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&tmp_dir.into_path(), &pipe_path, file_type);

        assert_eq!(
            Colour::Fixed(184).paint(" pipe.tmp"),
            name.render(&colors, &icons)
        );
    }

    #[test]
    fn test_print_without_icon_or_color() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::NoIcon);

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path).unwrap();

        let colors = Colors::new(color::Theme::NoColor);

        assert_eq!(
            "file.txt",
            meta.name.render(&colors, &icons).to_string().as_str()
        );
    }

    #[test]
    fn test_extensions_with_valid_file() {
        let path = Path::new("some-file.txt");

        let name = Name::new(
            &path,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(Some("txt"), name.extension());
    }

    #[test]
    fn test_extensions_with_file_without_extension() {
        let path = Path::new(".gitignore");

        let name = Name::new(
            &path,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(None, name.extension());
    }

    #[test]
    fn test_order_impl_is_case_insensitive() {
        let path_1 = Path::new("AAAA");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_2 = Path::new("aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(Ordering::Equal, name_1.cmp(&name_2));
    }

    #[test]
    fn test_partial_order_impl() {
        let path_a = Path::new("aaaa");
        let name_a = Name::new(
            &path_a,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_z = Path::new("zzzz");
        let name_z = Name::new(
            &path_z,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(true, name_a < name_z);
    }

    #[test]
    fn test_partial_order_impl_is_case_insensitive() {
        let path_a = Path::new("aaaa");
        let name_a = Name::new(
            &path_a,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_z = Path::new("ZZZZ");
        let name_z = Name::new(
            &path_z,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(true, name_a < name_z);
    }

    #[test]
    fn test_partial_eq_impl() {
        let path_1 = Path::new("aaaa");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_2 = Path::new("aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(true, name_1 == name_2);
    }

    #[test]
    fn test_partial_eq_impl_is_case_insensitive() {
        let path_1 = Path::new("AAAA");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_2 = Path::new("aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        assert_eq!(true, name_1 == name_2);
    }
}
