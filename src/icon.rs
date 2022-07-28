use std::collections::HashMap;

use crate::meta::{FileType, Name};
use crate::flags::IconOption;
use crate::theme::{Theme, icon::IconTheme};

pub struct Icons {
    display_icons: bool,
    icon_separator: String,

    theme: IconTheme,
}

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(opt: IconOption, icon_separator: String) -> Self {
        let display_icons = !(opt == IconOption::Never); // TODO(zwpaper): Auto

        Self {
            display_icons,
            icon_separator,
            theme: Theme::default().icon,
        }
    }

    pub fn get(&self, name: &Name) -> String {
        if !self.display_icons {
            return String::new();
        }

        // Check file types
        let file_type: FileType = name.file_type();
        let icon = match file_type {
            FileType::SymLink { is_dir: true } => "\u{f482}", // ""
            FileType::SymLink { is_dir: false } => "\u{f481}", // ""
            FileType::Socket => "\u{f6a7}",                   // ""
            FileType::Pipe => "\u{f731}",                     // ""
            FileType::CharDevice => "\u{e601}",               // ""
            FileType::BlockDevice => "\u{fc29}",              // "ﰩ"
            FileType::Special => "\u{f2dc}",                  // ""
            _ => {
                // Use the known names
                if let Some(icon) = self
                    .theme
                    .icons_by_name
                    .get(name.file_name().to_lowercase().as_str())
                {
                    icon
                }
                // Use the known extensions
                else if let Some(icon) = name.extension().and_then(|extension| {
                    self.theme
                        .icons_by_extension
                        .get(extension.to_lowercase().as_str())
                }) {
                    icon
                } else {
                    match file_type {
                        FileType::Directory { .. } => &self.theme.default_folder_icon,
                        // If a file has no extension and is executable, show an icon.
                        // Except for Windows, it marks everything as an executable.
                        #[cfg(not(windows))]
                        FileType::File { exec: true, .. } => "\u{f489}", // ""
                        _ => &self.theme.default_file_icon,
                    }
                }
            }
        };

        format!("{}{}", icon, self.icon_separator)
    }
}

#[cfg(test)]
mod test {
    use super::{Icons, Theme};
    use crate::meta::Meta;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::NoIcon, " ".to_string());
        let icon = icon.get(&meta.name);

        assert_eq!(icon, "");
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f016} "); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{1f5cb} ");
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f115} "); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{1f5c1} ");
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f115} "); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (file_name, file_icon) in &Icons::get_default_icons_by_name() {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(Theme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (ext, file_icon) in &Icons::get_default_icons_by_extension() {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::use Theme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }
}
