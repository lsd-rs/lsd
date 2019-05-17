use crate::meta::{FileType, Name};
use phf::phf_map;

pub static DEFAULT_ICONES_BY_NAME: phf::Map<&'static str, &'static str> = phf_map! {
        ".Trash" => "\u{f1f8}", // ""
        ".atom" => "\u{e764}", // ""
        ".bashprofile" => "\u{e615}", // ""
        ".bashrc" => "\u{f489}", // ""
        ".git" => "\u{f1d3}", // ""
        ".gitconfig" => "\u{f1d3}", // ""
        ".github" => "\u{f408}", // ""
        ".gitignore" =>"\u{f1d3}", // ""
        ".rvm" => "\u{e21e}", // ""
        ".vimrc" =>"\u{e62b}", // ""
        ".vscode" => "\u{e70c}", // ""
        ".zshrc" =>"\u{f489}", // ""
        "bin" => "\u{e5fc}", // ""
        "config" =>"\u{e5fc}", // ""
        "docker-compose.yml" => "\u{f308}", // ""
        "dockerfile" =>"\u{f308}", // ""
        "ds_store" => "\u{f179}", // ""
        "gitignore_global" =>"\u{f1d3}", // ""
        "gradle" =>"\u{e70e}",// ""
        "gruntfile.coffee" => "\u{e611}",// ""
        "gruntfile.js" => "\u{e611}", // ""
        "gruntfile.ls" => "\u{e611}", // ""
        "gulpfile.coffee" =>"\u{e610}", // ""
        "gulpfile.js" =>"\u{e610}", // ""
        "gulpfile.ls" => "\u{e610}", // ""
        "hidden" =>"\u{f023}",// ""
        "include" =>"\u{e5fc}",// ""
        "lib" => "\u{f121}", // ""
        "localized" =>"\u{f179}", // ""
        "node_modules" => "\u{e718}", // ""
        "npmignore" => "\u{e71e}", // ""
        "rubydoc" => "\u{e73b}", // ""
        "yarn.lock" => "\u{e718}", // ""
};

pub static DEFAULT_ICONES_BY_EXTENSION: phf::Map<&'static str, &'static str> = phf_map! {
        "apk"=> "\u{e70e}", // ""
        "avi"=> "\u{f03d}", // ""
        "avro"=> "\u{e60b}", // ""
        "awk"=> "\u{f489}", // ""
        "bash"=>"\u{f489}", // ""
        "bash_history"=> "\u{f489}", // ""
        "bash_profile"=>"\u{f489}", // ""
        "bashrc"=> "\u{f489}", // ""
        "bat"=> "\u{f17a}", // ""
        "bmp"=> "\u{f1c5}", // ""
        "c"=> "\u{e61e}", // ""
        "c++"=>"\u{e61d}", // ""
        "cc"=>"\u{e61d}", // ""
        "cfg"=> "\u{e615}", // ""
        "clj"=>"\u{e768}", // ""
        "cljs"=>"\u{e76a}", // ""
        "cls"=> "\u{e600}", // ""
        "coffee"=>"\u{f0f4}", // ""
        "conf"=> "\u{e615}", // ""
        "cp"=> "\u{e61d}", // ""
        "cpp"=>"\u{e61d}", // ""
        "csh"=> "\u{f489}", // ""
        "css"=>"\u{e749}", // ""
        "csv"=>"\u{f1c3}", // ""
        "cxx"=>"\u{e61d}", // ""
        "d"=>"\u{e7af}", // ""
        "dart"=>"\u{e798}", // ""
        "db"=> "\u{f1c0}", // ""
        "diff"=>"\u{f440}", // ""
        "doc"=> "\u{f1c2}", // ""
        "docx"=>"\u{f1c2}", // ""
        "ds_store"=>"\u{f179}", // ""
        "dump"=>"\u{f1c0}", // ""
        "ebook"=>"\u{e28b}", // ""
        "editorconfig"=>"\u{e615}", // ""
        "ejs"=>"\u{e618}", // ""
        "env"=>"\u{f462}", // ""
        "eot"=>"\u{f031}", // ""
        "epub"=>"\u{e28a}", // ""
        "erb"=>"\u{e73b}", // ""
        "erl"=>"\u{e7b1}", // ""
        "exe"=>"\u{f17a}", // ""
        "fish"=>"\u{f489}", // ""
        "flac"=>"\u{f001}", // ""
        "flv"=>"\u{f03d}", // ""
        "font"=>"\u{f031}", // ""
        "gdoc"=>"\u{f1c2}", // ""
        "gemfile"=>"\u{e21e}", // ""
        "gemspec"=>"\u{e21e}", // ""
        "gform"=> "\u{f298}", // ""
        "gif"=> "\u{f1c5}", // ""
        "git"=>"\u{f1d3}",// ""
        "go"=> "\u{e626}",// ""
        "gradle"=>"\u{e70e}", // ""
        "gsheet"=>"\u{f1c3}", // ""
        "gslides"=>"\u{f1c4}", // ""
        "guardfile"=>"\u{e21e}", // ""
        "gz"=>"\u{f410}", // ""
        "h"=>"\u{f0fd}", // ""
        "hbs"=>"\u{e60f}", // ""
        "hpp"=>"\u{f0fd}", // ""
        "hs"=>"\u{e777}", // ""
        "htm"=>"\u{f13b}", // ""
        "html"=>"\u{f13b}", // ""
        "hxx"=>"\u{f0fd}", // ""
        "ico"=> "\u{f1c5}", // ""
        "image"=>"\u{f1c5}", // ""
        "iml"=> "\u{e7b5}", // ""
        "ini"=>"\u{f17a}", // ""
        "ipynb"=> "\u{e606}", // ""
        "jar"=>"\u{e204}", // ""
        "java"=>"\u{e204}", // ""
        "jpeg"=> "\u{f1c5}", // ""
        "jpg"=> "\u{f1c5}", // ""
        "js"=> "\u{e74e}", // ""
        "json"=>"\u{e60b}", // ""
        "jsx"=>"\u{e7ba}", // ""
        "ksh"=> "\u{f489}", // ""
        "less"=>"\u{e758}", // ""
        "lhs"=>"\u{e777}", // ""
        "license"=>"\u{f48a}", // ""
        "localized"=>"\u{f179}", // ""
        "lock"=> "\u{e21e}", // ""
        "log"=> "\u{f18d}", // ""
        "lua"=>"\u{e620}", // ""
        "m4a"=>"\u{f001}", // ""
        "markdown"=>"\u{f48a}", // ""
        "md"=>"\u{f48a}", // ""
        "mkd"=>"\u{f48a}", // ""
        "mkv"=>"\u{f03d}", // ""
        "mobi"=>"\u{e28b}", // ""
        "mov"=>"\u{f03d}", // ""
        "mp3"=>"\u{f001}", // ""
        "mp4"=> "\u{f03d}", // ""
        "mustache"=>"\u{e60f}", // ""
        "npmignore"=>"\u{e71e}", // ""
        "ogg"=>"\u{f001}", // ""
        "ogv"=> "\u{f03d}", // ""
        "otf"=>"\u{f031}", // ""
        "pdf"=>"\u{f1c1}", // ""
        "php"=> "\u{e73d}", // ""
        "pl"=> "\u{e769}", // ""
        "png"=>"\u{f1c5}", // ""
        "ppt"=> "\u{f1c4}", // ""
        "pptx"=> "\u{f1c4}", // ""
        "procfile"=> "\u{e21e}", // ""
        "properties"=> "\u{e60b}", // ""
        "ps1"=>"\u{f489}",// ""
        "psd"=>"\u{e7b8}", // ""
        "pxm"=> "\u{f1c5}",// ""
        "py"=> "\u{e606}", // ""
        "pyc"=>"\u{e606}", // ""
        "r"=>"\u{f25d}", // ""
        "rakefile"=> "\u{e21e}", // ""
        "rar"=>"\u{f410}", // ""
        "rb"=> "\u{e21e}", // ""
        "rdata"=> "\u{f25d}", // ""
        "rdb"=>"\u{e76d}", // ""
        "rdoc"=> "\u{f48a}", // ""
        "rds"=> "\u{f25d}", // ""
        "readme"=>"\u{f48a}", // ""
        "rlib"=> "\u{e7a8}", // ""
        "rmd"=> "\u{f48a}", // ""
        "rs"=> "\u{e7a8}", // ""
        "rspec"=> "\u{e21e}", // ""
        "rspec_parallel"=> "\u{e21e}", // ""
        "rspec_status"=> "\u{e21e}", // ""
        "rss"=>"\u{f09e}", // ""
        "ru"=> "\u{e21e}", // ""
        "rubydoc"=> "\u{e73b}", // ""
        "sass"=> "\u{e603}", // ""
        "scala"=> "\u{e737}", // ""
        "scss"=> "\u{e749}", // ""
        "sh"=> "\u{f489}", // ""
        "shell"=> "\u{f489}", // ""
        "slim"=> "\u{e73b}", // ""
        "sql"=>"\u{f1c0}", // ""
        "sqlite3"=> "\u{e7c4}", // ""
        "styl"=> "\u{e600}", // ""
        "stylus"=> "\u{e600}", // ""
        "svg"=>"\u{f1c5}", // ""
        "swift"=> "\u{e755}", // ""
        "tar"=> "\u{f410}", // ""
        "tex"=>"\u{e600}", // ""
        "tiff"=> "\u{f1c5}", // ""
        "ts"=>"\u{e628}", // ""
        "tsx"=>"\u{e7ba}", // ""
        "ttf"=> "\u{f031}", // ""
        "twig"=> "\u{e61c}", // ""
        "txt"=> "\u{f15c}", // ""
        "video"=> "\u{f03d}", // ""
        "vim"=> "\u{e62b}", // ""
        "vue"=> "\u{fd42}", // "﵂"
        "wav"=> "\u{f001}", // ""
        "webm"=> "\u{f03d}", // ""
        "webp"=> "\u{f1c5}", // ""
        "windows"=> "\u{f17a}", // ""
        "woff"=> "\u{f031}", // ""
        "woff2"=> "\u{f031}", // ""
        "xls"=> "\u{f1c3}", // ""
        "xlsx"=> "\u{f1c3}", // ""
        "xml"=>"\u{e619}", // ""
        "xul"=> "\u{e619}", // ""
        "yaml"=> "\u{f481}", // ""
        "yml"=> "\u{f481}", // ""
        "zip"=>"\u{f410}", // ""
        "zsh"=>"\u{f489}", // ""
        "zsh-theme"=>"\u{f489}", // ""
        "zshrc"=>"\u{f489}", // ""
};

pub struct Icons {
    display_icons: bool,
    icons_by_name: Option<&'static phf::Map<&'static str, &'static str>>,
    icons_by_extension: Option<&'static phf::Map<&'static str, &'static str>>,
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

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(theme: Theme) -> Self {
        let display_icons = theme == Theme::Fancy || theme == Theme::Unicode;
        let (icons_by_name, icons_by_extension, default_file_icon, default_folder_icon) =
            if theme == Theme::Fancy {
                (
                    Some(&DEFAULT_ICONES_BY_NAME),
                    Some(&DEFAULT_ICONES_BY_EXTENSION),
                    "\u{f016}", // 
                    "\u{f115}", // 
                )
            } else {
                (
                    None,
                    None,
                    "\u{1f5cb}", // 🗋
                    "\u{1f5c1}", // 🗁
                )
            };

        Self {
            display_icons,
            icons_by_name,
            icons_by_extension,
            default_file_icon,
            default_folder_icon,
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
        if let Some(map) = self.icons_by_name {
            if let Some(icon) = map.get(name.name().as_str()) {
                res += icon;
                res += ICON_SPACE;
                return res;
            }
        }

        // Check the known extensions.

        if let Some(extension) = name.extension() {
            if let Some(map) = self.icons_by_extension {
                if let Some(icon) = map.get(extension.as_str()) {
                    res += icon;
                    res += ICON_SPACE;
                    return res;
                }
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
    use super::{Icons, Theme, DEFAULT_ICONES_BY_EXTENSION, DEFAULT_ICONES_BY_NAME, ICON_SPACE};
    use crate::meta::Meta;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path).unwrap();

        let icon = Icons::new(Theme::NoIcon);
        let icon = icon.get(&meta.name);

        assert_eq!(icon, "");
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path).unwrap();

        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&meta.name);

        assert_eq!(icon, format!("{}{}", "\u{f016}", ICON_SPACE)); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path).unwrap();

        let icon = Icons::new(Theme::Unicode);
        let icon = icon.get(&meta.name);
        assert_eq!(icon, format!("{}{}", "\u{1f5cb}", ICON_SPACE));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf()).unwrap();

        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&meta.name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf()).unwrap();

        let icon = Icons::new(Theme::Unicode);
        let icon = icon.get(&meta.name);
        
        assert_eq!(icon, format!("{}{}", "\u{1f5c1}", ICON_SPACE));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = TempDir::new("test_file_type.rs").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf()).unwrap();

        let icon = Icons::new(Theme::Fancy);
        let icon = icon.get(&meta.name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (file_name, file_icon) in &DEFAULT_ICONES_BY_NAME {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path).unwrap();

            let icon = Icons::new(Theme::Fancy);
            let icon = icon.get(&meta.name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (ext, file_icon) in &DEFAULT_ICONES_BY_EXTENSION {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path).unwrap();

            let icon = Icons::new(Theme::Fancy);
            let icon = icon.get(&meta.name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }
}
