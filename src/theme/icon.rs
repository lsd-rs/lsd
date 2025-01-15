use serde::Deserialize;
use std::collections::HashMap;

enum ByFilename {
    Name,
    Extension,
}

fn deserialize_by_filename<'de, D>(
    deserializer: D,
    by: ByFilename,
) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let default = match by {
        ByFilename::Name => IconTheme::get_default_icons_by_name(),
        ByFilename::Extension => IconTheme::get_default_icons_by_extension(),
    };
    HashMap::<_, _>::deserialize(deserializer)
        .map(|input| default.into_iter().chain(input).collect())
}

fn deserialize_by_name<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserialize_by_filename(deserializer, ByFilename::Name)
}

fn deserialize_by_extension<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserialize_by_filename(deserializer, ByFilename::Extension)
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct IconTheme {
    #[serde(deserialize_with = "deserialize_by_name")]
    pub name: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_by_extension")]
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
            dir: "\u{f115}".into(),           // 
            file: "\u{f016}".into(),          // 
            pipe: "\u{f0232}".into(),         // 󰈲
            socket: "\u{f01a8}".into(),       // 󰆨
            executable: "\u{f489}".into(),    // 
            symlink_dir: "\u{f482}".into(),   // 
            symlink_file: "\u{f481}".into(),  // 
            device_char: "\u{e601}".into(),   // 
            device_block: "\u{f072b}".into(), // 󰜫
            special: "\u{f2dc}".into(),       // 
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
            executable: "\u{1f3d7} ".into(),
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
            ("api", "\u{f048d}"),               // "󰒍"
            (".asoundrc", "\u{e615}"),          // ""
            (".atom", "\u{e764}"),              // ""
            (".ash", "\u{f489}"),               // ""
            (".ash_history", "\u{f489}"),       // ""
            ("authorized_keys", "\u{e60a}"),    // ""
            ("assets", "\u{f0c7}"),             // ""
            (".android", "\u{f17b}"),           // ""
            (".audacity-data", "\u{e5fc}"),     // ""
            ("backups", "\u{f006f}"),           // "󰁯"
            (".bash_history", "\u{f1183}"),     // "󱆃"
            (".bash_logout", "\u{f1183}"),      // "󱆃"
            (".bash_profile", "\u{f1183}"),     // "󱆃"
            (".bashrc", "\u{f1183}"),           // "󱆃"
            ("bin", "\u{e5fc}"),                // ""
            (".bpython_history", "\u{e606}"),   // ""
            ("build", "\u{f487}"),              // ""
            ("bspwmrc", "\u{e615}"),            // ""
            ("build.ninja", "\u{f0ad}"),        // ""
            (".cache", "\u{f00e8}"),            // "󰃨"
            ("cache", "\u{f00e8}"),             // "󰃨"
            ("cargo.lock", "\u{e68b}"),         // ""
            ("cargo.toml", "\u{e68b}"),         // ""
            (".cargo", "\u{e68b}"),             // ""
            (".ccls-cache", "\u{f00e8}"),       // "󰃨"
            ("changelog", "\u{e609}"),          // ""
            (".clang-format", "\u{e615}"),      // ""
            ("composer.json", "\u{e608}"),      // ""
            ("composer.lock", "\u{e608}"),      // ""
            ("conf.d", "\u{e5fc}"),             // ""
            ("config.ac", "\u{e615}"),          // ""
            ("config.el", "\u{e632}"),          // ""
            ("config.mk", "\u{e615}"),          // ""
            (".config", "\u{e5fc}"),            // ""
            ("config", "\u{e5fc}"),             // ""
            ("configure", "\u{f0ad}"),          // ""
            ("content", "\u{f0c7}"),            // ""
            ("contributing", "\u{e60a}"),       // ""
            ("copyright", "\u{e60a}"),          // ""
            ("cron.daily", "\u{e5fc}"),         // ""
            ("cron.d", "\u{e5fc}"),             // ""
            ("cron.deny", "\u{e615}"),          // ""
            ("cron.hourly", "\u{e5fc}"),        // ""
            ("cron.monthly", "\u{e5fc}"),       // ""
            ("crontab", "\u{e615}"),            // ""
            ("cron.weekly", "\u{e5fc}"),        // ""
            ("crypttab", "\u{e615}"),           // ""
            (".cshrc", "\u{f1183}"),            // "󱆃"
            ("csh.cshrc", "\u{f1183}"),         // "󱆃"
            ("csh.login", "\u{f1183}"),         // "󱆃"
            ("csh.logout", "\u{f1183}"),        // "󱆃"
            ("css", "\u{e749}"),                // ""
            ("custom.el", "\u{e632}"),          // ""
            (".dbus", "\u{f013}"),              // ""
            ("desktop", "\u{f108}"),            // ""
            ("docker-compose.yml", "\u{f308}"), // ""
            ("dockerfile", "\u{f308}"),         // ""
            ("doc", "\u{f02d}"),                // ""
            ("dist", "\u{f487}"),               // ""
            ("documents", "\u{f02d}"),          // ""
            (".doom.d", "\u{e632}"),            // ""
            ("downloads", "\u{f024d}"),         // "󰉍"
            (".ds_store", "\u{f179}"),          // ""
            (".editorconfig", "\u{e615}"),      // ""
            (".electron-gyp", "\u{e5fa}"),      // ""
            (".emacs.d", "\u{e632}"),           // ""
            (".env", "\u{f462}"),               // ""
            ("environment", "\u{f462}"),        // ""
            (".eslintrc.json", "\u{f462}"),     // ""
            (".eslintrc.js", "\u{f462}"),       // ""
            (".eslintrc.yml", "\u{f462}"),      // ""
            ("etc", "\u{e5fc}"),                // ""
            ("favicon.ico", "\u{f005}"),        // ""
            ("favicons", "\u{f005}"),           // ""
            (".fennelrc", "\u{e615}"),          // ""
            ("fstab", "\u{f1c0}"),              // ""
            (".fastboot", "\u{f17b}"),          // ""
            (".gitattributes", "\u{f1d3}"),     // ""
            (".gitconfig", "\u{f1d3}"),         // ""
            (".git-credentials", "\u{e60a}"),   // ""
            (".github", "\u{e5fd}"),            // ""
            ("gitignore_global", "\u{f1d3}"),   // ""
            (".gitignore", "\u{f1d3}"),         // ""
            (".gitlab-ci.yml", "\u{f296}"),     // ""
            (".gitmodules", "\u{f1d3}"),        // ""
            (".git", "\u{e5fb}"),               // ""
            (".gnupg", "\u{f08ac}"),            // "󰢬"
            ("go.mod", "\u{e627}"),             // ""
            ("go.sum", "\u{e627}"),             // ""
            ("go.work", "\u{e627}"),            // ""
            ("gradle", "\u{e660}"),             // ""
            ("gradle.properties", "\u{e660}"),  // ""
            ("gradlew", "\u{e660}"),            // ""
            ("gradlew.bat", "\u{e660}"),        // ""
            ("group", "\u{e615}"),              // ""
            ("gruntfile.coffee", "\u{e611}"),   // ""
            ("gruntfile.js", "\u{e611}"),       // ""
            ("gruntfile.ls", "\u{e611}"),       // ""
            ("gshadow", "\u{e615}"),            // ""
            ("gulpfile.coffee", "\u{e610}"),    // ""
            ("gulpfile.js", "\u{e610}"),        // ""
            ("gulpfile.ls", "\u{e610}"),        // ""
            ("heroku.yml", "\u{e77b}"),         // ""
            ("hidden", "\u{f023}"),             // ""
            ("home", "\u{f015}"),               // ""
            ("hostname", "\u{e615}"),           // ""
            ("hosts", "\u{f0002}"),             // "󰀂"
            (".htaccess", "\u{e615}"),          // ""
            ("htoprc", "\u{e615}"),             // ""
            (".htpasswd", "\u{e615}"),          // ""
            (".icons", "\u{f005}"),             // ""
            ("icons", "\u{f005}"),              // ""
            ("id_dsa", "\u{f0dd6}"),            // "󰷖"
            ("id_ecdsa", "\u{f0dd6}"),          // "󰷖"
            ("id_rsa", "\u{f0dd6}"),            // "󰷖"
            (".idlerc", "\u{e235}"),            // ""
            ("img", "\u{f1c5}"),                // ""
            ("include", "\u{e5fc}"),            // ""
            ("init.el", "\u{e632}"),            // ""
            (".inputrc", "\u{e615}"),           // ""
            ("inputrc", "\u{e615}"),            // ""
            (".java", "\u{e256}"),              // ""
            ("jenkinsfile", "\u{e66e}"),        // ""
            ("js", "\u{e74e}"),                 // ""
            ("jule.mod", "\u{e80c}"),           // ""
            (".jupyter", "\u{e606}"),           // ""
            ("kbuild", "\u{e615}"),             // ""
            ("kconfig", "\u{e615}"),            // ""
            ("kdeglobals", "\u{e615}"),         // ""
            ("kdenliverc", "\u{e615}"),         // ""
            ("known_hosts", "\u{e60a}"),        // ""
            (".kshrc", "\u{f489}"),             // ""
            ("libexec", "\u{f121}"),            // ""
            ("lib32", "\u{f121}"),              // ""
            ("lib64", "\u{f121}"),              // ""
            ("lib", "\u{f121}"),                // ""
            ("license.md", "\u{e60a}"),         // ""
            ("licenses", "\u{e60a}"),           // ""
            ("license.txt", "\u{e60a}"),        // ""
            ("license", "\u{e60a}"),            // ""
            ("localized", "\u{f179}"),          // ""
            ("lsb-release", "\u{e615}"),        // ""
            (".lynxrc", "\u{e615}"),            // ""
            (".mailcap", "\u{f01f0}"),          // "󰇰"
            ("mail", "\u{f01f0}"),              // "󰇰"
            ("magic", "\u{f0d0}"),              // ""
            ("maintainers", "\u{e60a}"),        // ""
            ("makefile.ac", "\u{e615}"),        // ""
            ("makefile", "\u{e615}"),           // ""
            ("manifest", "\u{f292}"),           // ""
            ("md5sum", "\u{f0565}"),            // "󰕥"
            ("meson.build", "\u{f0ad}"),        // ""
            ("metadata", "\u{e5fc}"),           // ""
            ("metadata.xml", "\u{f462}"),       // ""
            ("media", "\u{f40f}"),              // ""
            (".mime.types", "\u{f0645}"),       // "󰙅"
            ("mime.types", "\u{f0645}"),        // "󰙅"
            ("module.symvers", "\u{f471}"),     // ""
            (".mozilla", "\u{e786}"),           // ""
            ("music", "\u{f1359}"),             // "󱍙"
            ("muttrc", "\u{e615}"),             // ""
            (".muttrc", "\u{e615}"),            // ""
            (".mutt", "\u{e615}"),              // ""
            (".mypy_cache", "\u{f00e8}"),       // "󰃨"
            ("neomuttrc", "\u{e615}"),          // ""
            (".neomuttrc", "\u{e615}"),         // ""
            ("netlify.toml", "\u{f233}"),       // ""
            (".nix-channels", "\u{f313}"),      // ""
            (".nix-defexpr", "\u{f313}"),       // ""
            (".node-gyp", "\u{e5fa}"),          // ""
            ("node_modules", "\u{e5fa}"),       // ""
            (".node_repl_history", "\u{e718}"), // ""
            ("npmignore", "\u{e71e}"),          // ""
            (".npm", "\u{e5fa}"),               // ""
            ("nvim", "\u{f36f}"),               // ""
            ("obj", "\u{e624}"),                // ""
            ("os-release", "\u{e615}"),         // ""
            ("package.json", "\u{e718}"),       // ""
            ("package-lock.json", "\u{e718}"),  // ""
            ("packages.el", "\u{e632}"),        // ""
            ("pam.d", "\u{f08ac}"),             // "󰢬"
            ("passwd", "\u{f023}"),             // ""
            ("pictures", "\u{f024f}"),          // "󰉏"
            ("pkgbuild", "\u{f303}"),           // ""
            (".pki", "\u{f023}"),               // ""
            ("portage", "\u{f30d}"),            // ""
            ("profile", "\u{e615}"),            // ""
            (".profile", "\u{e615}"),           // ""
            ("public", "\u{f415}"),             // ""
            ("__pycache__", "\u{e606}"),        // ""
            ("pyproject.toml", "\u{e606}"),     // ""
            (".python_history", "\u{e606}"),    // ""
            (".pypirc", "\u{e606}"),            // ""
            ("rc.lua", "\u{e615}"),             // ""
            ("readme", "\u{e609}"),             // ""
            (".release.toml", "\u{e68b}"),      // ""
            ("requirements.txt", "\u{f0320}"),  // "󰌠"
            ("robots.txt", "\u{f06a9}"),        // "󰚩"
            ("root", "\u{f0250}"),              // "󰉐"
            ("rubydoc", "\u{e73b}"),            // ""
            ("runtime.txt", "\u{f0320}"),       // "󰌠"
            (".rustup", "\u{e68b}"),            // ""
            ("rustfmt.toml", "\u{e68b}"),       // ""
            (".rvm", "\u{e21e}"),               // ""
            ("sass", "\u{e603}"),               // ""
            ("sbin", "\u{e5fc}"),               // ""
            ("scripts", "\u{f489}"),            // ""
            ("scss", "\u{e603}"),               // ""
            ("sha256sum", "\u{f0565}"),         // "󰕥"
            ("shadow", "\u{e615}"),             // ""
            ("share", "\u{f064}"),              // ""
            (".shellcheckrc", "\u{e615}"),      // ""
            ("shells", "\u{e615}"),             // ""
            (".spacemacs", "\u{e632}"),         // ""
            (".sqlite_history", "\u{e7c4}"),    // ""
            ("src", "\u{f19fc}"),               // "󱧼"
            (".ssh", "\u{f08ac}"),              // "󰢬"
            ("static", "\u{f0c7}"),             // ""
            ("std", "\u{f0171}"),               // "󰅱"
            ("styles", "\u{e749}"),             // ""
            ("subgid", "\u{e615}"),             // ""
            ("subuid", "\u{e615}"),             // ""
            ("sudoers", "\u{f023}"),            // ""
            ("sxhkdrc", "\u{e615}"),            // ""
            ("template", "\u{f32e}"),           // ""
            ("tests", "\u{f0668}"),             // "󰙨"
            ("tigrc", "\u{e615}"),              // ""
            ("timezone", "\u{f43a}"),           // ""
            ("tox.ini", "\u{e615}"),            // ""
            (".trash", "\u{f1f8}"),             // ""
            ("ts", "\u{e628}"),                 // ""
            (".tox", "\u{e606}"),               // ""
            ("unlicense", "\u{e60a}"),          // ""
            ("url", "\u{f0ac}"),                // ""
            ("user-dirs.dirs", "\u{e5fc}"),     // ""
            ("vagrantfile", "\u{e615}"),        // ""
            ("vendor", "\u{f0ae6}"),            // "󰫦"
            ("venv", "\u{f0320}"),              // "󰌠"
            ("videos", "\u{f03d}"),             // ""
            (".viminfo", "\u{e62b}"),           // ""
            (".vimrc", "\u{e62b}"),             // ""
            ("vimrc", "\u{e62b}"),              // ""
            (".vim", "\u{e62b}"),               // ""
            ("vim", "\u{e62b}"),                // ""
            (".vscode", "\u{e70c}"),            // ""
            ("webpack.config.js", "\u{f072b}"), // "󰜫"
            (".wgetrc", "\u{e615}"),            // ""
            ("wgetrc", "\u{e615}"),             // ""
            (".xauthority", "\u{e615}"),        // ""
            (".Xauthority", "\u{e615}"),        // ""
            ("xbps.d", "\u{f32e}"),             // ""
            ("xbps-src", "\u{f32e}"),           // ""
            (".xinitrc", "\u{e615}"),           // ""
            (".xmodmap", "\u{e615}"),           // ""
            (".Xmodmap", "\u{e615}"),           // ""
            ("xmonad.hs", "\u{e615}"),          // ""
            ("xorg.conf.d", "\u{e5fc}"),        // ""
            (".xprofile", "\u{e615}"),          // ""
            (".Xprofile", "\u{e615}"),          // ""
            (".xresources", "\u{e615}"),        // ""
            (".yarnrc", "\u{e6a7}"),            // ""
            ("yarn.lock", "\u{e6a7}"),          // ""
            ("zathurarc", "\u{e615}"),          // ""
            (".zcompdump", "\u{e615}"),         // ""
            (".zlogin", "\u{f1183}"),           // "󱆃"
            (".zlogout", "\u{f1183}"),          // "󱆃"
            (".zprofile", "\u{f1183}"),         // "󱆃"
            (".zsh_history", "\u{f1183}"),      // "󱆃"
            (".zshrc", "\u{f1183}"),            // "󱆃"
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
            ("890", "\u{f015e}"),            // "󰅞"
            ("a", "\u{e624}"),               // ""
            ("ai", "\u{e7b4}"),              // ""
            ("ape", "\u{f001}"),             // ""
            ("apk", "\u{e70e}"),             // ""
            ("apng", "\u{f1c5}"),            // ""
            ("ar", "\u{f410}"),              // ""
            ("asc", "\u{f099d}"),            // "󰦝"
            ("asm", "\u{f471}"),             // ""
            ("asp", "\u{f121}"),             // ""
            ("avi", "\u{f008}"),             // ""
            ("avif", "\u{f1c5}"),            // ""
            ("avro", "\u{e60b}"),            // ""
            ("awk", "\u{f489}"),             // ""
            ("bak", "\u{f006f}"),            // "󰁯"
            ("bash_history", "\u{f489}"),    // ""
            ("bash_profile", "\u{f489}"),    // ""
            ("bashrc", "\u{f489}"),          // ""
            ("bash", "\u{f489}"),            // ""
            ("bat", "\u{f17a}"),             // ""
            ("bin", "\u{eae8}"),             // ""
            ("bio", "\u{f0411}"),            // "󰐑"
            ("blend", "\u{f00ab}"),          // "󰂫"
            ("blend1", "\u{f00ab}"),         // "󰂫"
            ("bmp", "\u{f1c5}"),             // ""
            ("bz2", "\u{f410}"),             // ""
            ("cc", "\u{e61d}"),              // ""
            ("cfg", "\u{e615}"),             // ""
            ("cip", "\u{f015e}"),            // "󰅞"
            ("cjs", "\u{e74e}"),             // ""
            ("class", "\u{e738}"),           // ""
            ("cljs", "\u{e76a}"),            // ""
            ("clj", "\u{e768}"),             // ""
            ("cls", "\u{e600}"),             // ""
            ("cl", "\u{f0172}"),             // "󰅲"
            ("cmd", "\u{f17a}"),             // ""
            ("coffee", "\u{f0f4}"),          // ""
            ("conf", "\u{e615}"),            // ""
            ("cpp", "\u{e61d}"),             // ""
            ("cp", "\u{e61d}"),              // ""
            ("cshtml", "\u{f1fa}"),          // ""
            ("csh", "\u{f489}"),             // ""
            ("csproj", "\u{f031b}"),         // "󰌛"
            ("css", "\u{e749}"),             // ""
            ("cs", "\u{f031b}"),             // "󰌛"
            ("csv", "\u{f1c3}"),             // ""
            ("csx", "\u{f031b}"),            // "󰌛"
            ("cts", "\u{e628}"),             // ""
            ("c++", "\u{e61d}"),             // ""
            ("c", "\u{e61e}"),               // ""
            ("cue", "\u{f001}"),             // ""
            ("cxx", "\u{e61d}"),             // ""
            ("cypher", "\u{f1c0}"),          // ""
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
            ("download", "\u{f43a}"),        // ""
            ("ds_store", "\u{f179}"),        // ""
            ("dump", "\u{f1c0}"),            // ""
            ("ebook", "\u{e28b}"),           // ""
            ("ebuild", "\u{f30d}"),          // ""
            ("eclass", "\u{f30d}"),          // ""
            ("editorconfig", "\u{e615}"),    // ""
            ("egg-info", "\u{e606}"),        // ""
            ("ejs", "\u{e618}"),             // ""
            ("elc", "\u{f0172}"),            // "󰅲"
            ("elf", "\u{f489}"),             // ""
            ("elm", "\u{e62c}"),             // ""
            ("el", "\u{f0172}"),             // "󰅲"
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
            ("fnl", "\u{e6af}"),             // ""
            ("font", "\u{f031}"),            // ""
            ("fpl", "\u{f0411}"),            // "󰐑"
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
            ("gradle", "\u{e660}"),          // ""
            ("gsheet", "\u{f1c3}"),          // ""
            ("gslides", "\u{f1c4}"),         // ""
            ("guardfile", "\u{e21e}"),       // ""
            ("gv", "\u{f1049}"),             // "󱁉"
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
            ("in", "\u{f15c}"),              // ""
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
            ("jule", "\u{e80c}"),            // ""
            ("key", "\u{f0306}"),            // "󰌆"
            ("ksh", "\u{f489}"),             // ""
            ("kt", "\u{e634}"),              // ""
            ("kts", "\u{e634}"),             // ""
            ("kusto", "\u{f1c0}"),           // ""
            ("ldb", "\u{f1c0}"),             // ""
            ("ld", "\u{e624}"),              // ""
            ("less", "\u{e758}"),            // ""
            ("lhs", "\u{e777}"),             // ""
            ("license", "\u{e60a}"),         // ""
            ("lisp", "\u{f0172}"),           // "󰅲"
            ("list", "\u{f03a}"),            // ""
            ("localized", "\u{f179}"),       // ""
            ("lock", "\u{f023}"),            // ""
            ("log", "\u{f18d}"),             // ""
            ("lss", "\u{e749}"),             // ""
            ("lua", "\u{e620}"),             // ""
            ("lz", "\u{f410}"),              // ""
            ("mgc", "\u{f0d0}"),             // ""
            ("m3u8", "\u{f0411}"),           // "󰐑"
            ("m3u", "\u{f0411}"),            // "󰐑"
            ("m4a", "\u{f001}"),             // ""
            ("m4v", "\u{f008}"),             // ""
            ("magnet", "\u{f076}"),          // ""
            ("malloy", "\u{f1c0}"),          // ""
            ("man", "\u{f02d}"),             // ""
            ("markdown", "\u{e609}"),        // ""
            ("md", "\u{e609}"),              // ""
            ("mjs", "\u{e74e}"),             // ""
            ("mkd", "\u{e609}"),             // ""
            ("mk", "\u{f085}"),              // ""
            ("mkv", "\u{f008}"),             // ""
            ("ml", "\u{e67a}"),              // ""
            ("mli", "\u{e67a}"),             // ""
            ("mll", "\u{e67a}"),             // ""
            ("mly", "\u{e67a}"),             // ""
            ("mobi", "\u{e28b}"),            // ""
            ("mov", "\u{f008}"),             // ""
            ("mp3", "\u{f001}"),             // ""
            ("mp4", "\u{f008}"),             // ""
            ("msi", "\u{f17a}"),             // ""
            ("mts", "\u{e628}"),             // ""
            ("mustache", "\u{e60f}"),        // ""
            ("nim", "\u{e677}"),             // ""
            ("nimble", "\u{e677}"),          // ""
            ("nix", "\u{f313}"),             // ""
            ("npmignore", "\u{e71e}"),       // ""
            ("odp", "\u{f1c4}"),             // ""
            ("ods", "\u{f1c3}"),             // ""
            ("odt", "\u{f1c2}"),             // ""
            ("ogg", "\u{f001}"),             // ""
            ("ogv", "\u{f008}"),             // ""
            ("old", "\u{f006f}"),            // "󰁯"
            ("opus", "\u{f001}"),            // ""
            ("orig", "\u{f006f}"),           // "󰁯"
            ("org", "\u{e633}"),             // ""
            ("otf", "\u{f031}"),             // ""
            ("o", "\u{eae8}"),               // ""
            ("part", "\u{f43a}"),            // ""
            ("patch", "\u{e728}"),           // ""
            ("pdb", "\u{f0aaa}"),            // "󰪪"
            ("pdf", "\u{f1c1}"),             // ""
            ("pem", "\u{f0306}"),            // "󰌆"
            ("phar", "\u{e608}"),            // ""
            ("php", "\u{e608}"),             // ""
            ("pkg", "\u{f187}"),             // ""
            ("pl", "\u{e67e}"),              // ""
            ("plist", "\u{f302}"),           // ""
            ("pls", "\u{f0411}"),            // "󰐑"
            ("plx", "\u{e67e}"),             // ""
            ("pm", "\u{e67e}"),              // ""
            ("png", "\u{f1c5}"),             // ""
            ("pod", "\u{e67e}"),             // ""
            ("pp", "\u{e631}"),              // ""
            ("ppt", "\u{f1c4}"),             // ""
            ("pptx", "\u{f1c4}"),            // ""
            ("procfile", "\u{e21e}"),        // ""
            ("properties", "\u{e60b}"),      // ""
            ("prql", "\u{f1c0}"),            // ""
            ("ps1", "\u{f489}"),             // ""
            ("psd", "\u{e7b8}"),             // ""
            ("pub", "\u{f0306}"),            // "󰌆"
            ("sbv", "\u{f015e}"),            // "󰅞"
            ("scc", "\u{f015e}"),            // "󰅞"
            ("slt", "\u{f0221}"),            // "󰈡"
            ("smi", "\u{f015e}"),            // "󰅞"
            ("pxm", "\u{f1c5}"),             // ""
            ("pyc", "\u{e606}"),             // ""
            ("py", "\u{e606}"),              // ""
            ("rakefile", "\u{e21e}"),        // ""
            ("rar", "\u{f410}"),             // ""
            ("razor", "\u{f1fa}"),           // ""
            ("rb", "\u{e21e}"),              // ""
            ("rdata", "\u{f07d4}"),          // "󰟔"
            ("rdb", "\u{e76d}"),             // ""
            ("rdoc", "\u{e609}"),            // ""
            ("rds", "\u{f07d4}"),            // "󰟔"
            ("readme", "\u{e609}"),          // ""
            ("rlib", "\u{e68b}"),            // ""
            ("rl", "\u{f11c}"),              // ""
            ("rmd", "\u{e609}"),             // ""
            ("rmeta", "\u{e68b}"),           // ""
            ("rpm", "\u{f187}"),             // ""
            ("rproj", "\u{f05c6}"),          // "󰗆"
            ("rq", "\u{f1c0}"),              // ""
            ("rspec_parallel", "\u{e21e}"),  // ""
            ("rspec_status", "\u{e21e}"),    // ""
            ("rspec", "\u{e21e}"),           // ""
            ("rss", "\u{f09e}"),             // ""
            ("rs", "\u{e68b}"),              // ""
            ("rtf", "\u{f15c}"),             // ""
            ("rubydoc", "\u{e73b}"),         // ""
            ("r", "\u{f07d4}"),              // "󰟔"
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
            ("srt", "\u{f0a16}"),            // "󰨖"
            ("styl", "\u{e600}"),            // ""
            ("stylus", "\u{e600}"),          // ""
            ("sublime-menu", "\u{e7aa}"),    // ""
            ("sublime-package", "\u{e7aa}"), // ""
            ("sublime-project", "\u{e7aa}"), // ""
            ("sublime-session", "\u{e7aa}"), // ""
            ("sub", "\u{f0a16}"),            // "󰨖"
            ("s", "\u{f471}"),               // ""
            ("svg", "\u{f1c5}"),             // ""
            ("svelte", "\u{e697}"),          // ""
            ("swift", "\u{e755}"),           // ""
            ("swp", "\u{e62b}"),             // ""
            ("sym", "\u{eae8}"),             // ""
            ("tar", "\u{f410}"),             // ""
            ("taz", "\u{f410}"),             // ""
            ("tbz", "\u{f410}"),             // ""
            ("tbz2", "\u{f410}"),            // ""
            ("tex", "\u{e600}"),             // ""
            ("tgz", "\u{f410}"),             // ""
            ("tiff", "\u{f1c5}"),            // ""
            ("timestamp", "\u{f43a}"),       // ""
            ("toml", "\u{e60b}"),            // ""
            ("torrent", "\u{f048d}"),        // "󰒍"
            ("trash", "\u{f1f8}"),           // ""
            ("ts", "\u{e628}"),              // ""
            ("tsx", "\u{e7ba}"),             // ""
            ("ttc", "\u{f031}"),             // ""
            ("ttf", "\u{f031}"),             // ""
            ("t", "\u{e769}"),               // ""
            ("twig", "\u{e61c}"),            // ""
            ("txt", "\u{f15c}"),             // ""
            ("unity", "\u{e721}"),           // ""
            ("unity32", "\u{e721}"),         // ""
            ("video", "\u{f008}"),           // ""
            ("vim", "\u{e62b}"),             // ""
            ("vlc", "\u{f0411}"),            // "󰐑"
            ("vtt", "\u{f015e}"),            // "󰅞"
            ("vue", "\u{f0844}"),            // "󰡄"
            ("wav", "\u{f001}"),             // ""
            ("webm", "\u{f008}"),            // ""
            ("webp", "\u{f1c5}"),            // ""
            ("whl", "\u{f487}"),             // ""
            ("windows", "\u{f17a}"),         // ""
            ("wma", "\u{f001}"),             // ""
            ("wmv", "\u{f008}"),             // ""
            ("woff2", "\u{f031}"),           // ""
            ("woff", "\u{f031}"),            // ""
            ("wpl", "\u{f0411}"),            // "󰐑"
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
            ("zig", "\u{e6a9}"),             // ""
            ("zon", "\u{e60b}"),             // ""
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
  .cargo: 
  .emacs.d: 
  a.out: 
extension:
  go: 
  hs: 
  rs: 
filetype:
  dir: 
  file: 
  pipe: 󰈲
  socket: 󰆨
  executable: 
  symlink-dir: 
  symlink-file: 
  device-char: 
  device-block: 󰜫
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

    #[test]
    fn test_custom_icon_by_name() {
        // When a user sets to use 📦-icon for a cargo.toml file,
        let theme: IconTheme = Theme::with_yaml("name:\n  cargo.toml: 📦").unwrap();
        // 📦-icon should be used for a cargo.toml file.
        assert_eq!(theme.name.get("cargo.toml").unwrap(), "📦");
    }

    #[test]
    fn test_default_icon_by_name_with_custom_entry() {
        // When a user sets to use 📦-icon for a cargo.toml file,
        let theme: IconTheme = Theme::with_yaml("name:\n  cargo.toml: 📦").unwrap();
        // the default icon  should be used for a cargo.lock file.
        assert_eq!(theme.name.get("cargo.lock").unwrap(), "\u{e68b}");
    }

    #[test]
    fn test_custom_icon_by_extension() {
        // When a user sets to use 🦀-icon for *.rs files,
        let theme: IconTheme = Theme::with_yaml("extension:\n  rs: 🦀").unwrap();
        // 🦀-icon should be used for *.rs files.
        assert_eq!(theme.extension.get("rs").unwrap(), "🦀");
    }

    #[test]
    fn test_default_icon_by_extension_with_custom_entry() {
        // When a user sets to use 🦀-icon for *.rs files,
        let theme: IconTheme = Theme::with_yaml("extension:\n  rs: 🦀").unwrap();
        // the default icon  should be used for *.go files.
        assert_eq!(theme.extension.get("go").unwrap(), "\u{e627}");
    }
}
