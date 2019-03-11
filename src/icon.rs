use crate::meta::{FileType, Name};
use phf::map::Map;

pub struct Icons {
    display_icons: bool,
    icons_by_name: &'static Map<&'static str, &'static str>,
    icons_by_extension: &'static Map<&'static str, &'static str>,
    default_folder_icon: &'static str,
    default_file_icon: &'static str,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
    NoIcon,
    Fancy,
    Unicode,
}

const ICON_SPACE: &str = "  ";

include!(concat!(env!("OUT_DIR"), "/default_icons_by_name.rs"));
include!(concat!(env!("OUT_DIR"), "/default_icons_by_extension.rs"));
include!(concat!(env!("OUT_DIR"), "/empty_icon_map.rs"));

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(theme: Theme) -> Self {
        let display_icons = theme == Theme::Fancy || theme == Theme::Unicode;
        if theme == Theme::Fancy {
            Icons {
                display_icons: display_icons,
                icons_by_name: &DEFAULT_ICONS_BY_NAME,
                icons_by_extension: &DEFAULT_ICONS_BY_EXTENSION,
                default_file_icon: "\u{f016}", // 
                default_folder_icon: "\u{f115}", // 
            }
        } else {
            Icons {
                display_icons: display_icons,
                icons_by_name: &EMPTY_ICON_MAP,
                icons_by_extension: &EMPTY_ICON_MAP,
                default_file_icon: "\u{1f5cb}", // 🗋
                default_folder_icon: "\u{1f5c1}", // 🗁
            }
        }
    }

    pub fn get(&self, name: &Name) -> String {
        if !self.display_icons {
            return String::new();
        }

        let mut res = String::with_capacity(4 + ICON_SPACE.len()); // 4 == max icon size

        // Check directory.
        if let FileType::Directory { .. } = name.file_type() {
            res += self.default_folder_icon;
            res += ICON_SPACE;
            return res;
        }

        // Check the known names.
        if let Some(icon) = self.icons_by_name.get(name.name().as_str()) {
            res += icon;
            res += ICON_SPACE;
            return res;
        }

        // Check the known extensions.
        if let Some(extension) = name.extension() {
            if let Some(icon) = self.icons_by_extension.get(extension.as_str()) {
                res += icon;
                res += ICON_SPACE;
                return res;
            }
        }

        // Use the default icons.
        res += self.default_file_icon;
        res += ICON_SPACE;
        res
    }

}

#[cfg(test)]
mod test {
    use super::{Icons, Theme, ICON_SPACE};
    use crate::meta::{FileType, Name, Permissions};
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::NoIcon);
        let icon = icon.get(&name);

        assert_eq!(icon, "");
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f016}", ICON_SPACE)); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Unicode);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{1f5cb}", ICON_SPACE));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Unicode);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{1f5c1}", ICON_SPACE));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = TempDir::new("test_file_type.rs").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (file_name, file_icon) in &DEFAULT_ICONS_BY_NAME {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = file_path.metadata().expect("failed to get metas");

            let file_type = FileType::new(&meta, &Permissions::from(&meta));
            let name = Name::new(&file_path, file_type);
            let icon = Icons::new(Theme::Fancy);
            let icon = icon.get(&name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (ext, file_icon) in &DEFAULT_ICONS_BY_EXTENSION {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = file_path.metadata().expect("failed to get metas");

            let file_type = FileType::new(&meta, &Permissions::from(&meta));
            let name = Name::new(&file_path, file_type);
            let icon = Icons::new(Theme::Fancy);
            let icon = icon.get(&name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }
}
