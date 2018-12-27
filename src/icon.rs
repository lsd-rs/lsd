use meta::{FileType, Name};
use std::collections::HashMap;

pub struct Icons {
    display_icons: bool,
    icons_by_name: HashMap<&'static str, &'static str>,
    icons_by_extension: HashMap<&'static str, &'static str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
    NoIcon,
    Default,
}

const ICON_SPACE: &str = "  ";

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(theme: Theme) -> Self {
        Self {
            display_icons: theme == Theme::Default,
            icons_by_name: Self::get_default_icons_by_name(),
            icons_by_extension: Self::get_default_icons_by_extension(),
        }
    }

    pub fn get(&self, name: &Name) -> String {
        if !self.display_icons {
            return String::new();
        }

        let mut res = String::with_capacity(4 + ICON_SPACE.len()); // 4 == max icon size

        // Check directory.
        if name.file_type() == FileType::Directory {
            res += "\u{f115}"; // 
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
        res += "\u{f016}"; // 
        res += ICON_SPACE;
        res
    }

    fn get_default_icons_by_name() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        m.insert(".Trash", "\u{f1f8}"); // ""
        m.insert(".atom", "\u{e764}"); // ""
        m.insert(".bashprofile", "\u{e615}"); // ""
        m.insert(".bashrc", "\u{f489}"); // ""
        m.insert(".git", "\u{f1d3}"); // ""
        m.insert(".gitconfig", "\u{f1d3}"); // ""
        m.insert(".github", "\u{f408}"); // ""
        m.insert(".gitignore", "\u{f1d3}"); // ""
        m.insert(".rvm", "\u{e21e}"); // ""
        m.insert(".vimrc", "\u{e62b}"); // ""
        m.insert(".vscode", "\u{e70c}"); // ""
        m.insert(".zshrc", "\u{f489}"); // ""
        m.insert("bin", "\u{e5fc}"); // ""
        m.insert("config", "\u{e5fc}"); // ""
        m.insert("docker-compose.yml", "\u{f308}"); // ""
        m.insert("dockerfile", "\u{f308}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
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
        m.insert("lib", "\u{f121}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("node_modules", "\u{e718}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""
        m.insert("yarn.lock", "\u{e718}"); // ""

        m
    }

    fn get_default_icons_by_extension() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        m.insert("apk", "\u{e70e}"); // ""
        m.insert("apk", "\u{e70e}"); // ""
        m.insert("avi", "\u{f03d}"); // ""
        m.insert("avro", "\u{e60b}"); // ""
        m.insert("awk", "\u{f489}"); // ""
        m.insert("bash", "\u{f489}"); // ""
        m.insert("bash_history", "\u{f489}"); // ""
        m.insert("bash_profile", "\u{f489}"); // ""
        m.insert("bashrc", "\u{f489}"); // ""
        m.insert("bat", "\u{f17a}"); // ""
        m.insert("bmp", "\u{f1c5}"); // ""
        m.insert("c", "\u{e61e}"); // ""
        m.insert("c++", "\u{e61d}"); // ""
        m.insert("cc", "\u{e61d}"); // ""
        m.insert("cfg", "\u{e615}"); // ""
        m.insert("clj", "\u{e768}"); // ""
        m.insert("cljs", "\u{e76a}"); // ""
        m.insert("cls", "\u{e600}"); // ""
        m.insert("coffee", "\u{f0f4}"); // ""
        m.insert("conf", "\u{e615}"); // ""
        m.insert("cp", "\u{e61d}"); // ""
        m.insert("cpp", "\u{e61d}"); // ""
        m.insert("csh", "\u{f489}"); // ""
        m.insert("css", "\u{e749}"); // ""
        m.insert("csv", "\u{f1c3}"); // ""
        m.insert("cxx", "\u{e61d}"); // ""
        m.insert("d", "\u{e7af}"); // ""
        m.insert("dart", "\u{e798}"); // ""
        m.insert("db", "\u{f1c0}"); // ""
        m.insert("diff", "\u{f440}"); // ""
        m.insert("doc", "\u{f1c2}"); // ""
        m.insert("docx", "\u{f1c2}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
        m.insert("dump", "\u{f1c0}"); // ""
        m.insert("ebook", "\u{e28b}"); // ""
        m.insert("editorconfig", "\u{e615}"); // ""
        m.insert("ejs", "\u{e618}"); // ""
        m.insert("env", "\u{f462}"); // ""
        m.insert("eot", "\u{f031}"); // ""
        m.insert("epub", "\u{e28a}"); // ""
        m.insert("erb", "\u{e73b}"); // ""
        m.insert("erl", "\u{e7b1}"); // ""
        m.insert("exe", "\u{f17a}"); // ""
        m.insert("fish", "\u{f489}"); // ""
        m.insert("flac", "\u{f001}"); // ""
        m.insert("flv", "\u{f03d}"); // ""
        m.insert("font", "\u{f031}"); // ""
        m.insert("gdoc", "\u{f1c2}"); // ""
        m.insert("gemfile", "\u{e21e}"); // ""
        m.insert("gemspec", "\u{e21e}"); // ""
        m.insert("gform", "\u{f298}"); // ""
        m.insert("gif", "\u{f1c5}"); // ""
        m.insert("git", "\u{f1d3}"); // ""
        m.insert("go", "\u{e626}"); // ""
        m.insert("gradle", "\u{e70e}"); // ""
        m.insert("gsheet", "\u{f1c3}"); // ""
        m.insert("gslides", "\u{f1c4}"); // ""
        m.insert("guardfile", "\u{e21e}"); // ""
        m.insert("gz", "\u{f410}"); // ""
        m.insert("h", "\u{f0fd}"); // ""
        m.insert("hbs", "\u{e60f}"); // ""
        m.insert("hpp", "\u{f0fd}"); // ""
        m.insert("hs", "\u{e777}"); // ""
        m.insert("htm", "\u{f13b}"); // ""
        m.insert("html", "\u{f13b}"); // ""
        m.insert("hxx", "\u{f0fd}"); // ""
        m.insert("ico", "\u{f1c5}"); // ""
        m.insert("image", "\u{f1c5}"); // ""
        m.insert("iml", "\u{e7b5}"); // ""
        m.insert("ini", "\u{f17a}"); // ""
        m.insert("ipynb", "\u{e606}"); // ""
        m.insert("jar", "\u{e204}"); // ""
        m.insert("java", "\u{e204}"); // ""
        m.insert("jpeg", "\u{f1c5}"); // ""
        m.insert("jpg", "\u{f1c5}"); // ""
        m.insert("js", "\u{e74e}"); // ""
        m.insert("json", "\u{e60b}"); // ""
        m.insert("jsx", "\u{e7ba}"); // ""
        m.insert("ksh", "\u{f489}"); // ""
        m.insert("less", "\u{e758}"); // ""
        m.insert("lhs", "\u{e777}"); // ""
        m.insert("license", "\u{f48a}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("lock", "\u{e21e}"); // ""
        m.insert("log", "\u{f18d}"); // ""
        m.insert("lua", "\u{e620}"); // ""
        m.insert("m4a", "\u{f001}"); // ""
        m.insert("markdown", "\u{f48a}"); // ""
        m.insert("md", "\u{f48a}"); // ""
        m.insert("mkd", "\u{f48a}"); // ""
        m.insert("mkv", "\u{f03d}"); // ""
        m.insert("mobi", "\u{e28b}"); // ""
        m.insert("mov", "\u{f03d}"); // ""
        m.insert("mp3", "\u{f001}"); // ""
        m.insert("mp4", "\u{f03d}"); // ""
        m.insert("mustache", "\u{e60f}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("ogg", "\u{f001}"); // ""
        m.insert("ogv", "\u{f03d}"); // ""
        m.insert("otf", "\u{f031}"); // ""
        m.insert("pdf", "\u{f1c1}"); // ""
        m.insert("php", "\u{e73d}"); // ""
        m.insert("pl", "\u{e769}"); // ""
        m.insert("png", "\u{f1c5}"); // ""
        m.insert("ppt", "\u{f1c4}"); // ""
        m.insert("pptx", "\u{f1c4}"); // ""
        m.insert("procfile", "\u{e21e}"); // ""
        m.insert("properties", "\u{e60b}"); // ""
        m.insert("ps1", "\u{f489}"); // ""
        m.insert("psd", "\u{e7b8}"); // ""
        m.insert("pxm", "\u{f1c5}"); // ""
        m.insert("py", "\u{e606}"); // ""
        m.insert("pyc", "\u{e606}"); // ""
        m.insert("r", "\u{f25d}"); // ""
        m.insert("rakefile", "\u{e21e}"); // ""
        m.insert("rar", "\u{f410}"); // ""
        m.insert("rb", "\u{e21e}"); // ""
        m.insert("rdata", "\u{f25d}"); // ""
        m.insert("rdb", "\u{e76d}"); // ""
        m.insert("rdoc", "\u{f48a}"); // ""
        m.insert("rds", "\u{f25d}"); // ""
        m.insert("readme", "\u{f48a}"); // ""
        m.insert("rlib", "\u{e7a8}"); // ""
        m.insert("rmd", "\u{f48a}"); // ""
        m.insert("rs", "\u{e7a8}"); // ""
        m.insert("rspec", "\u{e21e}"); // ""
        m.insert("rspec_parallel", "\u{e21e}"); // ""
        m.insert("rspec_status", "\u{e21e}"); // ""
        m.insert("rss", "\u{f09e}"); // ""
        m.insert("ru", "\u{e21e}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""
        m.insert("sass", "\u{e603}"); // ""
        m.insert("scala", "\u{e737}"); // ""
        m.insert("scss", "\u{e749}"); // ""
        m.insert("sh", "\u{f489}"); // ""
        m.insert("shell", "\u{f489}"); // ""
        m.insert("slim", "\u{e73b}"); // ""
        m.insert("sql", "\u{f1c0}"); // ""
        m.insert("sqlite3", "\u{e7c4}"); // ""
        m.insert("styl", "\u{e600}"); // ""
        m.insert("stylus", "\u{e600}"); // ""
        m.insert("svg", "\u{f1c5}"); // ""
        m.insert("swift", "\u{e755}"); // ""
        m.insert("tar", "\u{f410}"); // ""
        m.insert("tex", "\u{e600}"); // ""
        m.insert("tiff", "\u{f1c5}"); // ""
        m.insert("ts", "\u{e628}"); // ""
        m.insert("tsx", "\u{e7ba}"); // ""
        m.insert("ttf", "\u{f031}"); // ""
        m.insert("twig", "\u{e61c}"); // ""
        m.insert("txt", "\u{f15c}"); // ""
        m.insert("video", "\u{f03d}"); // ""
        m.insert("vim", "\u{e62b}"); // ""
        m.insert("vue", "\u{fd42}"); // "﵂"
        m.insert("wav", "\u{f001}"); // ""
        m.insert("webm", "\u{f03d}"); // ""
        m.insert("webp", "\u{f1c5}"); // ""
        m.insert("windows", "\u{f17a}"); // ""
        m.insert("woff", "\u{f031}"); // ""
        m.insert("woff2", "\u{f031}"); // ""
        m.insert("xls", "\u{f1c3}"); // ""
        m.insert("xlsx", "\u{f1c3}"); // ""
        m.insert("xml", "\u{e619}"); // ""
        m.insert("xul", "\u{e619}"); // ""
        m.insert("yaml", "\u{f481}"); // ""
        m.insert("yml", "\u{f481}"); // ""
        m.insert("zip", "\u{f410}"); // ""
        m.insert("zsh", "\u{f489}"); // ""
        m.insert("zsh-theme", "\u{f489}"); // ""
        m.insert("zshrc", "\u{f489}"); // ""

        m
    }
}

#[cfg(test)]
mod test {
    use super::{Icons, Theme, ICON_SPACE};
    use meta::{FileType, Name, Permissions};
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
        let icon = Icons::new(Theme::Default);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f016}", ICON_SPACE)); // 
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Default);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = TempDir::new("test_file_type.rs").expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = file_path.metadata().expect("failed to get metas");

        let file_type = FileType::new(&meta, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        let icon = Icons::new(Theme::Default);
        let icon = icon.get(&name);

        assert_eq!(icon, format!("{}{}", "\u{f115}", ICON_SPACE)); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (file_name, file_icon) in &Icons::get_default_icons_by_name() {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = file_path.metadata().expect("failed to get metas");

            let file_type = FileType::new(&meta, &Permissions::from(&meta));
            let name = Name::new(&file_path, file_type);
            let icon = Icons::new(Theme::Default);
            let icon = icon.get(&name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = TempDir::new("test_file_type").expect("failed to create temp dir");

        for (ext, file_icon) in &Icons::get_default_icons_by_extension() {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = file_path.metadata().expect("failed to get metas");

            let file_type = FileType::new(&meta, &Permissions::from(&meta));
            let name = Name::new(&file_path, file_type);
            let icon = Icons::new(Theme::Default);
            let icon = icon.get(&name);

            assert_eq!(icon, format!("{}{}", file_icon, ICON_SPACE));
        }
    }
}
