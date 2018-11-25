use meta::Meta;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref IconsByName: HashMap<&'static str, Icon> = {
        let mut m = HashMap::new();

        m.insert(".Trash", Icon(String::from("")));
        m.insert(".atom", Icon(String::from("")));
        m.insert(".git", Icon(String::from("")));
        m.insert(".github", Icon(String::from("")));
        m.insert(".rvm", Icon(String::from("")));
        m.insert(".vscode", Icon(String::from("")));
        m.insert("bin", Icon(String::from("")));
        m.insert("config", Icon(String::from("")));
        m.insert("ds_store", Icon(String::from("")));
        m.insert("folder", Icon(String::from("")));
        m.insert("gitconfig", Icon(String::from("")));
        m.insert("gitignore", Icon(String::from("")));
        m.insert("gitignore_global", Icon(String::from("")));
        m.insert("gradle", Icon(String::from("")));
        m.insert("hidden", Icon(String::from("")));
        m.insert("include", Icon(String::from("")));
        m.insert("lib", Icon(String::from("")));
        m.insert("localized", Icon(String::from("")));
        m.insert("node_modules", Icon(String::from("")));
        m.insert("npmignore", Icon(String::from("")));
        m.insert("rubydoc", Icon(String::from("")));
        m.insert("yarn.lock", Icon(String::from("")));

        m
    };
}

lazy_static! {
    pub static ref IconsByExtension: HashMap<&'static str, Icon> = {
        let mut m = HashMap::new();

        m.insert("ai", Icon(String::from("")));
        m.insert("apk", Icon(String::from("")));
        m.insert("apk", Icon(String::from("")));
        m.insert("avi", Icon(String::from("")));
        m.insert("avro", Icon(String::from("")));
        m.insert("bash", Icon(String::from("")));
        m.insert("bash_history", Icon(String::from("")));
        m.insert("bash_profile", Icon(String::from("")));
        m.insert("bashrc", Icon(String::from("")));
        m.insert("bat", Icon(String::from("")));
        m.insert("bmp", Icon(String::from("")));
        m.insert("bmp", Icon(String::from("")));
        m.insert("c", Icon(String::from("")));
        m.insert("clj", Icon(String::from("")));
        m.insert("cls", Icon(String::from("")));
        m.insert("coffee", Icon(String::from("")));
        m.insert("conf", Icon(String::from("")));
        m.insert("cpp", Icon(String::from("")));
        m.insert("css", Icon(String::from("")));
        m.insert("csv", Icon(String::from("")));
        m.insert("d", Icon(String::from("")));
        m.insert("dart", Icon(String::from("")));
        m.insert("db", Icon(String::from("")));
        m.insert("diff", Icon(String::from("")));
        m.insert("doc", Icon(String::from("")));
        m.insert("docx", Icon(String::from("")));
        m.insert("docx", Icon(String::from("")));
        m.insert("ds_store", Icon(String::from("")));
        m.insert("ds_store", Icon(String::from("")));
        m.insert("ebook", Icon(String::from("")));
        m.insert("editorconfig", Icon(String::from("")));
        m.insert("env", Icon(String::from("")));
        m.insert("eot", Icon(String::from("")));
        m.insert("eot", Icon(String::from("")));
        m.insert("epub", Icon(String::from("")));
        m.insert("erb", Icon(String::from("")));
        m.insert("erl", Icon(String::from("")));
        m.insert("exe", Icon(String::from("")));
        m.insert("file", Icon(String::from("")));
        m.insert("fish", Icon(String::from("")));
        m.insert("flac", Icon(String::from("")));
        m.insert("flac", Icon(String::from("")));
        m.insert("flv", Icon(String::from("")));
        m.insert("font", Icon(String::from("")));
        m.insert("gdoc", Icon(String::from("")));
        m.insert("gdoc", Icon(String::from("")));
        m.insert("gemfile", Icon(String::from("")));
        m.insert("gemspec", Icon(String::from("")));
        m.insert("gform", Icon(String::from("")));
        m.insert("gif", Icon(String::from("")));
        m.insert("gif", Icon(String::from("")));
        m.insert("git", Icon(String::from("")));
        m.insert("go", Icon(String::from("")));
        m.insert("gradle", Icon(String::from("")));
        m.insert("gradle", Icon(String::from("")));
        m.insert("gsheet", Icon(String::from("")));
        m.insert("gslides", Icon(String::from("")));
        m.insert("guardfile", Icon(String::from("")));
        m.insert("gz", Icon(String::from("")));
        m.insert("hs", Icon(String::from("")));
        m.insert("htm", Icon(String::from("")));
        m.insert("html", Icon(String::from("")));
        m.insert("ico", Icon(String::from("")));
        m.insert("ico", Icon(String::from("")));
        m.insert("image", Icon(String::from("")));
        m.insert("iml", Icon(String::from("")));
        m.insert("ini", Icon(String::from("")));
        m.insert("ipynb", Icon(String::from("")));
        m.insert("jar", Icon(String::from("")));
        m.insert("jar", Icon(String::from("")));
        m.insert("java", Icon(String::from("")));
        m.insert("jpeg", Icon(String::from("")));
        m.insert("jpeg", Icon(String::from("")));
        m.insert("jpg", Icon(String::from("")));
        m.insert("jpg", Icon(String::from("")));
        m.insert("js", Icon(String::from("")));
        m.insert("json", Icon(String::from("")));
        m.insert("jsx", Icon(String::from("")));
        m.insert("less", Icon(String::from("")));
        m.insert("lhs", Icon(String::from("")));
        m.insert("lhs", Icon(String::from("")));
        m.insert("license", Icon(String::from("")));
        m.insert("localized", Icon(String::from("")));
        m.insert("localized", Icon(String::from("")));
        m.insert("lock", Icon(String::from("")));
        m.insert("log", Icon(String::from("")));
        m.insert("lua", Icon(String::from("")));
        m.insert("m4a", Icon(String::from("")));
        m.insert("m4a", Icon(String::from("")));
        m.insert("markdown", Icon(String::from("")));
        m.insert("md", Icon(String::from("")));
        m.insert("mkd", Icon(String::from("")));
        m.insert("mkv", Icon(String::from("")));
        m.insert("mobi", Icon(String::from("")));
        m.insert("mobi", Icon(String::from("")));
        m.insert("mov", Icon(String::from("")));
        m.insert("mp3", Icon(String::from("")));
        m.insert("mp3", Icon(String::from("")));
        m.insert("mp4", Icon(String::from("")));
        m.insert("mustache", Icon(String::from("")));
        m.insert("npmignore", Icon(String::from("")));
        m.insert("ogg", Icon(String::from("")));
        m.insert("ogg", Icon(String::from("")));
        m.insert("ogv", Icon(String::from("")));
        m.insert("otf", Icon(String::from("")));
        m.insert("otf", Icon(String::from("")));
        m.insert("pdf", Icon(String::from("")));
        m.insert("php", Icon(String::from("")));
        m.insert("pl", Icon(String::from("")));
        m.insert("png", Icon(String::from("")));
        m.insert("png", Icon(String::from("")));
        m.insert("ppt", Icon(String::from("")));
        m.insert("pptx", Icon(String::from("")));
        m.insert("procfile", Icon(String::from("")));
        m.insert("properties", Icon(String::from("")));
        m.insert("psd", Icon(String::from("")));
        m.insert("pxm", Icon(String::from("")));
        m.insert("pxm", Icon(String::from("")));
        m.insert("py", Icon(String::from("")));
        m.insert("pyc", Icon(String::from("")));
        m.insert("r", Icon(String::from("")));
        m.insert("rakefile", Icon(String::from("")));
        m.insert("rar", Icon(String::from("")));
        m.insert("rb", Icon(String::from("")));
        m.insert("rdata", Icon(String::from("")));
        m.insert("rdb", Icon(String::from("")));
        m.insert("rdoc", Icon(String::from("")));
        m.insert("rdoc", Icon(String::from("")));
        m.insert("rds", Icon(String::from("")));
        m.insert("readme", Icon(String::from("")));
        m.insert("rspec", Icon(String::from("")));
        m.insert("rspec_parallel", Icon(String::from("")));
        m.insert("rspec_status", Icon(String::from("")));
        m.insert("rss", Icon(String::from("")));
        m.insert("ru", Icon(String::from("")));
        m.insert("rubydoc", Icon(String::from("")));
        m.insert("sass", Icon(String::from("")));
        m.insert("scala", Icon(String::from("")));
        m.insert("scss", Icon(String::from("")));
        m.insert("scss", Icon(String::from("")));
        m.insert("sh", Icon(String::from("")));
        m.insert("shell", Icon(String::from("")));
        m.insert("slim", Icon(String::from("")));
        m.insert("sqlite3", Icon(String::from("")));
        m.insert("styl", Icon(String::from("")));
        m.insert("stylus", Icon(String::from("")));
        m.insert("svg", Icon(String::from("")));
        m.insert("svg", Icon(String::from("")));
        m.insert("tar", Icon(String::from("")));
        m.insert("tex", Icon(String::from("")));
        m.insert("tiff", Icon(String::from("")));
        m.insert("tiff", Icon(String::from("")));
        m.insert("ts", Icon(String::from("")));
        m.insert("tsx", Icon(String::from("")));
        m.insert("tsx", Icon(String::from("")));
        m.insert("ttf", Icon(String::from("")));
        m.insert("ttf", Icon(String::from("")));
        m.insert("twig", Icon(String::from("")));
        m.insert("txt", Icon(String::from("")));
        m.insert("video", Icon(String::from("")));
        m.insert("vim", Icon(String::from("")));
        m.insert("wav", Icon(String::from("")));
        m.insert("wav", Icon(String::from("")));
        m.insert("webm", Icon(String::from("")));
        m.insert("webp", Icon(String::from("")));
        m.insert("webp", Icon(String::from("")));
        m.insert("windows", Icon(String::from("")));
        m.insert("woff", Icon(String::from("")));
        m.insert("woff", Icon(String::from("")));
        m.insert("woff2", Icon(String::from("")));
        m.insert("woff2", Icon(String::from("")));
        m.insert("xls", Icon(String::from("")));
        m.insert("xlsx", Icon(String::from("")));
        m.insert("xml", Icon(String::from("")));
        m.insert("xul", Icon(String::from("")));
        m.insert("yaml", Icon(String::from("")));
        m.insert("yarn.lock", Icon(String::from("")));
        m.insert("yml", Icon(String::from("")));
        m.insert("zip", Icon(String::from("")));
        m.insert("zsh", Icon(String::from("")));
        m.insert("zsh-theme", Icon(String::from("")));
        m.insert("zshrc", Icon(String::from("")));

        m
    };
}

#[derive(Debug, Clone)]
pub struct Icon(String);

impl Into<String> for Icon {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Icon {
    pub fn from_meta(meta: &Meta) -> Self {
        // Check the known names.
        if let Some(res) = IconsByName.get(&meta.name.as_str()) {
            return res.to_owned();
        }

        // Check the known extensions.
        let extension = meta.path.extension().unwrap_or_default().to_str().unwrap();
        if let Some(res) = IconsByExtension.get(extension) {
            return res.to_owned();
        }

        // Use the default icons.
        if meta.metadata.is_dir() {
            Icon(String::from(""))
        } else {
            Icon(String::from(""))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
