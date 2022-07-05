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
                    .icons_by_name
                    .get(name.file_name().to_lowercase().as_str())
                {
                    icon
                }
                // Use the known extensions
                else if let Some(icon) = name.extension().and_then(|extension| {
                    self.icons_by_extension
                        .get(extension.to_lowercase().as_str())
                }) {
                    icon
                } else {
                    match file_type {
                        FileType::Directory { .. } => self.default_folder_icon,
                        // If a file has no extension and is executable, show an icon.
                        // Except for Windows, it marks everything as an executable.
                        #[cfg(not(windows))]
                        FileType::File { exec: true, .. } => "\u{f489}", // ""
                        _ => self.default_file_icon,
                    }
                }
            }
        };

        format!("{}{}", icon, self.icon_separator)
    }

    fn get_default_icons_by_name() -> HashMap<&'static str, &'static str> {
        // Note: filenames must be lower-case
        HashMap::from([
            (".trash", "\u{f1f8}"),             // ""
            (".atom", "\u{e764}"),              // ""
            (".bash_profile", "\u{e615}"),      // ""
            (".bash_logout", "\u{e615}"),       // ""
            (".bashrc", "\u{f489}"),            // ""
            (".cargo", "\u{e7a8}"),             // ""
            (".clang-format", "\u{e615}"),      // ""
            (".config", "\u{e5fc}"),            // ""
            (".emacs.d", "\u{e779}"),           // ""
            (".doom.d", "\u{e779}"),            // ""
            (".git", "\u{e5fb}"),               // ""
            (".gitattributes", "\u{f1d3}"),     // ""
            (".gitconfig", "\u{f1d3}"),         // ""
            (".github", "\u{e5fd}"),            // ""
            (".gitignore", "\u{f1d3}"),         // ""
            (".gitlab-ci.yml", "\u{f296}"),     // ""
            (".gitmodules", "\u{f1d3}"),        // ""
            (".htaccess", "\u{e615}"),          // ""
            (".htpasswd", "\u{e615}"),          // ""
            (".inputrc", "\u{e615}"),           // ""
            (".node_repl_history", "\u{e718}"), // ""
            (".npm", "\u{e5fa}"),               // ""
            (".python_history", "\u{e606}"),    // ""
            (".release.toml", "\u{e7a8}"),      // ""
            (".rvm", "\u{e21e}"),               // ""
            (".ssh", "\u{f023}"),               // ""
            (".vim", "\u{e62b}"),               // ""
            (".vimrc", "\u{e62b}"),             // ""
            (".viminfo", "\u{e62b}"),           // ""
            (".vscode", "\u{e70c}"),            // ""
            (".xauthority", "\u{e615}"),        // ""
            (".xinitrc", "\u{e615}"),           // ""
            (".xresources", "\u{e615}"),        // ""
            (".zshrc", "\u{f489}"),             // ""
            (".zsh_history", "\u{e615}"),       // ""
            ("a.out", "\u{f489}"),              // ""
            ("authorized_keys", "\u{e60a}"),    // ""
            ("bin", "\u{e5fc}"),                // ""
            ("bspwmrc", "\u{e615}"),            // ""
            ("cargo.toml", "\u{e7a8}"),         // ""
            ("cargo.lock", "\u{e7a8}"),         // ""
            ("changelog", "\u{e609}"),          // ""
            ("composer.json", "\u{e608}"),      // ""
            ("config", "\u{e5fc}"),             // ""
            ("config.ac", "\u{e615}"),          // ""
            ("config.mk", "\u{e615}"),          // ""
            ("config.el", "\u{e779}"),          // ""
            ("custom.el", "\u{e779}"),          // ""
            ("contributing", "\u{e60a}"),       // ""
            ("cron.d", "\u{e5fc}"),             // ""
            ("cron.daily", "\u{e5fc}"),         // ""
            ("cron.hourly", "\u{e5fc}"),        // ""
            ("cron.weekly", "\u{e5fc}"),        // ""
            ("cron.monthly", "\u{e5fc}"),       // ""
            ("crontab", "\u{e615}"),            // ""
            ("crypttab", "\u{e615}"),           // ""
            ("desktop", "\u{f108}"),            // ""
            ("downloads", "\u{f498}"),          // ""
            ("docker-compose.yml", "\u{f308}"), // ""
            ("dockerfile", "\u{f308}"),         // ""
            ("documents", "\u{f02d}"),          // ""
            (".ds_store", "\u{f179}"),          // ""
            ("etc", "\u{e5fc}"),                // ""
            ("favicon.ico", "\u{f005}"),        // ""
            ("fstab", "\u{f1c0}"),              // ""
            ("gitignore_global", "\u{f1d3}"),   // ""
            ("gradle", "\u{e70e}"),             // ""
            ("group", "\u{e615}"),              // ""
            ("gruntfile.coffee", "\u{e611}"),   // ""
            ("gruntfile.js", "\u{e611}"),       // ""
            ("gruntfile.ls", "\u{e611}"),       // ""
            ("gshadow", "\u{e615}"),            // ""
            ("gulpfile.coffee", "\u{e610}"),    // ""
            ("gulpfile.js", "\u{e610}"),        // ""
            ("gulpfile.ls", "\u{e610}"),        // ""
            ("hidden", "\u{f023}"),             // ""
            ("hosts", "\u{f502}"),              // ""
            ("htoprc", "\u{e615}"),             // ""
            ("include", "\u{e5fc}"),            // ""
            ("init.el", "\u{e779}"),            // ""
            ("known_hosts", "\u{e60a}"),        // ""
            ("lib", "\u{f121}"),                // ""
            ("license", "\u{e60a}"),            // ""
            ("license.md", "\u{e60a}"),         // ""
            ("license.txt", "\u{e60a}"),        // ""
            ("localized", "\u{f179}"),          // ""
            ("mail", "\u{f6ef}"),               // ""
            ("makefile", "\u{e615}"),           // ""
            ("makefile.ac", "\u{e615}"),        // ""
            ("music", "\u{f025}"),              // ""
            ("muttrc", "\u{e615}"),             // ""
            ("node_modules", "\u{e5fa}"),       // ""
            ("npmignore", "\u{e71e}"),          // ""
            ("package.json", "\u{e718}"),       // ""
            ("packages.el", "\u{e779}"),        // ""
            ("package-lock.json", "\u{e718}"),  // ""
            ("passwd", "\u{f023}"),             // ""
            ("pictures", "\u{f03e}"),           // ""
            ("profile", "\u{e615}"),            // ""
            ("readme", "\u{e609}"),             // ""
            ("rc.lua", "\u{e615}"),             // ""
            ("rubydoc", "\u{e73b}"),            // ""
            ("robots.txt", "\u{fba7}"),         // "ﮧ"
            ("root", "\u{f023}"),               // ""
            ("shadow", "\u{e615}"),             // ""
            ("shells", "\u{e615}"),             // ""
            ("sudoers", "\u{f023}"),            // ""
            ("sxhkdrc", "\u{e615}"),            // ""
            ("tigrc", "\u{e615}"),              // ""
            ("vagrantfile", "\u{e615}"),        // ""
            ("videos", "\u{f03d}"),             // ""
            ("hostname", "\u{e615}"),           // ""
            ("webpack.config.js", "\u{fc29}"),  // "ﰩ"
            ("xmonad.hs", "\u{e615}"),          // ""
            ("xorg.conf.d", "\u{e5fc}"),        // ""
            ("xbps.d", "\u{e5fc}"),             // ""
        ])
    }

    fn get_default_icons_by_extension() -> HashMap<&'static str, &'static str> {
        // Note: extensions must be lower-case
        HashMap::from([
            ("1", "\u{f02d}"),               // ""
            ("7z", "\u{f410}"),              // ""
            ("a", "\u{e624}"),               // ""
            ("ai", "\u{e7b4}"),              // ""
            ("ape", "\u{f001}"),             // ""
            ("apk", "\u{e70e}"),             // ""
            ("asc", "\u{f023}"),             // ""
            ("asm", "\u{e614}"),             // ""
            ("asp", "\u{f121}"),             // ""
            ("avi", "\u{f008}"),             // ""
            ("avro", "\u{e60b}"),            // ""
            ("awk", "\u{f489}"),             // ""
            ("bash", "\u{f489}"),            // ""
            ("bash_history", "\u{f489}"),    // ""
            ("bash_profile", "\u{f489}"),    // ""
            ("bashrc", "\u{f489}"),          // ""
            ("bat", "\u{f17a}"),             // ""
            ("bin", "\u{f489}"),             // ""
            ("bio", "\u{f910}"),             // "蘿"
            ("bmp", "\u{f1c5}"),             // ""
            ("bz2", "\u{f410}"),             // ""
            ("c", "\u{e61e}"),               // ""
            ("c++", "\u{e61d}"),             // ""
            ("cc", "\u{e61d}"),              // ""
            ("cfg", "\u{e615}"),             // ""
            ("cl", "\u{f671}"),              // ""
            ("class", "\u{e738}"),           // ""
            ("clj", "\u{e768}"),             // ""
            ("cljs", "\u{e76a}"),            // ""
            ("cls", "\u{e600}"),             // ""
            ("coffee", "\u{f0f4}"),          // ""
            ("conf", "\u{e615}"),            // ""
            ("cp", "\u{e61d}"),              // ""
            ("cpp", "\u{e61d}"),             // ""
            ("cs", "\u{f81a}"),              // ""
            ("cshtml", "\u{f1fa}"),          // ""
            ("csproj", "\u{f81a}"),          // ""
            ("csx", "\u{f81a}"),             // ""
            ("csh", "\u{f489}"),             // ""
            ("css", "\u{e749}"),             // ""
            ("csv", "\u{f1c3}"),             // ""
            ("cue", "\u{f001}"),             // ""
            ("cxx", "\u{e61d}"),             // ""
            ("dart", "\u{e798}"),            // ""
            ("db", "\u{f1c0}"),              // ""
            ("deb", "\u{f187}"),             // ""
            ("desktop", "\u{f108}"),         // ""
            ("diff", "\u{e728}"),            // ""
            ("dll", "\u{f17a}"),             // ""
            ("doc", "\u{f1c2}"),             // ""
            ("dockerfile", "\u{f308}"),      // ""
            ("docx", "\u{f1c2}"),            // ""
            ("ds_store", "\u{f179}"),        // ""
            ("dump", "\u{f1c0}"),            // ""
            ("ebook", "\u{e28b}"),           // ""
            ("editorconfig", "\u{e615}"),    // ""
            ("ejs", "\u{e618}"),             // ""
            ("el", "\u{f671}"),              // ""
            ("elc", "\u{f671}"),             // ""
            ("elf", "\u{f489}"),             // ""
            ("elm", "\u{e62c}"),             // ""
            ("env", "\u{f462}"),             // ""
            ("eot", "\u{f031}"),             // ""
            ("epub", "\u{e28a}"),            // ""
            ("erb", "\u{e73b}"),             // ""
            ("erl", "\u{e7b1}"),             // ""
            ("exe", "\u{f17a}"),             // ""
            ("ex", "\u{e62d}"),              // ""
            ("exs", "\u{e62d}"),             // ""
            ("fish", "\u{f489}"),            // ""
            ("flac", "\u{f001}"),            // ""
            ("flv", "\u{f008}"),             // ""
            ("font", "\u{f031}"),            // ""
            ("fpl", "\u{f910}"),             // "蘿"
            ("fs", "\u{e7a7}"),              // ""
            ("fsx", "\u{e7a7}"),             // ""
            ("fsi", "\u{e7a7}"),             // ""
            ("gdoc", "\u{f1c2}"),            // ""
            ("gemfile", "\u{e21e}"),         // ""
            ("gemspec", "\u{e21e}"),         // ""
            ("gform", "\u{f298}"),           // ""
            ("gif", "\u{f1c5}"),             // ""
            ("git", "\u{f1d3}"),             // ""
            ("go", "\u{e627}"),              // ""
            ("gradle", "\u{e70e}"),          // ""
            ("gsheet", "\u{f1c3}"),          // ""
            ("gslides", "\u{f1c4}"),         // ""
            ("guardfile", "\u{e21e}"),       // ""
            ("gz", "\u{f410}"),              // ""
            ("h", "\u{f0fd}"),               // ""
            ("hbs", "\u{e60f}"),             // ""
            ("heic", "\u{f1c5}"),            // ""
            ("heif", "\u{f1c5}"),            // ""
            ("heix", "\u{f1c5}"),            // ""
            ("hpp", "\u{f0fd}"),             // ""
            ("hs", "\u{e777}"),              // ""
            ("htm", "\u{f13b}"),             // ""
            ("html", "\u{f13b}"),            // ""
            ("hxx", "\u{f0fd}"),             // ""
            ("ico", "\u{f1c5}"),             // ""
            ("image", "\u{f1c5}"),           // ""
            ("img", "\u{f1c0}"),             // ""
            ("iml", "\u{e7b5}"),             // ""
            ("ini", "\u{e615}"),             // ""
            ("ipynb", "\u{e606}"),           // ""
            ("iso", "\u{f1c0}"),             // ""
            ("jar", "\u{e738}"),             // ""
            ("java", "\u{e738}"),            // ""
            ("jpeg", "\u{f1c5}"),            // ""
            ("jpg", "\u{f1c5}"),             // ""
            ("js", "\u{e74e}"),              // ""
            ("json", "\u{e60b}"),            // ""
            ("jsx", "\u{e7ba}"),             // ""
            ("jl", "\u{e624}"),              // ""
            ("key", "\u{e60a}"),             // ""
            ("ksh", "\u{f489}"),             // ""
            ("ld", "\u{e624}"),              // ""
            ("ldb", "\u{f1c0}"),             // ""
            ("less", "\u{e758}"),            // ""
            ("lhs", "\u{e777}"),             // ""
            ("license", "\u{e60a}"),         // ""
            ("lisp", "\u{f671}"),            // ""
            ("localized", "\u{f179}"),       // ""
            ("lock", "\u{f023}"),            // ""
            ("log", "\u{f18d}"),             // ""
            ("lua", "\u{e620}"),             // ""
            ("lz", "\u{f410}"),              // ""
            ("m3u", "\u{f910}"),             // "蘿"
            ("m3u8", "\u{f910}"),            // "蘿"
            ("m4a", "\u{f001}"),             // ""
            ("m4v", "\u{f008}"),             // ""
            ("magnet", "\u{f076}"),          // ""
            ("markdown", "\u{e609}"),        // ""
            ("md", "\u{e609}"),              // ""
            ("mjs", "\u{e74e}"),             // ""
            ("mkd", "\u{e609}"),             // ""
            ("mkv", "\u{f008}"),             // ""
            ("mobi", "\u{e28b}"),            // ""
            ("mov", "\u{f008}"),             // ""
            ("mp3", "\u{f001}"),             // ""
            ("mp4", "\u{f008}"),             // ""
            ("msi", "\u{f17a}"),             // ""
            ("mustache", "\u{e60f}"),        // ""
            ("nix", "\u{f313}"),             // ""
            ("npmignore", "\u{e71e}"),       // ""
            ("o", "\u{e624}"),               // ""
            ("opus", "\u{f001}"),            // ""
            ("ogg", "\u{f001}"),             // ""
            ("ogv", "\u{f008}"),             // ""
            ("otf", "\u{f031}"),             // ""
            ("pdf", "\u{f1c1}"),             // ""
            ("pem", "\u{f805}"),             // ""
            ("phar", "\u{e608}"),            // ""
            ("php", "\u{e608}"),             // ""
            ("pkg", "\u{f187}"),             // ""
            ("pl", "\u{e769}"),              // ""
            ("plist", "\u{f302}"),           // ""
            ("pls", "\u{f910}"),             // "蘿"
            ("pm", "\u{e769}"),              // ""
            ("png", "\u{f1c5}"),             // ""
            ("ppt", "\u{f1c4}"),             // ""
            ("pptx", "\u{f1c4}"),            // ""
            ("procfile", "\u{e21e}"),        // ""
            ("properties", "\u{e60b}"),      // ""
            ("ps1", "\u{f489}"),             // ""
            ("psd", "\u{e7b8}"),             // ""
            ("pub", "\u{e60a}"),             // ""
            ("pxm", "\u{f1c5}"),             // ""
            ("py", "\u{e606}"),              // ""
            ("pyc", "\u{e606}"),             // ""
            ("r", "\u{fcd2}"),               // "ﳒ"
            ("rakefile", "\u{e21e}"),        // ""
            ("rar", "\u{f410}"),             // ""
            ("razor", "\u{f1fa}"),           // ""
            ("rb", "\u{e21e}"),              // ""
            ("rdata", "\u{fcd2}"),           // "ﳒ"
            ("rdb", "\u{e76d}"),             // ""
            ("rdoc", "\u{e609}"),            // ""
            ("rds", "\u{fcd2}"),             // "ﳒ"
            ("readme", "\u{e609}"),          // ""
            ("rlib", "\u{e7a8}"),            // ""
            ("rmd", "\u{e609}"),             // ""
            ("rpm", "\u{f187}"),             // ""
            ("rproj", "\u{fac5}"),           // "鉶"
            ("rs", "\u{e7a8}"),              // ""
            ("rspec", "\u{e21e}"),           // ""
            ("rspec_parallel", "\u{e21e}"),  // ""
            ("rspec_status", "\u{e21e}"),    // ""
            ("rss", "\u{f09e}"),             // ""
            ("rtf", "\u{f15c}"),             // ""
            ("ru", "\u{e21e}"),              // ""
            ("rubydoc", "\u{e73b}"),         // ""
            ("s", "\u{e614}"),               // ""
            ("sass", "\u{e603}"),            // ""
            ("scala", "\u{e737}"),           // ""
            ("scpt", "\u{f302}"),            // ""
            ("scss", "\u{e603}"),            // ""
            ("sh", "\u{f489}"),              // ""
            ("shell", "\u{f489}"),           // ""
            ("sig", "\u{e60a}"),             // ""
            ("slim", "\u{e73b}"),            // ""
            ("sln", "\u{e70c}"),             // ""
            ("so", "\u{e624}"),              // ""
            ("sql", "\u{f1c0}"),             // ""
            ("sqlite3", "\u{e7c4}"),         // ""
            ("srt", "\u{f02d}"),             // ""
            ("styl", "\u{e600}"),            // ""
            ("stylus", "\u{e600}"),          // ""
            ("sub", "\u{f02d}"),             // ""
            ("sublime-package", "\u{e7aa}"), // ""
            ("sublime-session", "\u{e7aa}"), // ""
            ("svg", "\u{f1c5}"),             // ""
            ("swift", "\u{e755}"),           // ""
            ("swp", "\u{e62b}"),             // ""
            ("sym", "\u{e624}"),             // ""
            ("t", "\u{e769}"),               // ""
            ("tar", "\u{f410}"),             // ""
            ("tex", "\u{e600}"),             // ""
            ("tgz", "\u{f410}"),             // ""
            ("tiff", "\u{f1c5}"),            // ""
            ("toml", "\u{e60b}"),            // ""
            ("torrent", "\u{f98c}"),         // "歷"
            ("ts", "\u{e628}"),              // ""
            ("tsx", "\u{e7ba}"),             // ""
            ("ttc", "\u{f031}"),             // ""
            ("ttf", "\u{f031}"),             // ""
            ("twig", "\u{e61c}"),            // ""
            ("txt", "\u{f15c}"),             // ""
            ("video", "\u{f008}"),           // ""
            ("vim", "\u{e62b}"),             // ""
            ("vlc", "\u{f910}"),             // "蘿"
            ("vue", "\u{fd42}"),             // "﵂"
            ("wav", "\u{f001}"),             // ""
            ("webm", "\u{f008}"),            // ""
            ("webp", "\u{f1c5}"),            // ""
            ("windows", "\u{f17a}"),         // ""
            ("wma", "\u{f001}"),             // ""
            ("wmv", "\u{f008}"),             // ""
            ("wpl", "\u{f910}"),             // "蘿"
            ("woff", "\u{f031}"),            // ""
            ("woff2", "\u{f031}"),           // ""
            ("xbps", "\u{f187}"),            // ""
            ("xcf", "\u{f1c5}"),             // ""
            ("xls", "\u{f1c3}"),             // ""
            ("xlsx", "\u{f1c3}"),            // ""
            ("xml", "\u{f121}"),             // ""
            ("xul", "\u{f269}"),             // ""
            ("xz", "\u{f410}"),              // ""
            ("yaml", "\u{e60b}"),            // ""
            ("yml", "\u{e60b}"),             // ""
            ("zip", "\u{f410}"),             // ""
            ("zsh", "\u{f489}"),             // ""
            ("zsh-theme", "\u{f489}"),       // ""
            ("zshrc", "\u{f489}"),           // ""
        ])
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
