use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use crate::hashmap;
use crate::meta::{FileType, Name};
use fxhash::FxHashMap;

pub struct Icons {
    display_icons: bool,
    icons_by_name: FxHashMap<&'static str, char>,
    icons_by_extension: FxHashMap<&'static str, char>,
    default_folder_icon: char,
    default_file_icon: char,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Theme {
    NoIcon,
    Fancy,
    Unicode,
}

impl Icons {
    pub fn new(theme: Theme) -> Self {
        let display_icons = theme != Theme::NoIcon;
        if theme == Theme::Fancy {
            Self {
                display_icons,
                icons_by_name: default_icons_by_name(),
                icons_by_extension: default_icons_by_extension(),
                default_file_icon: '\u{f016}',   // 
                default_folder_icon: '\u{f115}', // 
            }
        } else {
            Self {
                display_icons,
                icons_by_name: FxHashMap::default(),
                icons_by_extension: FxHashMap::default(),
                default_file_icon: '\u{1f5cb}',   // 🗋
                default_folder_icon: '\u{1f5c1}', // 🗁
            }
        }
    }

    pub fn get(&self, name: &Name) -> Option<char> {
        if !self.display_icons {
            return None;
        }

        // Check file types
        match name.file_type() {
            FileType::Directory { .. } => Some(self.default_folder_icon),
            FileType::SymLink { is_dir: true } => Some('\u{f482}'), // ""
            FileType::SymLink { is_dir: false } => Some('\u{f481}'), // ""
            FileType::Socket => Some('\u{f6a7}'),                   // ""
            FileType::Pipe => Some('\u{f731}'),                     // ""
            FileType::CharDevice => Some('\u{e601}'),               // ""
            FileType::BlockDevice => Some('\u{fc29}'),              // "ﰩ"
            FileType::Special => Some('\u{f2dc}'),                  // ""
            FileType::File { .. } => self
                .icons_by_name
                .get(name.get_name().to_ascii_lowercase().as_str())
                .or_else(|| {
                    if let Some(ext) = name.extension() {
                        return self
                            .icons_by_extension
                            .get(ext.to_ascii_lowercase().as_str());
                    }

                    let mut reader = BufReader::new(File::open(name.get_path()).ok()?);
                    let mut buf = [0; 2];
                    reader.read_exact(&mut buf).ok()?;
                    if b"#!" != &buf {
                        return None;
                    }
                    let line = reader.lines().next()?.ok()?;
                    let end_path = line.split('/').next_back()?;

                    let command = if end_path.starts_with("env") {
                        end_path.split(' ').next_back() // #!/bin/env bash
                    } else {
                        end_path.split(' ').next() // #!/bin/bash -vv
                    }?;
                    self.icons_by_shebang(command)
                })
                .cloned()
                .or(Some(self.default_file_icon)),
        }
    }

    fn icons_by_shebang(&self, cmd: &str) -> Option<&char> {
        // This function tries to get an icon from the interpreter.
        // First we check if interpreter is also an extension e.g. php, lua
        // otherwise we check for when the interpreter name differs from the extension
        if let Some(icon) = self.icons_by_extension.get(cmd) {
            Some(icon)
        } else if cmd.ends_with("sh") {
            self.icons_by_extension.get("sh")
        } else if cmd.starts_with("python") {
            self.icons_by_extension.get("py")
        } else if cmd.starts_with("node") {
            self.icons_by_extension.get("js")
        } else if cmd.starts_with("perl") {
            self.icons_by_extension.get("pl")
        } else if cmd.starts_with("ruby") {
            self.icons_by_extension.get("rb")
        } else {
            None
        }
    }
}

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
fn default_icons_by_name() -> FxHashMap<&'static str, char> {
    // Note: names must be lower-case
    hashmap! {
        /*  */ ".trash"=> '\u{f1f8}',
        /*  */ ".atom" => '\u{e764}',
        /*  */ ".bashprofile" => '\u{e615}',
        /*  */ ".bashrc" => '\u{f489}',
        /*  */ ".git" => '\u{f1d3}',
        /*  */ ".github" => '\u{f408}',
        /*  */ ".gitignore" => '\u{f1d3}',
        /*  */ ".gitmodules" => '\u{f1d3}',
        /*  */ ".rvm" => '\u{e21e}',
        /*  */ ".vimrc" => '\u{e62b}',
        /*  */ ".vscode" => '\u{e70c}',
        /*  */ ".zshrc" => '\u{f489}',
        /*  */ "bin" => '\u{e5fc}',
        /*  */ "config" => '\u{e5fc}',
        /*  */ "docker-compose.yml" => '\u{f308}',
        /*  */ "dockerfile" => '\u{f308}',
        /*  */ "ds_store" => '\u{f179}',
        /*  */ "gitignore_global" => '\u{f1d3}',
        /*  */ "gradle" => '\u{e70e}',
        /*  */ "gruntfile.coffee" => '\u{e611}',
        /*  */ "gruntfile.js" => '\u{e611}',
        /*  */ "gruntfile.ls" => '\u{e611}',
        /*  */ "gulpfile.coffee" => '\u{e610}',
        /*  */ "gulpfile.js" => '\u{e610}',
        /*  */ "gulpfile.ls" => '\u{e610}',
        /*  */ "hidden" => '\u{f023}',
        /*  */ "include" => '\u{e5fc}',
        /*  */ "lib" => '\u{f121}',
        /*  */ "localized" => '\u{f179}',
        /*  */ "node_modules" => '\u{e718}',
        /*  */ "npmignore" => '\u{e71e}',
        /*  */ "rubydoc" => '\u{e73b}',
    }
}

fn default_icons_by_extension() -> FxHashMap<&'static str, char> {
    // Note: extensions must be lower-case
    hashmap! {
        /*  */ "7z" => '\u{f410}',
        /*  */ "apk" => '\u{e70e}',
        /*  */ "avi" => '\u{f03d}',
        /*  */ "avro" => '\u{e60b}',
        /*  */ "awk" => '\u{f489}',
        /*  */ "bak" => '\u{f56e}',
        /*  */ "bash" => '\u{f489}',
        /*  */ "bash_history" => '\u{f489}',
        /*  */ "bash_profile" => '\u{f489}',
        /*  */ "bashrc" => '\u{f489}',
        /*  */ "bat" => '\u{f17a}',
        /* 蘿*/ "bio" => '\u{f910}',
        /*  */ "bmp" => '\u{f1c5}',
        /*  */ "bz2" => '\u{f410}',
        /*  */ "c" => '\u{e61e}',
        /*  */ "c++" => '\u{e61d}',
        /*  */ "cc" => '\u{e61d}',
        /*  */ "cfg" => '\u{e615}',
        /*  */ "clj" => '\u{e768}',
        /*  */ "cljs" => '\u{e76a}',
        /*  */ "cls" => '\u{e600}',
        /*  */ "coffee" => '\u{f0f4}',
        /*  */ "conf" => '\u{e615}',
        /*  */ "cp" => '\u{e61d}',
        /*  */ "cpp" => '\u{e61d}',
        /*  */ "cs" => '\u{f81a}',
        /*  */ "cshtml" => '\u{f1fa}',
        /*  */ "csproj" => '\u{f81a}',
        /*  */ "csx" => '\u{f81a}',
        /*  */ "csh" => '\u{f489}',
        /*  */ "css" => '\u{e749}',
        /*  */ "csv" => '\u{f1c3}',
        /*  */ "cxx" => '\u{e61d}',
        /*  */ "d" => '\u{e7af}',
        /*  */ "dart" => '\u{e798}',
        /*  */ "db" => '\u{f1c0}',
        /*  */ "diff" => '\u{f440}',
        /*  */ "doc" => '\u{f1c2}',
        /*  */ "docx" => '\u{f1c2}',
        /*  */ "ds_store" => '\u{f179}',
        /*  */ "dump" => '\u{f1c0}',
        /*  */ "ebook" => '\u{e28b}',
        /*  */ "editorconfig" => '\u{e615}',
        /*  */ "ejs" => '\u{e618}',
        /*  */ "elm" => '\u{e62c}',
        /*  */ "env" => '\u{f462}',
        /*  */ "eot" => '\u{f031}',
        /*  */ "epub" => '\u{e28a}',
        /*  */ "erb" => '\u{e73b}',
        /*  */ "erl" => '\u{e7b1}',
        /*  */ "exe" => '\u{f17a}',
        /*  */ "ex" => '\u{e62d}',
        /*  */ "exs" => '\u{e62d}',
        /*  */ "fish" => '\u{f489}',
        /*  */ "flac" => '\u{f001}',
        /*  */ "flv" => '\u{f03d}',
        /*  */ "font" => '\u{f031}',
        /* 蘿*/ "fpl" => '\u{f910}',
        /*  */ "gdoc" => '\u{f1c2}',
        /*  */ "gemfile" => '\u{e21e}',
        /*  */ "gemspec" => '\u{e21e}',
        /*  */ "gform" => '\u{f298}',
        /*  */ "gif" => '\u{f1c5}',
        /*  */ "git" => '\u{f1d3}',
        /*  */ "go" => '\u{e626}',
        /*  */ "gradle" => '\u{e70e}',
        /*  */ "gsheet" => '\u{f1c3}',
        /*  */ "gslides" => '\u{f1c4}',
        /*  */ "guardfile" => '\u{e21e}',
        /*  */ "gz" => '\u{f410}',
        /*  */ "h" => '\u{f0fd}',
        /*  */ "hbs" => '\u{e60f}',
        /*  */ "hpp" => '\u{f0fd}',
        /*  */ "hs" => '\u{e777}',
        /*  */ "htm" => '\u{f13b}',
        /*  */ "html" => '\u{f13b}',
        /*  */ "hxx" => '\u{f0fd}',
        /*  */ "ico" => '\u{f1c5}',
        /*  */ "image" => '\u{f1c5}',
        /*  */ "iml" => '\u{e7b5}',
        /*  */ "ini" => '\u{e615}',
        /*  */ "ipynb" => '\u{e606}',
        /*  */ "jar" => '\u{e204}',
        /*  */ "java" => '\u{e204}',
        /*  */ "jpeg" => '\u{f1c5}',
        /*  */ "jpg" => '\u{f1c5}',
        /*  */ "js" => '\u{e74e}',
        /*  */ "json" => '\u{e60b}',
        /*  */ "jsx" => '\u{e7ba}',
        /*  */ "jl" => '\u{e624}',
        /*  */ "ksh" => '\u{f489}',
        /*  */ "less" => '\u{e758}',
        /*  */ "lhs" => '\u{e777}',
        /*  */ "license" => '\u{f48a}',
        /*  */ "localized" => '\u{f179}',
        /*  */ "lock" => '\u{f023}',
        /*  */ "log" => '\u{f18d}',
        /*  */ "lua" => '\u{e620}',
        /*  */ "lz" => '\u{f410}',
        /* 蘿*/ "m3u" => '\u{f910}',
        /* 蘿*/ "m3u8" => '\u{f910}',
        /*  */ "m4a" => '\u{f001}',
        /*  */ "markdown" => '\u{f48a}',
        /*  */ "md" => '\u{f48a}',
        /*  */ "mkd" => '\u{f48a}',
        /*  */ "mkv" => '\u{f03d}',
        /*  */ "mobi" => '\u{e28b}',
        /*  */ "mov" => '\u{f03d}',
        /*  */ "mp3" => '\u{f001}',
        /*  */ "mp4" => '\u{f03d}',
        /*  */ "mustache" => '\u{e60f}',
        /*  */ "nix" => '\u{f313}',
        /*  */ "npmignore" => '\u{e71e}',
        /*  */ "opus" => '\u{f001}',
        /*  */ "ogg" => '\u{f001}',
        /*  */ "ogv" => '\u{f03d}',
        /*  */ "otf" => '\u{f031}',
        /*  */ "pcap" => '\u{f471}',
        /*  */ "pdf" => '\u{f1c1}',
        /*  */ "php" => '\u{e73d}',
        /*  */ "pl" => '\u{e769}',
        /* 蘿 */ "pls" => '\u{f910}',
        /*  */ "png" => '\u{f1c5}',
        /*  */ "ppt" => '\u{f1c4}',
        /*  */ "pptx" => '\u{f1c4}',
        /*  */ "procfile" => '\u{e21e}',
        /*  */ "properties" => '\u{e60b}',
        /*  */ "ps1" => '\u{f489}',
        /*  */ "psd" => '\u{e7b8}',
        /*  */ "pxm" => '\u{f1c5}',
        /*  */ "py" => '\u{e606}',
        /*  */ "pyc" => '\u{e606}',
        /*  */ "r" => '\u{f25d}',
        /*  */ "rakefile" => '\u{e21e}',
        /*  */ "rar" => '\u{f410}',
        /*  */ "razor" => '\u{f1fa}',
        /*  */ "rb" => '\u{e21e}',
        /*  */ "rdata" => '\u{f25d}',
        /*  */ "rdb" => '\u{e76d}',
        /*  */ "rdoc" => '\u{f48a}',
        /*  */ "rds" => '\u{f25d}',
        /*  */ "readme" => '\u{f48a}',
        /*  */ "rlib" => '\u{e7a8}',
        /*  */ "rmd" => '\u{f48a}',
        /*  */ "rs" => '\u{e7a8}',
        /*  */ "rspec" => '\u{e21e}',
        /*  */ "rspec_parallel" => '\u{e21e}',
        /*  */ "rspec_status" => '\u{e21e}',
        /*  */ "rss" => '\u{f09e}',
        /*  */ "ru" => '\u{e21e}',
        /*  */ "rubydoc" => '\u{e73b}',
        /*  */ "sass" => '\u{e603}',
        /*  */ "scala" => '\u{e737}',
        /*  */ "scss" => '\u{e749}',
        /*  */ "sh" => '\u{f489}',
        /*  */ "shell" => '\u{f489}',
        /*  */ "slim" => '\u{e73b}',
        /*  */ "sln" => '\u{e70c}',
        /*  */ "sql" => '\u{f1c0}',
        /*  */ "sqlite3" => '\u{e7c4}',
        /*  */ "styl" => '\u{e600}',
        /*  */ "stylus" => '\u{e600}',
        /*  */ "svg" => '\u{f1c5}',
        /*  */ "swift" => '\u{e755}',
        /*  */ "tar" => '\u{f410}',
        /*  */ "tex" => '\u{e600}',
        /*  */ "tiff" => '\u{f1c5}',
        /*  */ "ts" => '\u{e628}',
        /*  */ "tsx" => '\u{e7ba}',
        /*  */ "ttc" => '\u{f031}',
        /*  */ "ttf" => '\u{f031}',
        /*  */ "twig" => '\u{e61c}',
        /*  */ "txt" => '\u{f15c}',
        /*  */ "video" => '\u{f03d}',
        /*  */ "vim" => '\u{e62b}',
        /* 蘿 */ "vlc" => '\u{f910}',
        /* ﵂ */ "vue" => '\u{fd42}',
        /*  */ "wav" => '\u{f001}',
        /*  */ "webm" => '\u{f03d}',
        /*  */ "webp" => '\u{f1c5}',
        /*  */ "windows" => '\u{f17a}',
        /*  */ "wma" => '\u{f001}',
        /*  */ "wmv" => '\u{f03d}',
        /* 蘿 */ "wpl" => '\u{f910}',
        /*  */ "woff" => '\u{f031}',
        /*  */ "woff2" => '\u{f031}',
        /*  */ "xls" => '\u{f1c3}',
        /*  */ "xlsx" => '\u{f1c3}',
        /*  */ "xml" => '\u{e619}',
        /*  */ "xul" => '\u{e619}',
        /*  */ "xz" => '\u{f410}',
        /*  */ "yaml" => '\u{e60b}',
        /*  */ "yml" => '\u{e60b}',
        /*  */ "zip" => '\u{f410}',
        /*  */ "zsh" => '\u{f489}',
        /*  */ "zsh-theme" => '\u{f489}',
        /*  */ "zshrc" => '\u{f489}',
    }
}

#[cfg(test)]
mod test {
    use super::{Icons, Theme};
    use crate::meta::Meta;
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::NoIcon).get(&meta.name);
        assert_eq!(icon, None);
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Fancy).get(&meta.name);
        assert_eq!(icon, Some('\u{f016}')); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Unicode).get(&meta.name);
        assert_eq!(icon, Some('\u{1f5cb}'));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy).get(&meta.name);
        assert_eq!(icon, Some('\u{f115}')); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Unicode).get(&meta.name);
        assert_eq!(icon, Some('\u{1f5c1}'));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy).get(&meta.name);

        assert_eq!(icon, Some('\u{f115}')); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (file_name, file_icon) in super::default_icons_by_name() {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(Theme::Fancy).get(&meta.name);
            assert_eq!(icon, Some(file_icon));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (ext, file_icon) in super::default_icons_by_extension() {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(Theme::Fancy).get(&meta.name);
            assert_eq!(icon, Some(file_icon));
        }
    }

    #[test]
    fn test_shebangs() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");

        for (expected, shebang) in vec![
            ('\u{e606}', "env python2.7"), // test #!/env cmd
            ('\u{e606}', "python3"),       // test #!/cmd
            ('\u{e606}', "python -vv"),    // test #!/cmd args
            ('\u{e620}', "lua"),           // test shebang is extension
        ] {
            let mut f = File::create(&file_path).expect("failed to create file");
            f.write(format!("#!/path/to/{}\n", shebang).as_ref())
                .unwrap();
            f.flush().unwrap();

            let meta = Meta::from_path(&file_path, false).unwrap();
            let icon = Icons::new(Theme::Fancy).get(&meta.name);

            assert_eq!(icon, Some(expected));
        }
    }
}
