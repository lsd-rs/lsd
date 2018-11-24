use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Logo(String);

impl Into<String> for Logo {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Logo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Logo {
    pub fn folder() -> Self {
        Logo(String::from(""))
    }

    pub fn from_pathbuf(path: &PathBuf) -> Self {
        Logo::from_extension(path.extension().unwrap_or_default().to_str().unwrap())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "ai" => Logo(String::from("\u{e7b4}")),
            "android" => Logo(String::from("\u{e70e}")),
            "apple" => Logo(String::from("\u{f179}")),
            "audio" => Logo(String::from("\u{f001}")),
            "avro" => Logo(String::from("\u{e60b}")),
            "c" => Logo(String::from("\u{e61e}")),
            "clj" => Logo(String::from("\u{e768}")),
            "coffee" => Logo(String::from("\u{f0f4}")),
            "conf" => Logo(String::from("\u{e615}")),
            "cpp" => Logo(String::from("\u{e61d}")),
            "css" => Logo(String::from("\u{e749}")),
            "d" => Logo(String::from("\u{e7af}")),
            "dart" => Logo(String::from("\u{e798}")),
            "db" => Logo(String::from("\u{f1c0}")),
            "diff" => Logo(String::from("\u{f440}")),
            "doc" => Logo(String::from("\u{f1c2}")),
            "ebook" => Logo(String::from("\u{e28b}")),
            "env" => Logo(String::from("\u{f462}")),
            "epub" => Logo(String::from("\u{e28a}")),
            "erl" => Logo(String::from("\u{e7b1}")),
            "file" => Logo(String::from("\u{f15b}")),
            "font" => Logo(String::from("\u{f031}")),
            "gform" => Logo(String::from("\u{f298}")),
            "git" => Logo(String::from("\u{f1d3}")),
            "go" => Logo(String::from("\u{e626}")),
            "gruntfile.js" => Logo(String::from("\u{e74c}")),
            "hs" => Logo(String::from("\u{e777}")),
            "html" => Logo(String::from("\u{f13b}")),
            "image" => Logo(String::from("\u{f1c5}")),
            "iml" => Logo(String::from("\u{e7b5}")),
            "java" => Logo(String::from("\u{e204}")),
            "js" => Logo(String::from("\u{e74e}")),
            "json" => Logo(String::from("\u{e60b}")),
            "jsx" => Logo(String::from("\u{e7ba}")),
            "less" => Logo(String::from("\u{e758}")),
            "log" => Logo(String::from("\u{f18d}")),
            "lua" => Logo(String::from("\u{e620}")),
            "md" => Logo(String::from("\u{f48a}")),
            "mustache" => Logo(String::from("\u{e60f}")),
            "npmignore" => Logo(String::from("\u{e71e}")),
            "pdf" => Logo(String::from("\u{f1c1}")),
            "php" => Logo(String::from("\u{e73d}")),
            "pl" => Logo(String::from("\u{e769}")),
            "ppt" => Logo(String::from("\u{f1c4}")),
            "psd" => Logo(String::from("\u{e7b8}")),
            "py" => Logo(String::from("\u{e606}")),
            "r" => Logo(String::from("\u{f25d}")),
            "rb" => Logo(String::from("\u{e21e}")),
            "rdb" => Logo(String::from("\u{e76d}")),
            "rss" => Logo(String::from("\u{f09e}")),
            "rubydoc" => Logo(String::from("\u{e73b}")),
            "sass" => Logo(String::from("\u{e603}")),
            "scala" => Logo(String::from("\u{e737}")),
            "shell" => Logo(String::from("\u{f489}")),
            "sqlite3" => Logo(String::from("\u{e7c4}")),
            "styl" => Logo(String::from("\u{e600}")),
            "tex" => Logo(String::from("\u{e600}")),
            "ts" => Logo(String::from("\u{e628}")),
            "twig" => Logo(String::from("\u{e61c}")),
            "txt" => Logo(String::from("\u{f15c}")),
            "video" => Logo(String::from("\u{f03d}")),
            "vim" => Logo(String::from("\u{e62b}")),
            "windows" => Logo(String::from("\u{f17a}")),
            "xls" => Logo(String::from("\u{f1c3}")),
            "xml" => Logo(String::from("\u{e619}")),
            "yarn.lock" => Logo(String::from("\u{e718}")),
            "yml" => Logo(String::from("\u{f481}")),
            "zip" => Logo(String::from("\u{f410}")),
            _ => Logo(String::from("\u{f15b}")),
        }
    }
}
