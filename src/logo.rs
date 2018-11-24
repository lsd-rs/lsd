use meta::Meta;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref LogosByName: HashMap<&'static str, Logo> = {
        let mut m = HashMap::new();

        m.insert(".Trash", Logo(String::from("")));
        m.insert(".atom", Logo(String::from("")));
        m.insert(".git", Logo(String::from("")));
        m.insert(".github", Logo(String::from("")));
        m.insert(".rvm", Logo(String::from("")));
        m.insert(".vscode", Logo(String::from("")));
        m.insert("bin", Logo(String::from("")));
        m.insert("config", Logo(String::from("")));
        m.insert("ds_store", Logo(String::from("")));
        m.insert("folder", Logo(String::from("")));
        m.insert("gitconfig", Logo(String::from("")));
        m.insert("gitignore", Logo(String::from("")));
        m.insert("gitignore_global", Logo(String::from("")));
        m.insert("gradle", Logo(String::from("")));
        m.insert("hidden", Logo(String::from("")));
        m.insert("include", Logo(String::from("")));
        m.insert("lib", Logo(String::from("")));
        m.insert("localized", Logo(String::from("")));
        m.insert("node_modules", Logo(String::from("")));
        m.insert("npmignore", Logo(String::from("")));
        m.insert("rubydoc", Logo(String::from("")));
        m.insert("yarn.lock", Logo(String::from("")));

        m
    };
}

lazy_static! {
    pub static ref LogosByExtension: HashMap<&'static str, Logo> = {
        let mut m = HashMap::new();

        m.insert("ai", Logo(String::from("")));
        m.insert("apk", Logo(String::from("")));
        m.insert("apk", Logo(String::from("")));
        m.insert("avi", Logo(String::from("")));
        m.insert("avro", Logo(String::from("")));
        m.insert("bash", Logo(String::from("")));
        m.insert("bash_history", Logo(String::from("")));
        m.insert("bash_profile", Logo(String::from("")));
        m.insert("bashrc", Logo(String::from("")));
        m.insert("bat", Logo(String::from("")));
        m.insert("bmp", Logo(String::from("")));
        m.insert("bmp", Logo(String::from("")));
        m.insert("c", Logo(String::from("")));
        m.insert("clj", Logo(String::from("")));
        m.insert("cls", Logo(String::from("")));
        m.insert("coffee", Logo(String::from("")));
        m.insert("conf", Logo(String::from("")));
        m.insert("cpp", Logo(String::from("")));
        m.insert("css", Logo(String::from("")));
        m.insert("csv", Logo(String::from("")));
        m.insert("d", Logo(String::from("")));
        m.insert("dart", Logo(String::from("")));
        m.insert("db", Logo(String::from("")));
        m.insert("diff", Logo(String::from("")));
        m.insert("doc", Logo(String::from("")));
        m.insert("docx", Logo(String::from("")));
        m.insert("docx", Logo(String::from("")));
        m.insert("ds_store", Logo(String::from("")));
        m.insert("ds_store", Logo(String::from("")));
        m.insert("ebook", Logo(String::from("")));
        m.insert("editorconfig", Logo(String::from("")));
        m.insert("env", Logo(String::from("")));
        m.insert("eot", Logo(String::from("")));
        m.insert("eot", Logo(String::from("")));
        m.insert("epub", Logo(String::from("")));
        m.insert("erb", Logo(String::from("")));
        m.insert("erl", Logo(String::from("")));
        m.insert("exe", Logo(String::from("")));
        m.insert("file", Logo(String::from("")));
        m.insert("fish", Logo(String::from("")));
        m.insert("flac", Logo(String::from("")));
        m.insert("flac", Logo(String::from("")));
        m.insert("flv", Logo(String::from("")));
        m.insert("font", Logo(String::from("")));
        m.insert("gdoc", Logo(String::from("")));
        m.insert("gdoc", Logo(String::from("")));
        m.insert("gemfile", Logo(String::from("")));
        m.insert("gemspec", Logo(String::from("")));
        m.insert("gform", Logo(String::from("")));
        m.insert("gif", Logo(String::from("")));
        m.insert("gif", Logo(String::from("")));
        m.insert("git", Logo(String::from("")));
        m.insert("go", Logo(String::from("")));
        m.insert("gradle", Logo(String::from("")));
        m.insert("gradle", Logo(String::from("")));
        m.insert("gsheet", Logo(String::from("")));
        m.insert("gslides", Logo(String::from("")));
        m.insert("guardfile", Logo(String::from("")));
        m.insert("gz", Logo(String::from("")));
        m.insert("hs", Logo(String::from("")));
        m.insert("htm", Logo(String::from("")));
        m.insert("html", Logo(String::from("")));
        m.insert("ico", Logo(String::from("")));
        m.insert("ico", Logo(String::from("")));
        m.insert("image", Logo(String::from("")));
        m.insert("iml", Logo(String::from("")));
        m.insert("ini", Logo(String::from("")));
        m.insert("ipynb", Logo(String::from("")));
        m.insert("jar", Logo(String::from("")));
        m.insert("jar", Logo(String::from("")));
        m.insert("java", Logo(String::from("")));
        m.insert("jpeg", Logo(String::from("")));
        m.insert("jpeg", Logo(String::from("")));
        m.insert("jpg", Logo(String::from("")));
        m.insert("jpg", Logo(String::from("")));
        m.insert("js", Logo(String::from("")));
        m.insert("json", Logo(String::from("")));
        m.insert("jsx", Logo(String::from("")));
        m.insert("less", Logo(String::from("")));
        m.insert("lhs", Logo(String::from("")));
        m.insert("lhs", Logo(String::from("")));
        m.insert("license", Logo(String::from("")));
        m.insert("localized", Logo(String::from("")));
        m.insert("localized", Logo(String::from("")));
        m.insert("lock", Logo(String::from("")));
        m.insert("log", Logo(String::from("")));
        m.insert("lua", Logo(String::from("")));
        m.insert("m4a", Logo(String::from("")));
        m.insert("m4a", Logo(String::from("")));
        m.insert("markdown", Logo(String::from("")));
        m.insert("md", Logo(String::from("")));
        m.insert("mkd", Logo(String::from("")));
        m.insert("mkv", Logo(String::from("")));
        m.insert("mobi", Logo(String::from("")));
        m.insert("mobi", Logo(String::from("")));
        m.insert("mov", Logo(String::from("")));
        m.insert("mp3", Logo(String::from("")));
        m.insert("mp3", Logo(String::from("")));
        m.insert("mp4", Logo(String::from("")));
        m.insert("mustache", Logo(String::from("")));
        m.insert("npmignore", Logo(String::from("")));
        m.insert("ogg", Logo(String::from("")));
        m.insert("ogg", Logo(String::from("")));
        m.insert("ogv", Logo(String::from("")));
        m.insert("otf", Logo(String::from("")));
        m.insert("otf", Logo(String::from("")));
        m.insert("pdf", Logo(String::from("")));
        m.insert("php", Logo(String::from("")));
        m.insert("pl", Logo(String::from("")));
        m.insert("png", Logo(String::from("")));
        m.insert("png", Logo(String::from("")));
        m.insert("ppt", Logo(String::from("")));
        m.insert("pptx", Logo(String::from("")));
        m.insert("procfile", Logo(String::from("")));
        m.insert("properties", Logo(String::from("")));
        m.insert("psd", Logo(String::from("")));
        m.insert("pxm", Logo(String::from("")));
        m.insert("pxm", Logo(String::from("")));
        m.insert("py", Logo(String::from("")));
        m.insert("pyc", Logo(String::from("")));
        m.insert("r", Logo(String::from("")));
        m.insert("rakefile", Logo(String::from("")));
        m.insert("rar", Logo(String::from("")));
        m.insert("rb", Logo(String::from("")));
        m.insert("rdata", Logo(String::from("")));
        m.insert("rdb", Logo(String::from("")));
        m.insert("rdoc", Logo(String::from("")));
        m.insert("rdoc", Logo(String::from("")));
        m.insert("rds", Logo(String::from("")));
        m.insert("readme", Logo(String::from("")));
        m.insert("rspec", Logo(String::from("")));
        m.insert("rspec_parallel", Logo(String::from("")));
        m.insert("rspec_status", Logo(String::from("")));
        m.insert("rss", Logo(String::from("")));
        m.insert("ru", Logo(String::from("")));
        m.insert("rubydoc", Logo(String::from("")));
        m.insert("sass", Logo(String::from("")));
        m.insert("scala", Logo(String::from("")));
        m.insert("scss", Logo(String::from("")));
        m.insert("scss", Logo(String::from("")));
        m.insert("sh", Logo(String::from("")));
        m.insert("shell", Logo(String::from("")));
        m.insert("slim", Logo(String::from("")));
        m.insert("sqlite3", Logo(String::from("")));
        m.insert("styl", Logo(String::from("")));
        m.insert("stylus", Logo(String::from("")));
        m.insert("svg", Logo(String::from("")));
        m.insert("svg", Logo(String::from("")));
        m.insert("tar", Logo(String::from("")));
        m.insert("tex", Logo(String::from("")));
        m.insert("tiff", Logo(String::from("")));
        m.insert("tiff", Logo(String::from("")));
        m.insert("ts", Logo(String::from("")));
        m.insert("tsx", Logo(String::from("")));
        m.insert("tsx", Logo(String::from("")));
        m.insert("ttf", Logo(String::from("")));
        m.insert("ttf", Logo(String::from("")));
        m.insert("twig", Logo(String::from("")));
        m.insert("txt", Logo(String::from("")));
        m.insert("video", Logo(String::from("")));
        m.insert("vim", Logo(String::from("")));
        m.insert("wav", Logo(String::from("")));
        m.insert("wav", Logo(String::from("")));
        m.insert("webm", Logo(String::from("")));
        m.insert("webp", Logo(String::from("")));
        m.insert("webp", Logo(String::from("")));
        m.insert("windows", Logo(String::from("")));
        m.insert("woff", Logo(String::from("")));
        m.insert("woff", Logo(String::from("")));
        m.insert("woff2", Logo(String::from("")));
        m.insert("woff2", Logo(String::from("")));
        m.insert("xls", Logo(String::from("")));
        m.insert("xlsx", Logo(String::from("")));
        m.insert("xml", Logo(String::from("")));
        m.insert("xul", Logo(String::from("")));
        m.insert("yaml", Logo(String::from("")));
        m.insert("yarn.lock", Logo(String::from("")));
        m.insert("yml", Logo(String::from("")));
        m.insert("zip", Logo(String::from("")));
        m.insert("zsh", Logo(String::from("")));
        m.insert("zsh-theme", Logo(String::from("")));
        m.insert("zshrc", Logo(String::from("")));

        m
    };
}

#[derive(Debug, Clone)]
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
    pub fn from_meta(meta: &Meta) -> Self {
        // Check the known names.
        if let Some(res) = LogosByName.get(&meta.name.as_str()) {
            return res.to_owned();
        }

        // Check the known extensions.
        let extension = meta.path.extension().unwrap_or_default().to_str().unwrap();
        if let Some(res) = LogosByExtension.get(extension) {
            return res.to_owned();
        }

        // Use the default icons.
        if meta.metadata.is_dir() {
            Logo(String::from(""))
        } else {
            Logo(String::from(""))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
