use crate::color::{ColoredString, Colors, Elem};
use crate::icon::Icons;
use crate::meta::filetype::FileType;
use std::cmp::{Ordering, PartialOrd};
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub enum DisplayOption<'a> {
    FileName,
    Relative { base_path: &'a Path },
    None,
}

#[derive(Clone, Debug, Eq)]
pub struct Name {
    pub name: String,
    path: PathBuf,
    extension: Option<String>,
    file_type: FileType,
}

impl Name {
    pub fn new(path: &Path, file_type: FileType) -> Self {
        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };

        let mut extension = None;
        if let Some(res) = path.extension() {
            extension = Some(
                res.to_str()
                    .expect("failed to encode file name")
                    .to_string(),
            );
        }

        Self {
            name,
            path: PathBuf::from(path),
            extension,
            file_type,
        }
    }

    pub fn file_name(&self) -> &str {
        self.path.file_name().and_then(OsStr::to_str).unwrap()
    }

    fn relative_path<T: AsRef<Path> + Clone>(&self, base_path: T) -> PathBuf {
        let base_path = base_path.as_ref();

        let shared_components: PathBuf = self
            .path
            .components()
            .zip(base_path.components())
            .take_while(|(target_component, base_component)| target_component == base_component)
            .map(|tuple| tuple.0)
            .collect();

        base_path
            .strip_prefix(&shared_components)
            .unwrap()
            .components()
            .map(|_| Component::ParentDir)
            .chain(
                self.path
                    .strip_prefix(&shared_components)
                    .unwrap()
                    .components(),
            )
            .collect()
    }

    pub fn render(
        &self,
        colors: &Colors,
        icons: &Icons,
        display_option: &DisplayOption,
    ) -> ColoredString {
        let content = match display_option {
            DisplayOption::FileName => format!("{}{}", icons.get(self), self.file_name()),
            DisplayOption::Relative { base_path } => format!(
                "{}{}",
                icons.get(self),
                self.relative_path(base_path).to_string_lossy()
            ),
            DisplayOption::None => format!("{}{}", icons.get(self), self.path.to_string_lossy()),
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
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name.to_lowercase())
    }
}

#[cfg(test)]
mod test {
    use super::DisplayOption;
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
    use std::path::{Path, PathBuf};
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
            name.render(&colors, &icons, &DisplayOption::FileName)
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
            meta.name.render(&colors, &icons, &DisplayOption::FileName)
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
        let name = Name::new(&symlink_path, file_type);

        assert_eq!(
            Colour::Fixed(44).paint(" target.tmp"),
            name.render(&colors, &icons, &DisplayOption::FileName)
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
        let name = Name::new(&pipe_path, file_type);

        assert_eq!(
            Colour::Fixed(184).paint(" pipe.tmp"),
            name.render(&colors, &icons, &DisplayOption::FileName)
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
            meta.name
                .render(&colors, &icons, &DisplayOption::FileName)
                .to_string()
                .as_str()
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
        let path_1 = Path::new("/AAAA");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_2 = Path::new("/aaaa");
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
        let path_a = Path::new("/aaaa");
        let name_a = Name::new(
            &path_a,
            FileType::File {
                uid: false,
                exec: false,
            },
        );

        let path_z = Path::new("/zzzz");
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

    #[test]
    fn test_parent_relative_path() {
        let name = Name::new(
            Path::new("/home/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = Path::new("/home/parent2");

        assert_eq!(
            PathBuf::from("../parent1/child"),
            name.relative_path(base_path),
        )
    }

    #[test]
    fn test_current_relative_path() {
        let name = Name::new(
            Path::new("/home/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = PathBuf::from("/home/parent1");

        assert_eq!(PathBuf::from("child"), name.relative_path(base_path),)
    }

    #[test]
    fn test_grand_parent_relative_path() {
        let name = Name::new(
            Path::new("/home/grand-parent1/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = PathBuf::from("/home/grand-parent2/parent1");

        assert_eq!(
            PathBuf::from("../../grand-parent1/parent1/child"),
            name.relative_path(base_path),
        )
    }
}
