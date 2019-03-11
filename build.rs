// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
extern crate clap;
extern crate version_check;
extern crate phf_codegen;

use clap::Shell;
use std::env;
use std::fs::File;
use std::fs;
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::process::exit;

include!("src/app.rs");

fn main() {
    match version_check::is_min_version("1.31.0") {
        Some((true, _)) => {}
        // rustc version too small or can't figure it out
        _ => {
            writeln!(&mut io::stderr(), "'lsd' requires rustc >= 1.31.0").unwrap();
            exit(1);
        }
    }

    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or(std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut app = build();
    app.gen_completions("lsd", Shell::Bash, &outdir);
    app.gen_completions("lsd", Shell::Fish, &outdir);
    app.gen_completions("lsd", Shell::Zsh, &outdir);
    app.gen_completions("lsd", Shell::PowerShell, &outdir);

    {
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("default_icons_by_name.rs");
        let mut file = BufWriter::new(File::create(&path).unwrap());

        write!(&mut file, "static DEFAULT_ICONS_BY_NAME: phf::Map<&'static str, &'static str> = ").unwrap();
        phf_codegen::Map::new()
            .entry(".Trash", "\"\u{f1f8}\"") // ""
            .entry(".atom", "\"\u{e764}\"") // ""
            .entry(".bashprofile", "\"\u{e615}\"") // ""
            .entry(".bashrc", "\"\u{f489}\"") // ""
            .entry(".git", "\"\u{f1d3}\"") // ""
            .entry(".gitconfig", "\"\u{f1d3}\"") // ""
            .entry(".github", "\"\u{f408}\"") // ""
            .entry(".gitignore", "\"\u{f1d3}\"") // ""
            .entry(".rvm", "\"\u{e21e}\"") // ""
            .entry(".vimrc", "\"\u{e62b}\"") // ""
            .entry(".vscode", "\"\u{e70c}\"") // ""
            .entry(".zshrc", "\"\u{f489}\"") // ""
            .entry("bin", "\"\u{e5fc}\"") // ""
            .entry("config", "\"\u{e5fc}\"") // ""
            .entry("docker-compose.yml", "\"\u{f308}\"") // ""
            .entry("dockerfile", "\"\u{f308}\"") // ""
            .entry("ds_store", "\"\u{f179}\"") // ""
            .entry("gitignore_global", "\"\u{f1d3}\"") // ""
            .entry("gradle", "\"\u{e70e}\"") // ""
            .entry("gruntfile.coffee", "\"\u{e611}\"") // ""
            .entry("gruntfile.js", "\"\u{e611}\"") // ""
            .entry("gruntfile.ls", "\"\u{e611}\"") // ""
            .entry("gulpfile.coffee", "\"\u{e610}\"") // ""
            .entry("gulpfile.js", "\"\u{e610}\"") // ""
            .entry("gulpfile.ls", "\"\u{e610}\"") // ""
            .entry("hidden", "\"\u{f023}\"") // ""
            .entry("include", "\"\u{e5fc}\"") // ""
            .entry("lib", "\"\u{f121}\"") // ""
            .entry("localized", "\"\u{f179}\"") // ""
            .entry("node_modules", "\"\u{e718}\"") // ""
            .entry("npmignore", "\"\u{e71e}\"") // ""
            .entry("rubydoc", "\"\u{e73b}\"") // ""
            .entry("yarn.lock", "\"\u{e718}\"") // ""
            .build(&mut file)
            .unwrap();
        write!(&mut file, ";\n").unwrap();
    }

    {
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("default_icons_by_extension.rs");
        let mut file = BufWriter::new(File::create(&path).unwrap());

        write!(&mut file, "static DEFAULT_ICONS_BY_EXTENSION: phf::Map<&'static str, &'static str> = ").unwrap();
        phf_codegen::Map::new()
            .entry("apk", "\"\u{e70e}\"") // ""
            .entry("avi", "\"\u{f03d}\"") // ""
            .entry("avro", "\"\u{e60b}\"") // ""
            .entry("awk", "\"\u{f489}\"") // ""
            .entry("bash", "\"\u{f489}\"") // ""
            .entry("bash_history", "\"\u{f489}\"") // ""
            .entry("bash_profile", "\"\u{f489}\"") // ""
            .entry("bashrc", "\"\u{f489}\"") // ""
            .entry("bat", "\"\u{f17a}\"") // ""
            .entry("bmp", "\"\u{f1c5}\"") // ""
            .entry("c", "\"\u{e61e}\"") // ""
            .entry("c++", "\"\u{e61d}\"") // ""
            .entry("cc", "\"\u{e61d}\"") // ""
            .entry("cfg", "\"\u{e615}\"") // ""
            .entry("clj", "\"\u{e768}\"") // ""
            .entry("cljs", "\"\u{e76a}\"") // ""
            .entry("cls", "\"\u{e600}\"") // ""
            .entry("coffee", "\"\u{f0f4}\"") // ""
            .entry("conf", "\"\u{e615}\"") // ""
            .entry("cp", "\"\u{e61d}\"") // ""
            .entry("cpp", "\"\u{e61d}\"") // ""
            .entry("csh", "\"\u{f489}\"") // ""
            .entry("css", "\"\u{e749}\"") // ""
            .entry("csv", "\"\u{f1c3}\"") // ""
            .entry("cxx", "\"\u{e61d}\"") // ""
            .entry("d", "\"\u{e7af}\"") // ""
            .entry("dart", "\"\u{e798}\"") // ""
            .entry("db", "\"\u{f1c0}\"") // ""
            .entry("diff", "\"\u{f440}\"") // ""
            .entry("doc", "\"\u{f1c2}\"") // ""
            .entry("docx", "\"\u{f1c2}\"") // ""
            .entry("ds_store", "\"\u{f179}\"") // ""
            .entry("dump", "\"\u{f1c0}\"") // ""
            .entry("ebook", "\"\u{e28b}\"") // ""
            .entry("editorconfig", "\"\u{e615}\"") // ""
            .entry("ejs", "\"\u{e618}\"") // ""
            .entry("env", "\"\u{f462}\"") // ""
            .entry("eot", "\"\u{f031}\"") // ""
            .entry("epub", "\"\u{e28a}\"") // ""
            .entry("erb", "\"\u{e73b}\"") // ""
            .entry("erl", "\"\u{e7b1}\"") // ""
            .entry("exe", "\"\u{f17a}\"") // ""
            .entry("fish", "\"\u{f489}\"") // ""
            .entry("flac", "\"\u{f001}\"") // ""
            .entry("flv", "\"\u{f03d}\"") // ""
            .entry("font", "\"\u{f031}\"") // ""
            .entry("gdoc", "\"\u{f1c2}\"") // ""
            .entry("gemfile", "\"\u{e21e}\"") // ""
            .entry("gemspec", "\"\u{e21e}\"") // ""
            .entry("gform", "\"\u{f298}\"") // ""
            .entry("gif", "\"\u{f1c5}\"") // ""
            .entry("git", "\"\u{f1d3}\"") // ""
            .entry("go", "\"\u{e626}\"") // ""
            .entry("gradle", "\"\u{e70e}\"") // ""
            .entry("gsheet", "\"\u{f1c3}\"") // ""
            .entry("gslides", "\"\u{f1c4}\"") // ""
            .entry("guardfile", "\"\u{e21e}\"") // ""
            .entry("gz", "\"\u{f410}\"") // ""
            .entry("h", "\"\u{f0fd}\"") // ""
            .entry("hbs", "\"\u{e60f}\"") // ""
            .entry("hpp", "\"\u{f0fd}\"") // ""
            .entry("hs", "\"\u{e777}\"") // ""
            .entry("htm", "\"\u{f13b}\"") // ""
            .entry("html", "\"\u{f13b}\"") // ""
            .entry("hxx", "\"\u{f0fd}\"") // ""
            .entry("ico", "\"\u{f1c5}\"") // ""
            .entry("image", "\"\u{f1c5}\"") // ""
            .entry("iml", "\"\u{e7b5}\"") // ""
            .entry("ini", "\"\u{f17a}\"") // ""
            .entry("ipynb", "\"\u{e606}\"") // ""
            .entry("jar", "\"\u{e204}\"") // ""
            .entry("java", "\"\u{e204}\"") // ""
            .entry("jpeg", "\"\u{f1c5}\"") // ""
            .entry("jpg", "\"\u{f1c5}\"") // ""
            .entry("js", "\"\u{e74e}\"") // ""
            .entry("json", "\"\u{e60b}\"") // ""
            .entry("jsx", "\"\u{e7ba}\"") // ""
            .entry("ksh", "\"\u{f489}\"") // ""
            .entry("less", "\"\u{e758}\"") // ""
            .entry("lhs", "\"\u{e777}\"") // ""
            .entry("license", "\"\u{f48a}\"") // ""
            .entry("localized", "\"\u{f179}\"") // ""
            .entry("lock", "\"\u{e21e}\"") // ""
            .entry("log", "\"\u{f18d}\"") // ""
            .entry("lua", "\"\u{e620}\"") // ""
            .entry("m4a", "\"\u{f001}\"") // ""
            .entry("markdown", "\"\u{f48a}\"") // ""
            .entry("md", "\"\u{f48a}\"") // ""
            .entry("mkd", "\"\u{f48a}\"") // ""
            .entry("mkv", "\"\u{f03d}\"") // ""
            .entry("mobi", "\"\u{e28b}\"") // ""
            .entry("mov", "\"\u{f03d}\"") // ""
            .entry("mp3", "\"\u{f001}\"") // ""
            .entry("mp4", "\"\u{f03d}\"") // ""
            .entry("mustache", "\"\u{e60f}\"") // ""
            .entry("npmignore", "\"\u{e71e}\"") // ""
            .entry("ogg", "\"\u{f001}\"") // ""
            .entry("ogv", "\"\u{f03d}\"") // ""
            .entry("otf", "\"\u{f031}\"") // ""
            .entry("pdf", "\"\u{f1c1}\"") // ""
            .entry("php", "\"\u{e73d}\"") // ""
            .entry("pl", "\"\u{e769}\"") // ""
            .entry("png", "\"\u{f1c5}\"") // ""
            .entry("ppt", "\"\u{f1c4}\"") // ""
            .entry("pptx", "\"\u{f1c4}\"") // ""
            .entry("procfile", "\"\u{e21e}\"") // ""
            .entry("properties", "\"\u{e60b}\"") // ""
            .entry("ps1", "\"\u{f489}\"") // ""
            .entry("psd", "\"\u{e7b8}\"") // ""
            .entry("pxm", "\"\u{f1c5}\"") // ""
            .entry("py", "\"\u{e606}\"") // ""
            .entry("pyc", "\"\u{e606}\"") // ""
            .entry("r", "\"\u{f25d}\"") // ""
            .entry("rakefile", "\"\u{e21e}\"") // ""
            .entry("rar", "\"\u{f410}\"") // ""
            .entry("rb", "\"\u{e21e}\"") // ""
            .entry("rdata", "\"\u{f25d}\"") // ""
            .entry("rdb", "\"\u{e76d}\"") // ""
            .entry("rdoc", "\"\u{f48a}\"") // ""
            .entry("rds", "\"\u{f25d}\"") // ""
            .entry("readme", "\"\u{f48a}\"") // ""
            .entry("rlib", "\"\u{e7a8}\"") // ""
            .entry("rmd", "\"\u{f48a}\"") // ""
            .entry("rs", "\"\u{e7a8}\"") // ""
            .entry("rspec", "\"\u{e21e}\"") // ""
            .entry("rspec_parallel", "\"\u{e21e}\"") // ""
            .entry("rspec_status", "\"\u{e21e}\"") // ""
            .entry("rss", "\"\u{f09e}\"") // ""
            .entry("ru", "\"\u{e21e}\"") // ""
            .entry("rubydoc", "\"\u{e73b}\"") // ""
            .entry("sass", "\"\u{e603}\"") // ""
            .entry("scala", "\"\u{e737}\"") // ""
            .entry("scss", "\"\u{e749}\"") // ""
            .entry("sh", "\"\u{f489}\"") // ""
            .entry("shell", "\"\u{f489}\"") // ""
            .entry("slim", "\"\u{e73b}\"") // ""
            .entry("sql", "\"\u{f1c0}\"") // ""
            .entry("sqlite3", "\"\u{e7c4}\"") // ""
            .entry("styl", "\"\u{e600}\"") // ""
            .entry("stylus", "\"\u{e600}\"") // ""
            .entry("svg", "\"\u{f1c5}\"") // ""
            .entry("swift", "\"\u{e755}\"") // ""
            .entry("tar", "\"\u{f410}\"") // ""
            .entry("tex", "\"\u{e600}\"") // ""
            .entry("tiff", "\"\u{f1c5}\"") // ""
            .entry("ts", "\"\u{e628}\"") // ""
            .entry("tsx", "\"\u{e7ba}\"") // ""
            .entry("ttf", "\"\u{f031}\"") // ""
            .entry("twig", "\"\u{e61c}\"") // ""
            .entry("txt", "\"\u{f15c}\"") // ""
            .entry("video", "\"\u{f03d}\"") // ""
            .entry("vim", "\"\u{e62b}\"") // ""
            .entry("vue", "\"\u{fd42}\"") // "﵂"
            .entry("wav", "\"\u{f001}\"") // ""
            .entry("webm", "\"\u{f03d}\"") // ""
            .entry("webp", "\"\u{f1c5}\"") // ""
            .entry("windows", "\"\u{f17a}\"") // ""
            .entry("woff", "\"\u{f031}\"") // ""
            .entry("woff2", "\"\u{f031}\"") // ""
            .entry("xls", "\"\u{f1c3}\"") // ""
            .entry("xlsx", "\"\u{f1c3}\"") // ""
            .entry("xml", "\"\u{e619}\"") // ""
            .entry("xul", "\"\u{e619}\"") // ""
            .entry("yaml", "\"\u{f481}\"") // ""
            .entry("yml", "\"\u{f481}\"") // ""
            .entry("zip", "\"\u{f410}\"") // ""
            .entry("zsh", "\"\u{f489}\"") // ""
            .entry("zsh-theme", "\"\u{f489}\"") // ""
            .entry("zshrc", "\"\u{f489}\"") // ""
            .build(&mut file)
            .unwrap();
        write!(&mut file, ";\n").unwrap();
    }

    {
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("empty_icon_map.rs");
        let mut file = BufWriter::new(File::create(&path).unwrap());

        write!(&mut file, "static EMPTY_ICON_MAP: phf::Map<&'static str, &'static str> = ").unwrap();
        let map : phf_codegen::Map<&'static str> = phf_codegen::Map::new();
        map.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }
}
