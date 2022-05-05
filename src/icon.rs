use crate::meta::{FileType, Name};
use std::collections::HashMap;

pub struct Icons {
    display_icons: bool,
    icons_by_name: HashMap<&'static str, &'static str>,
    icons_by_extension: HashMap<&'static str, &'static str>,
    default_folder_icon: &'static str,
    default_file_icon: &'static str,
    icon_separator: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
    NoIcon,
    Fancy,
    Unicode,
}

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(theme: Theme, icon_separator: String) -> Self {
        let display_icons = theme == Theme::Fancy || theme == Theme::Unicode;
        let (icons_by_name, icons_by_extension, default_file_icon, default_folder_icon) =
            if theme == Theme::Fancy {
                (
                    Self::get_default_icons_by_name(),
                    Self::get_default_icons_by_extension(),
                    "\u{f016}", // 
                    "\u{f115}", // 
                )
            } else {
                (
                    HashMap::new(),
                    HashMap::new(),
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
            icon_separator,
        }
    }

    pub fn get(&self, name: &Name) -> String {
        if !self.display_icons {
            return String::new();
        }

        // Check file types
        let file_type: FileType = name.file_type();

        let icon = if let FileType::Directory { .. } = file_type {
            self.default_folder_icon
        } else if let FileType::SymLink { is_dir: true } = file_type {
            "\u{f482}" // ""
        } else if let FileType::SymLink { is_dir: false } = file_type {
            "\u{f481}" // ""
        } else if let FileType::Socket = file_type {
            "\u{f6a7}" // ""
        } else if let FileType::Pipe = file_type {
            "\u{f731}" // ""
        } else if let FileType::CharDevice = file_type {
            "\u{e601}" // ""
        } else if let FileType::BlockDevice = file_type {
            "\u{fc29}" // "ﰩ"
        } else if let FileType::Special = file_type {
            "\u{f2dc}" // ""
        } else if let Some(icon) = self
            .icons_by_name
            .get(name.file_name().to_lowercase().as_str())
        {
            // Use the known names.
            icon
        } else if let Some(icon) = name.extension().and_then(|extension| {
            self.icons_by_extension
                .get(extension.to_lowercase().as_str())
        }) {
            // Use the known extensions.
            icon
        } else if let FileType::File { exec: true, .. } = file_type {
            // If file has no extension and is executable
            "\u{f489}" // ""
        } else {
            // Use the default icons.
            self.default_file_icon
        };

        format!("{}{}", icon, self.icon_separator)
    }

    fn get_default_icons_by_name() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        // Note: filenames must be lower-case

        m.insert(".trash", "\u{f1f8}"); // ""
        m.insert(".atom", "\u{e764}"); // ""
        m.insert(".bash_profile", "\u{f489}"); // ""
        m.insert(".bash_logout", "\u{f489}"); // ""
        m.insert(".bashrc", "\u{f489}"); // ""
        m.insert(".clang-format", "\u{e615}"); // ""
        m.insert(".config", "\u{e5fc}"); // ""
        m.insert(".emacs.d", "\u{e779}"); // ""
        m.insert(".git", "\u{e5fb}"); // ""
        m.insert(".gitattributes", "\u{f1d3}"); // ""
        m.insert(".gitconfig", "\u{f1d3}"); // ""
        m.insert(".github", "\u{f408}"); // ""
        m.insert(".gitignore", "\u{f1d3}"); // ""
        m.insert(".gitlab-ci.yml", "\u{f296}"); // ""
        m.insert(".gitmodules", "\u{f1d3}"); // ""
        m.insert(".rvm", "\u{e21e}"); // ""
        m.insert(".vimrc", "\u{e62b}"); // ""
        m.insert(".viminfo", "\u{e62b}"); // ""
        m.insert(".vscode", "\u{e70c}"); // ""
        m.insert(".xauthority", "\u{e615}"); // ""
        m.insert(".xdefaults", "\u{e615}"); // ""
        m.insert(".xinitrc", "\u{e615}"); // ""
        m.insert(".xresources", "\u{e615}"); // ""
        m.insert(".zshrc", "\u{f489}"); // ""
        m.insert("a.out", "\u{f489}"); // ""
        m.insert("authorized_keys", "\u{e60a}"); // ""
        m.insert("bin", "\u{e5fc}"); // ""
        m.insert("bspwmrc", "\u{e615}"); // ""
        m.insert("cargo.toml", "\u{e7a8}"); // ""
        m.insert("cargo.lock", "\u{e7a8}"); // ""
        m.insert("changelog", "\u{f48a}"); // ""
        m.insert("composer.json", "\u{e608}"); // ""
        m.insert("config", "\u{e5fc}"); // ""
        m.insert("config.mk", "\u{e615}"); // ""
        m.insert("config.ac", "\u{e615}"); // ""
        m.insert("desktop", "\u{f108}"); // ""
        m.insert("docker-compose.yml", "\u{f308}"); // ""
        m.insert("dockerfile", "\u{f308}"); // ""
        m.insert("downloads", "\u{f498}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
        m.insert("favicon.ico", "\u{f005}"); // ""
        m.insert("gitignore_global", "\u{f1d3}"); // ""
        m.insert("gradle", "\u{e70e}"); // ""
        m.insert("gruntfile.coffee", "\u{e611}"); // ""
        m.insert("gruntfile.js", "\u{e611}"); // ""
        m.insert("gruntfile.ls", "\u{e611}"); // ""
        m.insert("gulpfile.coffee", "\u{e610}"); // ""
        m.insert("gulpfile.js", "\u{e610}"); // ""
        m.insert("gulpfile.ls", "\u{e610}"); // ""
        m.insert("hidden", "\u{f023}"); // ""
        m.insert("include", "\u{e5fc}"); // ""
        m.insert("known_hosts", "\u{e60a}"); // ""
        m.insert("lib", "\u{f121}"); // ""
        m.insert("license", "\u{e60a}"); // ""
        m.insert("license.md", "\u{e60a}"); // ""
        m.insert("license.txt", "\u{e60a}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("makefile", "\u{e615}"); // ""
        m.insert("makefile.ac", "\u{e615}"); // ""
        m.insert("muttrc", "\u{e615}"); // ""
        m.insert("node_modules", "\u{e718}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("package.json", "\u{e718}"); // ""
        m.insert("package-lock.json", "\u{e718}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""
        m.insert("robots.txt", "\u{fba7}"); // "ﮧ"
        m.insert("root", "\u{f023}"); // ""
        m.insert("sxhkdrc", "\u{e615}"); // ""
        m.insert("tmp", "\u{f1f8}"); // ""
        m.insert("vagrantfile", "\u{e615}"); // ""
        m.insert("webpack.config.js", "\u{fc29}"); // "ﰩ"
        m.insert("xmonad.hs", "\u{e615}"); // ""

        m
    }

    fn get_default_icons_by_extension() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        // Note: extensions must be lower-case

        m.insert("1", "\u{f02d}"); // ""
        m.insert("7z", "\u{f410}"); // ""
        m.insert("a", "\u{e624}"); // ""
        m.insert("ai", "\u{e7b4}"); // ""
        m.insert("ape", "\u{f001}"); // ""
        m.insert("apk", "\u{e70e}"); // ""
        m.insert("asm", "\u{e614}"); // ""
        m.insert("asp", "\u{f121}"); // ""
        m.insert("avi", "\u{f008}"); // ""
        m.insert("avro", "\u{e60b}"); // ""
        m.insert("awk", "\u{f489}"); // ""
        m.insert("bash", "\u{f489}"); // ""
        m.insert("bash_history", "\u{f489}"); // ""
        m.insert("bash_profile", "\u{f489}"); // ""
        m.insert("bashrc", "\u{f489}"); // ""
        m.insert("bat", "\u{f17a}"); // ""
        m.insert("bin", "\u{f489}"); // ""
        m.insert("bio", "\u{f910}"); // "蘿"
        m.insert("bmp", "\u{f1c5}"); // ""
        m.insert("bz2", "\u{f410}"); // ""
        m.insert("c", "\u{e61e}"); // ""
        m.insert("c++", "\u{e61d}"); // ""
        m.insert("cc", "\u{e61d}"); // ""
        m.insert("cfg", "\u{e615}"); // ""
        m.insert("cl", "\u{f671}"); // ""
        m.insert("class", "\u{e738}"); // ""
        m.insert("clj", "\u{e768}"); // ""
        m.insert("cljs", "\u{e76a}"); // ""
        m.insert("cls", "\u{e600}"); // ""
        m.insert("coffee", "\u{f0f4}"); // ""
        m.insert("conf", "\u{e615}"); // ""
        m.insert("config", "\u{e5fc}"); // ""
        m.insert("cp", "\u{e61d}"); // ""
        m.insert("cpp", "\u{e61d}"); // ""
        m.insert("cs", "\u{f81a}"); // ""
        m.insert("cshtml", "\u{f1fa}"); // ""
        m.insert("csproj", "\u{f81a}"); // ""
        m.insert("csx", "\u{f81a}"); // ""
        m.insert("csh", "\u{f489}"); // ""
        m.insert("css", "\u{e749}"); // ""
        m.insert("csv", "\u{f1c3}"); // ""
        m.insert("cue", "\u{f001}"); // ""
        m.insert("cxx", "\u{e61d}"); // ""
        m.insert("d", "\u{e7af}"); // ""
        m.insert("dart", "\u{e798}"); // ""
        m.insert("db", "\u{f1c0}"); // ""
        m.insert("deb", "\u{f187}"); // ""
        m.insert("diff", "\u{e728}"); // ""
        m.insert("dll", "\u{f17a}"); // ""
        m.insert("doc", "\u{f1c2}"); // ""
        m.insert("dockerfile", "\u{f308}"); // ""
        m.insert("docx", "\u{f1c2}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
        m.insert("dump", "\u{f1c0}"); // ""
        m.insert("ebook", "\u{e28b}"); // ""
        m.insert("editorconfig", "\u{e615}"); // ""
        m.insert("ejs", "\u{e618}"); // ""
        m.insert("el", "\u{e779}"); // ""
        m.insert("elc", "\u{e779}"); // ""
        m.insert("elf", "\u{f489}"); // ""
        m.insert("elm", "\u{e62c}"); // ""
        m.insert("env", "\u{f462}"); // ""
        m.insert("eot", "\u{f031}"); // ""
        m.insert("epub", "\u{e28a}"); // ""
        m.insert("erb", "\u{e73b}"); // ""
        m.insert("erl", "\u{e7b1}"); // ""
        m.insert("exe", "\u{f17a}"); // ""
        m.insert("ex", "\u{e62d}"); // ""
        m.insert("exs", "\u{e62d}"); // ""
        m.insert("fish", "\u{f489}"); // ""
        m.insert("flac", "\u{f001}"); // ""
        m.insert("flv", "\u{f008}"); // ""
        m.insert("font", "\u{f031}"); // ""
        m.insert("fpl", "\u{f910}"); // "蘿"
        m.insert("fs", "\u{e7a7}"); // ""
        m.insert("fsx", "\u{e7a7}"); // ""
        m.insert("fsi", "\u{e7a7}"); // ""
        m.insert("gdoc", "\u{f1c2}"); // ""
        m.insert("gemfile", "\u{e21e}"); // ""
        m.insert("gemspec", "\u{e21e}"); // ""
        m.insert("gform", "\u{f298}"); // ""
        m.insert("gif", "\u{f1c5}"); // ""
        m.insert("git", "\u{f1d3}"); // ""
        m.insert("go", "\u{e724}"); // ""
        m.insert("gradle", "\u{e70e}"); // ""
        m.insert("gsheet", "\u{f1c3}"); // ""
        m.insert("gslides", "\u{f1c4}"); // ""
        m.insert("guardfile", "\u{e21e}"); // ""
        m.insert("gz", "\u{f410}"); // ""
        m.insert("h", "\u{f0fd}"); // ""
        m.insert("hbs", "\u{e60f}"); // ""
        m.insert("heic", "\u{f1c5}"); // ""
        m.insert("heif", "\u{f1c5}"); // ""
        m.insert("heix", "\u{f1c5}"); // ""
        m.insert("hpp", "\u{f0fd}"); // ""
        m.insert("hs", "\u{e777}"); // ""
        m.insert("htm", "\u{f13b}"); // ""
        m.insert("html", "\u{f13b}"); // ""
        m.insert("hxx", "\u{f0fd}"); // ""
        m.insert("ico", "\u{f1c5}"); // ""
        m.insert("image", "\u{f1c5}"); // ""
        m.insert("img", "\u{f1c0}"); // ""
        m.insert("iml", "\u{e7b5}"); // ""
        m.insert("ini", "\u{e615}"); // ""
        m.insert("ipynb", "\u{e606}"); // ""
        m.insert("iso", "\u{f1c0}"); // ""
        m.insert("jar", "\u{e738}"); // ""
        m.insert("java", "\u{e738}"); // ""
        m.insert("jpeg", "\u{f1c5}"); // ""
        m.insert("jpg", "\u{f1c5}"); // ""
        m.insert("js", "\u{e74e}"); // ""
        m.insert("json", "\u{e60b}"); // ""
        m.insert("jsp", "\u{e738}"); // ""
        m.insert("jsx", "\u{e7ba}"); // ""
        m.insert("jl", "\u{e624}"); // ""
        m.insert("key", "\u{e60a}"); // ""
        m.insert("ksh", "\u{f489}"); // ""
        m.insert("ld", "\u{e624}"); // ""
        m.insert("less", "\u{e758}"); // ""
        m.insert("lhs", "\u{e777}"); // ""
        m.insert("license", "\u{e60a}"); // ""
        m.insert("lisp", "\u{f671}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("lock", "\u{f023}"); // ""
        m.insert("log", "\u{f18d}"); // ""
        m.insert("lua", "\u{e620}"); // ""
        m.insert("lz", "\u{f410}"); // ""
        m.insert("m3u", "\u{f910}"); // "蘿"
        m.insert("m3u8", "\u{f910}"); // "蘿"
        m.insert("m4a", "\u{f001}"); // ""
        m.insert("m4v", "\u{f008}"); // ""
        m.insert("magnet", "\u{f076}"); // ""
        m.insert("markdown", "\u{f48a}"); // ""
        m.insert("md", "\u{f48a}"); // ""
        m.insert("mjs", "\u{e74e}"); // ""
        m.insert("mkd", "\u{f48a}"); // ""
        m.insert("mkv", "\u{f008}"); // ""
        m.insert("mobi", "\u{e28b}"); // ""
        m.insert("mov", "\u{f008}"); // ""
        m.insert("mp3", "\u{f001}"); // ""
        m.insert("mp4", "\u{f008}"); // ""
        m.insert("msi", "\u{f17a}"); // ""
        m.insert("mustache", "\u{e60f}"); // ""
        m.insert("nix", "\u{f313}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("o", "\u{e624}"); // ""
        m.insert("opus", "\u{f001}"); // ""
        m.insert("ogg", "\u{f001}"); // ""
        m.insert("ogv", "\u{f008}"); // ""
        m.insert("otf", "\u{f031}"); // ""
        m.insert("pdf", "\u{f1c1}"); // ""
        m.insert("pem", "\u{f805}"); // ""
        m.insert("phar", "\u{e608}"); // ""
        m.insert("php", "\u{e608}"); // ""
        m.insert("pkg", "\u{f187}"); // ""
        m.insert("pl", "\u{e769}"); // ""
        m.insert("plist", "\u{f121}"); // ""
        m.insert("pls", "\u{f910}"); // "蘿"
        m.insert("pm", "\u{e769}"); // ""
        m.insert("png", "\u{f1c5}"); // ""
        m.insert("ppt", "\u{f1c4}"); // ""
        m.insert("pptx", "\u{f1c4}"); // ""
        m.insert("procfile", "\u{e21e}"); // ""
        m.insert("properties", "\u{e60b}"); // ""
        m.insert("ps1", "\u{f489}"); // ""
        m.insert("psd", "\u{e7b8}"); // ""
        m.insert("pub", "\u{e60a}"); // ""
        m.insert("pxm", "\u{f1c5}"); // ""
        m.insert("py", "\u{e606}"); // ""
        m.insert("pyc", "\u{e606}"); // ""
        m.insert("r", "\u{fcd2}"); // "ﳒ"
        m.insert("rakefile", "\u{e21e}"); // ""
        m.insert("rar", "\u{f410}"); // ""
        m.insert("razor", "\u{f1fa}"); // ""
        m.insert("rb", "\u{e21e}"); // ""
        m.insert("rdata", "\u{fcd2}"); // "ﳒ"
        m.insert("rdb", "\u{e76d}"); // ""
        m.insert("rdoc", "\u{f48a}"); // ""
        m.insert("rds", "\u{fcd2}"); // "ﳒ"
        m.insert("readme", "\u{f48a}"); // ""
        m.insert("rlib", "\u{e7a8}"); // ""
        m.insert("rmd", "\u{f48a}"); // ""
        m.insert("rpm", "\u{f187}"); // ""
        m.insert("rproj", "\u{fac5}"); // "鉶"
        m.insert("rs", "\u{e7a8}"); // ""
        m.insert("rspec", "\u{e21e}"); // ""
        m.insert("rspec_parallel", "\u{e21e}"); // ""
        m.insert("rspec_status", "\u{e21e}"); // ""
        m.insert("rss", "\u{f09e}"); // ""
        m.insert("rtf", "\u{f15c}"); // ""
        m.insert("ru", "\u{e21e}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""
        m.insert("s", "\u{e614}"); // ""
        m.insert("sass", "\u{e603}"); // ""
        m.insert("scala", "\u{e737}"); // ""
        m.insert("scpt", "\u{f302}"); // ""
        m.insert("scss", "\u{e603}"); // ""
        m.insert("sh", "\u{f489}"); // ""
        m.insert("shell", "\u{f489}"); // ""
        m.insert("sig", "\u{e60a}"); // ""
        m.insert("slim", "\u{e73b}"); // ""
        m.insert("sln", "\u{e70c}"); // ""
        m.insert("so", "\u{e624}"); // ""
        m.insert("sql", "\u{f1c0}"); // ""
        m.insert("sqlite3", "\u{e7c4}"); // ""
        m.insert("srt", "\u{f02d}"); // ""
        m.insert("styl", "\u{e600}"); // ""
        m.insert("stylus", "\u{e600}"); // ""
        m.insert("sub", "\u{f02d}"); // ""
        m.insert("svg", "\u{f1c5}"); // ""
        m.insert("swift", "\u{e755}"); // ""
        m.insert("t", "\u{e769}"); // ""
        m.insert("tar", "\u{f410}"); // ""
        m.insert("tex", "\u{e600}"); // ""
        m.insert("tiff", "\u{f1c5}"); // ""
        m.insert("toml", "\u{e60b}"); // ""
        m.insert("torrent", "\u{f98c}"); // "歷"
        m.insert("ts", "\u{e628}"); // ""
        m.insert("tsx", "\u{e7ba}"); // ""
        m.insert("ttc", "\u{f031}"); // ""
        m.insert("ttf", "\u{f031}"); // ""
        m.insert("twig", "\u{e61c}"); // ""
        m.insert("txt", "\u{f15c}"); // ""
        m.insert("video", "\u{f008}"); // ""
        m.insert("vim", "\u{e62b}"); // ""
        m.insert("vlc", "\u{f910}"); // "蘿"
        m.insert("vue", "\u{fd42}"); // "﵂"
        m.insert("wav", "\u{f001}"); // ""
        m.insert("webm", "\u{f008}"); // ""
        m.insert("webp", "\u{f1c5}"); // ""
        m.insert("windows", "\u{f17a}"); // ""
        m.insert("wma", "\u{f001}"); // ""
        m.insert("wmv", "\u{f008}"); // ""
        m.insert("wpl", "\u{f910}"); // "蘿"
        m.insert("woff", "\u{f031}"); // ""
        m.insert("woff2", "\u{f031}"); // ""
        m.insert("xbps", "\u{f187}"); // ""
        m.insert("xcf", "\u{f1c5}"); // ""
        m.insert("xhtml", "\u{f121}"); // ""
        m.insert("xls", "\u{f1c3}"); // ""
        m.insert("xlsx", "\u{f1c3}"); // ""
        m.insert("xml", "\u{f121}"); // ""
        m.insert("xul", "\u{f269}"); // ""
        m.insert("xz", "\u{f410}"); // ""
        m.insert("yaml", "\u{e60b}"); // ""
        m.insert("yml", "\u{e60b}"); // ""
        m.insert("zip", "\u{f410}"); // ""
        m.insert("zsh", "\u{f489}"); // ""
        m.insert("zsh-theme", "\u{f489}"); // ""
        m.insert("zshrc", "\u{f489}"); // ""

        m
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

        assert_eq!(icon_str, format!("{}{}", "\u{f016}", icon.icon_separator)); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f5cb}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator)); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f5c1}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator)); // 
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

            let icon = Icons::new(Theme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }
}
