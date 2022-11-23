use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct IconTheme {
    pub name: HashMap<String, String>,
    pub extension: HashMap<String, String>,
    pub filetype: ByType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ByType {
    pub dir: String,
    pub file: String,
    pub pipe: String,
    pub socket: String,
    pub executable: String,
    pub device_char: String,
    pub device_block: String,
    pub special: String,
    pub symlink_dir: String,
    pub symlink_file: String,
}

impl Default for IconTheme {
    fn default() -> Self {
        IconTheme {
            name: Self::get_default_icons_by_name(),
            extension: Self::get_default_icons_by_extension(),
            filetype: ByType::default(),
        }
    }
}

impl Default for ByType {
    fn default() -> ByType {
        ByType {
            dir: "\u{f115}".into(),          // 
            file: "\u{f016}".into(),         // 
            pipe: "\u{f731}".into(),         // 
            socket: "\u{f6a7}".into(),       // 
            executable: "\u{f489}".into(),   // 
            symlink_dir: "\u{f482}".into(),  // 
            symlink_file: "\u{f481}".into(), // 
            device_char: "\u{e601}".into(),  // 
            device_block: "\u{fc29}".into(), // ﰩ
            special: "\u{f2dc}".into(),      // 
        }
    }
}

impl ByType {
    pub fn unicode() -> Self {
        ByType {
            dir: "\u{1f4c2}".into(),
            file: "\u{1f4c4}".into(),
            pipe: "\u{1f4e9}".into(),
            socket: "\u{1f4ec}".into(),
            executable: "\u{f40d}".into(),
            symlink_dir: "\u{1f5c2}".into(),
            symlink_file: "\u{1f516}".into(),
            device_char: "\u{1f5a8}".into(),
            device_block: "\u{1f4bd}".into(),
            special: "\u{1f4df}".into(),
        }
    }
}

impl IconTheme {
    pub fn unicode() -> Self {
        IconTheme {
            name: HashMap::new(),
            extension: HashMap::new(),
            filetype: ByType::unicode(),
        }
    }

    // pub only for testing in icons.rs
    pub fn get_default_icons_by_name() -> HashMap<String, String> {
        // Note: filenames must be lower-case
        [
            ("a.out", "\u{f489}"),              // ""
            ("api", "\u{f98c}"),                // "歷"
            (".atom", "\u{e764}"),              // ""
            ("authorized_keys", "\u{e60a}"),    // ""
            ("backups", "\u{f56e}"),            // ""
            (".bash_logout", "\u{e615}"),       // ""
            (".bash_profile", "\u{e615}"),      // ""
            (".bashrc", "\u{f489}"),            // ""
            ("bin", "\u{e5fc}"),                // ""
            (".bpython_history", "\u{e606}"),   // ""
            ("bspwmrc", "\u{e615}"),            // ""
            ("cargo.lock", "\u{e7a8}"),         // ""
            ("cargo.toml", "\u{e7a8}"),         // ""
            (".cargo", "\u{e7a8}"),             // ""
            ("changelog", "\u{e609}"),          // ""
            (".clang-format", "\u{e615}"),      // ""
            ("composer.json", "\u{e608}"),      // ""
            ("conf.d", "\u{e5fc}"),             // ""
            ("config.ac", "\u{e615}"),          // ""
            ("config.el", "\u{e779}"),          // ""
            ("config.mk", "\u{e615}"),          // ""
            (".config", "\u{e5fc}"),            // ""
            ("config", "\u{e5fc}"),             // ""
            ("contributing", "\u{e60a}"),       // ""
            ("copyright", "\u{e60a}"),          // ""
            ("cron.daily", "\u{e5fc}"),         // ""
            ("cron.d", "\u{e5fc}"),             // ""
            ("cron.hourly", "\u{e5fc}"),        // ""
            ("cron.monthly", "\u{e5fc}"),       // ""
            ("crontab", "\u{e615}"),            // ""
            ("cron.weekly", "\u{e5fc}"),        // ""
            ("crypttab", "\u{e615}"),           // ""
            ("css", "\u{e749}"),                // ""
            ("custom.el", "\u{e779}"),          // ""
            (".dbus", "\u{f013}"),              // ""
            ("desktop", "\u{f108}"),            // ""
            ("docker-compose.yml", "\u{f308}"), // ""
            ("dockerfile", "\u{f308}"),         // ""
            ("doc", "\u{f02d}"),                // ""
            ("documents", "\u{f02d}"),          // ""
            (".doom.d", "\u{e779}"),            // ""
            ("downloads", "\u{f498}"),          // ""
            (".ds_store", "\u{f179}"),          // ""
            (".editorconfig", "\u{e615}"),      // ""
            (".emacs.d", "\u{e779}"),           // ""
            (".env", "\u{f462}"),               // ""
            (".eslintrc.json", "\u{f462}"),     // ""
            (".eslintrc.js", "\u{f462}"),       // ""
            (".eslintrc.yml", "\u{f462}"),      // ""
            ("etc", "\u{e5fc}"),                // ""
            ("favicon.ico", "\u{f005}"),        // ""
            ("favicons", "\u{f005}"),           // ""
            ("fstab", "\u{f1c0}"),              // ""
            (".gitattributes", "\u{f1d3}"),     // ""
            (".gitconfig", "\u{f1d3}"),         // ""
            (".git-credentials", "\u{e60a}"),   // ""
            (".github", "\u{e5fd}"),            // ""
            ("gitignore_global", "\u{f1d3}"),   // ""
            (".gitignore", "\u{f1d3}"),         // ""
            (".gitlab-ci.yml", "\u{f296}"),     // ""
            (".gitmodules", "\u{f1d3}"),        // ""
            (".git", "\u{e5fb}"),               // ""
            (".gnupg", "\u{f023}"),             // ""
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
            ("home", "\u{f015}"),               // ""
            ("hostname", "\u{e615}"),           // ""
            ("hosts", "\u{f502}"),              // ""
            (".htaccess", "\u{e615}"),          // ""
            ("htoprc", "\u{e615}"),             // ""
            (".htpasswd", "\u{e615}"),          // ""
            (".idlerc", "\u{e235}"),            // ""
            ("img", "\u{f1c5}"),                // ""
            ("include", "\u{e5fc}"),            // ""
            ("init.el", "\u{e779}"),            // ""
            (".inputrc", "\u{e615}"),           // ""
            ("inputrc", "\u{e615}"),            // ""
            ("js", "\u{e74e}"),                 // ""
            (".jupyter", "\u{e606}"),           // ""
            ("kbuild", "\u{e615}"),             // ""
            ("kconfig", "\u{e615}"),            // ""
            ("known_hosts", "\u{e60a}"),        // ""
            (".kshrc", "\u{f489}"),             // ""
            ("lib64", "\u{f121}"),              // ""
            ("lib", "\u{f121}"),                // ""
            ("license.md", "\u{e60a}"),         // ""
            ("licenses", "\u{e60a}"),           // ""
            ("license.txt", "\u{e60a}"),        // ""
            ("license", "\u{e60a}"),            // ""
            ("localized", "\u{f179}"),          // ""
            ("lsb-release", "\u{e615}"),        // ""
            (".lynxrc", "\u{e615}"),            // ""
            (".mailcap", "\u{f6ef}"),           // ""
            ("mail", "\u{f6ef}"),               // ""
            ("maintainers", "\u{e60a}"),        // ""
            ("makefile.ac", "\u{e615}"),        // ""
            ("makefile", "\u{e615}"),           // ""
            ("manifest", "\u{f292}"),           // ""
            ("metadata", "\u{e5fc}"),           // ""
            ("metadata.xml", "\u{f462}"),       // ""
            ("mime.types", "\u{fb44}"),         // "פּ"
            ("module.symvers", "\u{f471}"),     // ""
            (".mozilla", "\u{e786}"),           // ""
            ("music", "\u{f025}"),              // ""
            ("muttrc", "\u{e615}"),             // ""
            (".mutt", "\u{e615}"),              // ""
            ("netlify.toml", "\u{f233}"),       // ""
            ("node_modules", "\u{e5fa}"),       // ""
            (".node_repl_history", "\u{e718}"), // ""
            ("npmignore", "\u{e71e}"),          // ""
            (".npm", "\u{e5fa}"),               // ""
            ("nvim", "\u{e62b}"),               // ""
            ("os-release", "\u{e615}"),         // ""
            ("package.json", "\u{e718}"),       // ""
            ("package-lock.json", "\u{e718}"),  // ""
            ("packages.el", "\u{e779}"),        // ""
            ("passwd", "\u{f023}"),             // ""
            ("pictures", "\u{f03e}"),           // ""
            ("pkgbuild", "\u{f303}"),           // ""
            (".pki", "\u{f023}"),               // ""
            ("portage", "\u{e5fc}"),            // ""
            ("profile", "\u{e615}"),            // ""
            (".profile", "\u{f68c}"),           // ""
            ("public", "\u{f415}"),             // ""
            ("__pycache__", "\u{f81f}"),        // ""
            (".python_history", "\u{e606}"),    // ""
            ("rc.lua", "\u{e615}"),             // ""
            ("readme", "\u{e609}"),             // ""
            (".release.toml", "\u{e7a8}"),      // ""
            ("requirements.txt", "\u{f81f}"),   // ""
            ("robots.txt", "\u{fba7}"),         // "ﮧ"
            ("root", "\u{f023}"),               // ""
            ("rubydoc", "\u{e73b}"),            // ""
            ("runtime.txt", "\u{f81f}"),        // ""
            (".rustup", "\u{e7a8}"),            // ""
            (".rvm", "\u{e21e}"),               // ""
            ("sass", "\u{e603}"),               // ""
            ("sbin", "\u{e5fc}"),               // ""
            ("scripts", "\u{f489}"),            // ""
            ("scss", "\u{e603}"),               // ""
            ("shadow", "\u{e615}"),             // ""
            ("share", "\u{f064}"),              // ""
            (".shellcheckrc", "\u{e615}"),      // ""
            ("shells", "\u{e615}"),             // ""
            (".sqlite_history", "\u{e7c4}"),    // ""
            ("src", "\u{f121}"),                // ""
            (".ssh", "\u{f023}"),               // ""
            ("styles", "\u{e749}"),             // ""
            ("sudoers", "\u{f023}"),            // ""
            ("sxhkdrc", "\u{e615}"),            // ""
            ("tigrc", "\u{e615}"),              // ""
            ("tox.ini", "\u{f81f}"),            // ""
            (".trash", "\u{f1f8}"),             // ""
            ("ts", "\u{e628}"),                 // ""
            ("unlicense", "\u{e60a}"),          // ""
            ("url", "\u{f0ac}"),                // ""
            ("user-dirs.dirs", "\u{e5fc}"),     // ""
            ("vagrantfile", "\u{e615}"),        // ""
            ("venv", "\u{f81f}"),               // ""
            ("videos", "\u{f03d}"),             // ""
            (".viminfo", "\u{e62b}"),           // ""
            (".vimrc", "\u{e62b}"),             // ""
            ("vimrc", "\u{e62b}"),              // ""
            (".vim", "\u{e62b}"),               // ""
            ("vim", "\u{e62b}"),                // ""
            (".vscode", "\u{e70c}"),            // ""
            ("webpack.config.js", "\u{fc29}"),  // "ﰩ"
            (".wgetrc", "\u{e615}"),            // ""
            ("wgetrc", "\u{e615}"),             // ""
            (".xauthority", "\u{e615}"),        // ""
            (".Xauthority", "\u{e615}"),        // ""
            ("xbps.d", "\u{e5fc}"),             // ""
            (".xinitrc", "\u{e615}"),           // ""
            (".xmodmap", "\u{e615}"),           // ""
            (".Xmodmap", "\u{e615}"),           // ""
            ("xmonad.hs", "\u{e615}"),          // ""
            ("xorg.conf.d", "\u{e5fc}"),        // ""
            (".xprofile", "\u{e615}"),          // ""
            (".Xprofile", "\u{e615}"),          // ""
            (".xresources", "\u{e615}"),        // ""
            ("zathurarc", "\u{e615}"),          // ""
            (".zsh_history", "\u{e615}"),       // ""
            (".zshrc", "\u{f489}"),             // ""
        ]
        .iter()
        .map(|&s| (s.0.to_owned(), s.1.to_owned()))
        .collect::<HashMap<_, _>>()
    }

    // pub only for testing in icons.rs
    pub fn get_default_icons_by_extension() -> HashMap<String, String> {
        // Note: extensions must be lower-case
        [
            ("1", "\u{f02d}"),               // ""
            ("2", "\u{f02d}"),               // ""
            ("3", "\u{f02d}"),               // ""
            ("4", "\u{f02d}"),               // ""
            ("5", "\u{f02d}"),               // ""
            ("6", "\u{f02d}"),               // ""
            ("7", "\u{f02d}"),               // ""
            ("7z", "\u{f410}"),              // ""
            ("8", "\u{f02d}"),               // ""
            ("ai", "\u{e7b4}"),              // ""
            ("ape", "\u{f001}"),             // ""
            ("apk", "\u{e70e}"),             // ""
            ("asc", "\u{f023}"),             // ""
            ("asm", "\u{f471}"),             // ""
            ("asp", "\u{f121}"),             // ""
            ("a", "\u{e624}"),               // ""
            ("avi", "\u{f008}"),             // ""
            ("avro", "\u{e60b}"),            // ""
            ("awk", "\u{f489}"),             // ""
            ("bak", "\u{f56e}"),             // ""
            ("bash_history", "\u{f489}"),    // ""
            ("bash_profile", "\u{f489}"),    // ""
            ("bashrc", "\u{f489}"),          // ""
            ("bash", "\u{f489}"),            // ""
            ("bat", "\u{f17a}"),             // ""
            ("bin", "\u{f489}"),             // ""
            ("bio", "\u{f910}"),             // "蘿"
            ("bmp", "\u{f1c5}"),             // ""
            ("bz2", "\u{f410}"),             // ""
            ("cc", "\u{e61d}"),              // ""
            ("cfg", "\u{e615}"),             // ""
            ("class", "\u{e738}"),           // ""
            ("cljs", "\u{e76a}"),            // ""
            ("clj", "\u{e768}"),             // ""
            ("cls", "\u{e600}"),             // ""
            ("cl", "\u{f671}"),              // ""
            ("coffee", "\u{f0f4}"),          // ""
            ("conf", "\u{e615}"),            // ""
            ("cpp", "\u{e61d}"),             // ""
            ("cp", "\u{e61d}"),              // ""
            ("cshtml", "\u{f1fa}"),          // ""
            ("csh", "\u{f489}"),             // ""
            ("csproj", "\u{f81a}"),          // ""
            ("css", "\u{e749}"),             // ""
            ("cs", "\u{f81a}"),              // ""
            ("csv", "\u{f1c3}"),             // ""
            ("csx", "\u{f81a}"),             // ""
            ("c++", "\u{e61d}"),             // ""
            ("c", "\u{e61e}"),               // ""
            ("cue", "\u{f001}"),             // ""
            ("cxx", "\u{e61d}"),             // ""
            ("dart", "\u{e798}"),            // ""
            ("dat", "\u{f1c0}"),             // ""
            ("db", "\u{f1c0}"),              // ""
            ("deb", "\u{f187}"),             // ""
            ("desktop", "\u{f108}"),         // ""
            ("diff", "\u{e728}"),            // ""
            ("dll", "\u{f17a}"),             // ""
            ("dockerfile", "\u{f308}"),      // ""
            ("doc", "\u{f1c2}"),             // ""
            ("docx", "\u{f1c2}"),            // ""
            ("ds_store", "\u{f179}"),        // ""
            ("dump", "\u{f1c0}"),            // ""
            ("ebook", "\u{e28b}"),           // ""
            ("ebuild", "\u{f30d}"),          // ""
            ("eclass", "\u{f30d}"),          // ""
            ("editorconfig", "\u{e615}"),    // ""
            ("ejs", "\u{e618}"),             // ""
            ("elc", "\u{f671}"),             // ""
            ("elf", "\u{f489}"),             // ""
            ("elm", "\u{e62c}"),             // ""
            ("el", "\u{f671}"),              // ""
            ("env", "\u{f462}"),             // ""
            ("eot", "\u{f031}"),             // ""
            ("epub", "\u{e28a}"),            // ""
            ("erb", "\u{e73b}"),             // ""
            ("erl", "\u{e7b1}"),             // ""
            ("exe", "\u{f17a}"),             // ""
            ("exs", "\u{e62d}"),             // ""
            ("ex", "\u{e62d}"),              // ""
            ("fish", "\u{f489}"),            // ""
            ("flac", "\u{f001}"),            // ""
            ("flv", "\u{f008}"),             // ""
            ("font", "\u{f031}"),            // ""
            ("fpl", "\u{f910}"),             // "蘿"
            ("fsi", "\u{e7a7}"),             // ""
            ("fs", "\u{e7a7}"),              // ""
            ("fsx", "\u{e7a7}"),             // ""
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
            ("hbs", "\u{e60f}"),             // ""
            ("heic", "\u{f1c5}"),            // ""
            ("heif", "\u{f1c5}"),            // ""
            ("heix", "\u{f1c5}"),            // ""
            ("hh", "\u{f0fd}"),              // ""
            ("hpp", "\u{f0fd}"),             // ""
            ("hs", "\u{e777}"),              // ""
            ("html", "\u{f13b}"),            // ""
            ("htm", "\u{f13b}"),             // ""
            ("h", "\u{f0fd}"),               // ""
            ("hxx", "\u{f0fd}"),             // ""
            ("ico", "\u{f1c5}"),             // ""
            ("image", "\u{f1c5}"),           // ""
            ("img", "\u{f1c0}"),             // ""
            ("iml", "\u{e7b5}"),             // ""
            ("info", "\u{e795}"),            // ""
            ("ini", "\u{e615}"),             // ""
            ("ipynb", "\u{e606}"),           // ""
            ("iso", "\u{f1c0}"),             // ""
            ("j2", "\u{e000}"),              // ""
            ("jar", "\u{e738}"),             // ""
            ("java", "\u{e738}"),            // ""
            ("jinja", "\u{e000}"),           // ""
            ("jl", "\u{e624}"),              // ""
            ("jpeg", "\u{f1c5}"),            // ""
            ("jpg", "\u{f1c5}"),             // ""
            ("jsonc", "\u{e60b}"),           // ""
            ("json", "\u{e60b}"),            // ""
            ("js", "\u{e74e}"),              // ""
            ("jsx", "\u{e7ba}"),             // ""
            ("key", "\u{e60a}"),             // ""
            ("kt", "\u{e634}"),              // ""
            ("kts", "\u{e634}"),             // ""
            ("ksh", "\u{f489}"),             // ""
            ("kts", "\u{e634}"),             // ""
            ("kt", "\u{e634}"),              // ""
            ("ldb", "\u{f1c0}"),             // ""
            ("ld", "\u{e624}"),              // ""
            ("less", "\u{e758}"),            // ""
            ("lhs", "\u{e777}"),             // ""
            ("license", "\u{e60a}"),         // ""
            ("lisp", "\u{f671}"),            // ""
            ("list", "\u{f03a}"),            // ""
            ("localized", "\u{f179}"),       // ""
            ("lock", "\u{f023}"),            // ""
            ("log", "\u{f18d}"),             // ""
            ("lss", "\u{e749}"),             // ""
            ("lua", "\u{e620}"),             // ""
            ("lz", "\u{f410}"),              // ""
            ("m3u8", "\u{f910}"),            // "蘿"
            ("m3u", "\u{f910}"),             // "蘿"
            ("m4a", "\u{f001}"),             // ""
            ("m4v", "\u{f008}"),             // ""
            ("magnet", "\u{f076}"),          // ""
            ("man", "\u{f02d}"),             // ""
            ("markdown", "\u{e609}"),        // ""
            ("md", "\u{e609}"),              // ""
            ("mjs", "\u{e74e}"),             // ""
            ("mkd", "\u{e609}"),             // ""
            ("mk", "\u{f085}"),              // ""
            ("mkv", "\u{f008}"),             // ""
            ("mobi", "\u{e28b}"),            // ""
            ("mov", "\u{f008}"),             // ""
            ("mp3", "\u{f001}"),             // ""
            ("mp4", "\u{f008}"),             // ""
            ("msi", "\u{f17a}"),             // ""
            ("mustache", "\u{e60f}"),        // ""
            ("nix", "\u{f313}"),             // ""
            ("npmignore", "\u{e71e}"),       // ""
            ("ogg", "\u{f001}"),             // ""
            ("ogv", "\u{f008}"),             // ""
            ("old", "\u{f56e}"),             // ""
            ("opus", "\u{f001}"),            // ""
            ("orig", "\u{f56e}"),            // ""
            ("otf", "\u{f031}"),             // ""
            ("o", "\u{e624}"),               // ""
            ("pdf", "\u{f1c1}"),             // ""
            ("pem", "\u{f805}"),             // ""
            ("phar", "\u{e608}"),            // ""
            ("php", "\u{e608}"),             // ""
            ("pkg", "\u{f187}"),             // ""
            ("plist", "\u{f302}"),           // ""
            ("pls", "\u{f910}"),             // "蘿"
            ("pl", "\u{e769}"),              // ""
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
            ("pyc", "\u{e606}"),             // ""
            ("py", "\u{e606}"),              // ""
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
            ("rl", "\u{f11c}"),              // ""
            ("rmd", "\u{e609}"),             // ""
            ("rpm", "\u{f187}"),             // ""
            ("rproj", "\u{fac5}"),           // "鉶"
            ("rspec_parallel", "\u{e21e}"),  // ""
            ("rspec_status", "\u{e21e}"),    // ""
            ("rspec", "\u{e21e}"),           // ""
            ("rss", "\u{f09e}"),             // ""
            ("rs", "\u{e7a8}"),              // ""
            ("rtf", "\u{f15c}"),             // ""
            ("rubydoc", "\u{e73b}"),         // ""
            ("r", "\u{fcd2}"),               // "ﳒ"
            ("ru", "\u{e21e}"),              // ""
            ("sass", "\u{e603}"),            // ""
            ("scala", "\u{e737}"),           // ""
            ("scpt", "\u{f302}"),            // ""
            ("scss", "\u{e603}"),            // ""
            ("shell", "\u{f489}"),           // ""
            ("sh", "\u{f489}"),              // ""
            ("sig", "\u{e60a}"),             // ""
            ("slim", "\u{e73b}"),            // ""
            ("sln", "\u{e70c}"),             // ""
            ("so", "\u{e624}"),              // ""
            ("sqlite3", "\u{e7c4}"),         // ""
            ("sql", "\u{f1c0}"),             // ""
            ("srt", "\u{f02d}"),             // ""
            ("styl", "\u{e600}"),            // ""
            ("stylus", "\u{e600}"),          // ""
            ("sublime-package", "\u{e7aa}"), // ""
            ("sublime-session", "\u{e7aa}"), // ""
            ("sub", "\u{f02d}"),             // ""
            ("s", "\u{f471}"),               // ""
            ("svg", "\u{f1c5}"),             // ""
            ("swift", "\u{e755}"),           // ""
            ("swp", "\u{e62b}"),             // ""
            ("sym", "\u{e624}"),             // ""
            ("tar", "\u{f410}"),             // ""
            ("tex", "\u{e600}"),             // ""
            ("tgz", "\u{f410}"),             // ""
            ("tiff", "\u{f1c5}"),            // ""
            ("toml", "\u{e60b}"),            // ""
            ("torrent", "\u{f98c}"),         // "歷"
            ("trash", "\u{f1f8}"),           // ""
            ("ts", "\u{e628}"),              // ""
            ("tsx", "\u{e7ba}"),             // ""
            ("ttc", "\u{f031}"),             // ""
            ("ttf", "\u{f031}"),             // ""
            ("t", "\u{e769}"),               // ""
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
            ("woff2", "\u{f031}"),           // ""
            ("woff", "\u{f031}"),            // ""
            ("wpl", "\u{f910}"),             // "蘿"
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
            ("zshrc", "\u{f489}"),           // ""
            ("zsh-theme", "\u{f489}"),       // ""
            ("zsh", "\u{f489}"),             // ""
            ("zst", "\u{f410}"),             // ""
        ]
        .iter()
        .map(|&s| (s.0.to_owned(), s.1.to_owned()))
        .collect::<HashMap<_, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::IconTheme;
    use crate::theme::Theme;

    fn partial_default_yaml() -> &'static str {
        r#"---
name:
  .trash: 
  .cargo: 
  .emacs.d: 
  a.out: 
extension:
  go: 
  hs: 
  rs: 
filetype:
  dir: 
  file: 
  pipe: 
  socket: 
  executable: 
  symlink-dir: 
  symlink-file: 
  device-char: 
  device-block: ﰩ
  special: 
"#
    }

    fn check_partial_yaml(def: &IconTheme, yaml: &IconTheme) {
        assert_eq!(def.filetype.dir, yaml.filetype.dir,);
    }

    #[test]
    fn test_default_theme() {
        let def = IconTheme::default();
        let yaml = Theme::with_yaml(partial_default_yaml()).unwrap();
        check_partial_yaml(&def, &yaml);
    }

    #[test]
    fn test_tmp_partial_default_theme_file() {
        use std::fs::File;
        use std::io::Write;
        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("icon.yaml");
        let mut file = File::create(&theme).unwrap();
        writeln!(file, "{}", partial_default_yaml()).unwrap();
        let def = IconTheme::default();
        let decoded = Theme::from_path(theme.to_str().unwrap()).unwrap();
        check_partial_yaml(&def, &decoded);
    }

    #[test]
    fn test_empty_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("  ").unwrap();
        let default = IconTheme::default();
        check_partial_yaml(&empty, &default);
    }

    #[test]
    fn test_partial_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("filetype:\n  dir: ").unwrap(); //  is the default value
        let default = IconTheme::default();
        check_partial_yaml(&empty, &default);
    }

    #[test]
    fn test_serde_dir_from_yaml() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("filetype:\n  dir: ").unwrap();
        assert_eq!(empty.filetype.dir, "");
    }
}
