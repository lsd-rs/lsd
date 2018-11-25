use meta::Meta;
use std::collections::HashMap;

lazy_static! {
    pub static ref IconsByName: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        m.insert(".Trash", "");
        m.insert(".atom", "");
        m.insert(".git", "");
        m.insert(".github", "");
        m.insert(".rvm", "");
        m.insert(".vscode", "");
        m.insert("bin", "");
        m.insert("config", "");
        m.insert("ds_store", "");
        m.insert("folder", "");
        m.insert("gitconfig", "");
        m.insert("gitignore", "");
        m.insert("gitignore_global", "");
        m.insert("gradle", "");
        m.insert("hidden", "");
        m.insert("include", "");
        m.insert("lib", "");
        m.insert("localized", "");
        m.insert("node_modules", "");
        m.insert("npmignore", "");
        m.insert("rubydoc", "");
        m.insert("yarn.lock", "");

        m
    };
}

lazy_static! {
    pub static ref IconsByExtension: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        m.insert("ai", "");
        m.insert("apk", "");
        m.insert("apk", "");
        m.insert("avi", "");
        m.insert("avro", "");
        m.insert("bash", "");
        m.insert("bash_history", "");
        m.insert("bash_profile", "");
        m.insert("bashrc", "");
        m.insert("bat", "");
        m.insert("bmp", "");
        m.insert("bmp", "");
        m.insert("c", "");
        m.insert("cfg", "");
        m.insert("clj", "");
        m.insert("cls", "");
        m.insert("coffee", "");
        m.insert("conf", "");
        m.insert("cpp", "");
        m.insert("css", "");
        m.insert("csv", "");
        m.insert("d", "");
        m.insert("dart", "");
        m.insert("db", "");
        m.insert("diff", "");
        m.insert("doc", "");
        m.insert("docx", "");
        m.insert("docx", "");
        m.insert("ds_store", "");
        m.insert("ds_store", "");
        m.insert("ebook", "");
        m.insert("editorconfig", "");
        m.insert("env", "");
        m.insert("eot", "");
        m.insert("eot", "");
        m.insert("epub", "");
        m.insert("erb", "");
        m.insert("erl", "");
        m.insert("exe", "");
        m.insert("file", "");
        m.insert("fish", "");
        m.insert("flac", "");
        m.insert("flac", "");
        m.insert("flv", "");
        m.insert("font", "");
        m.insert("gdoc", "");
        m.insert("gdoc", "");
        m.insert("gemfile", "");
        m.insert("gemspec", "");
        m.insert("gform", "");
        m.insert("gif", "");
        m.insert("gif", "");
        m.insert("git", "");
        m.insert("go", "");
        m.insert("gradle", "");
        m.insert("gradle", "");
        m.insert("gsheet", "");
        m.insert("gslides", "");
        m.insert("guardfile", "");
        m.insert("gz", "");
        m.insert("hs", "");
        m.insert("htm", "");
        m.insert("html", "");
        m.insert("ico", "");
        m.insert("ico", "");
        m.insert("image", "");
        m.insert("iml", "");
        m.insert("ini", "");
        m.insert("ipynb", "");
        m.insert("jar", "");
        m.insert("jar", "");
        m.insert("java", "");
        m.insert("jpeg", "");
        m.insert("jpeg", "");
        m.insert("jpg", "");
        m.insert("jpg", "");
        m.insert("js", "");
        m.insert("json", "");
        m.insert("jsx", "");
        m.insert("less", "");
        m.insert("lhs", "");
        m.insert("lhs", "");
        m.insert("license", "");
        m.insert("localized", "");
        m.insert("localized", "");
        m.insert("lock", "");
        m.insert("log", "");
        m.insert("lua", "");
        m.insert("m4a", "");
        m.insert("m4a", "");
        m.insert("markdown", "");
        m.insert("md", "");
        m.insert("mkd", "");
        m.insert("mkv", "");
        m.insert("mobi", "");
        m.insert("mobi", "");
        m.insert("mov", "");
        m.insert("mp3", "");
        m.insert("mp3", "");
        m.insert("mp4", "");
        m.insert("mustache", "");
        m.insert("npmignore", "");
        m.insert("ogg", "");
        m.insert("ogg", "");
        m.insert("ogv", "");
        m.insert("otf", "");
        m.insert("otf", "");
        m.insert("pdf", "");
        m.insert("php", "");
        m.insert("pl", "");
        m.insert("png", "");
        m.insert("png", "");
        m.insert("ppt", "");
        m.insert("pptx", "");
        m.insert("procfile", "");
        m.insert("properties", "");
        m.insert("psd", "");
        m.insert("pxm", "");
        m.insert("pxm", "");
        m.insert("py", "");
        m.insert("pyc", "");
        m.insert("r", "");
        m.insert("rakefile", "");
        m.insert("rar", "");
        m.insert("rb", "");
        m.insert("rdata", "");
        m.insert("rdb", "");
        m.insert("rdoc", "");
        m.insert("rdoc", "");
        m.insert("rds", "");
        m.insert("readme", "");
        m.insert("rspec", "");
        m.insert("rspec_parallel", "");
        m.insert("rspec_status", "");
        m.insert("rss", "");
        m.insert("ru", "");
        m.insert("rubydoc", "");
        m.insert("sass", "");
        m.insert("scala", "");
        m.insert("scss", "");
        m.insert("scss", "");
        m.insert("sh", "");
        m.insert("shell", "");
        m.insert("slim", "");
        m.insert("sqlite3", "");
        m.insert("styl", "");
        m.insert("stylus", "");
        m.insert("svg", "");
        m.insert("svg", "");
        m.insert("tar", "");
        m.insert("tex", "");
        m.insert("tiff", "");
        m.insert("tiff", "");
        m.insert("ts", "");
        m.insert("tsx", "");
        m.insert("tsx", "");
        m.insert("ttf", "");
        m.insert("ttf", "");
        m.insert("twig", "");
        m.insert("txt", "");
        m.insert("video", "");
        m.insert("vim", "");
        m.insert("wav", "");
        m.insert("wav", "");
        m.insert("webm", "");
        m.insert("webp", "");
        m.insert("webp", "");
        m.insert("windows", "");
        m.insert("woff", "");
        m.insert("woff", "");
        m.insert("woff2", "");
        m.insert("woff2", "");
        m.insert("xls", "");
        m.insert("xlsx", "");
        m.insert("xml", "");
        m.insert("xul", "");
        m.insert("yaml", "");
        m.insert("yarn.lock", "");
        m.insert("yml", "");
        m.insert("zip", "");
        m.insert("zsh", "");
        m.insert("zsh-theme", "");
        m.insert("zshrc", "");

        m
    };
}

pub fn from_meta(meta: &Meta) -> &'static str {
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
        ""
    } else {
        ""
    }
}
