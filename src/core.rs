use formatter::*;
use size::*;
use std::cmp::Ordering;
use std::fs::{read_link, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};
use Options;

pub struct Core<'a> {
    formatter: Formatter,
    options: &'a Options,
}

#[derive(Debug)]
pub struct Meta {
    pub path: PathBuf,
    pub name: String,
    pub metadata: Metadata,
    pub group: String,
    pub user: String,
    pub symlink: Option<String>,
    pub size_value: String,
    pub size_unit: String,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core {
            options,
            formatter: Formatter::new(),
        }
    }

    pub fn print(&self, inputs: Vec<&str>) {
        let print_folder_name: bool = inputs.len() > 1;

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for input in inputs {
            let path = Path::new(input);

            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(
                    self.path_to_meta(&path)
                        .expect("failed to convert path to meta"),
                );
            } else {
                match path.metadata() {
                    Ok(_) => panic!("shouldn't failed"),
                    Err(err) => println!("cannot access '{}': {}", path.display(), err),
                };
            }
        }

        files.sort_unstable_by(sort_by_meta);
        dirs.sort_unstable();

        if !files.is_empty() {
            self.print_long(&files);
        }

        for dir in dirs {
            let folder_metas = self.list_folder_content(dir);
            if folder_metas.is_empty() {
                continue;
            }

            if print_folder_name {
                println!("\n{}:", dir.display())
            }
            self.print_long(&folder_metas);
        }
    }

    pub fn list_folder_content(&self, folder: &Path) -> Vec<Meta> {
        let mut content: Vec<Meta> = Vec::new();

        let dir = match folder.read_dir() {
            Ok(dir) => dir,
            Err(err) => {
                println!("cannot open directory'{}': {}", folder.display(), err);
                return content;
            }
        };

        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(meta) = self.path_to_meta(entry.path().as_path()) {
                    content.push(meta);
                }
            }
        }

        content.sort_unstable_by(sort_by_meta);

        content
    }

    pub fn path_to_meta(&self, path: &Path) -> Option<Meta> {
        let mut name: Option<&str> = None;
        if let Some(os_str_name) = path.file_name() {
            if let Some(name_str) = os_str_name.to_str() {
                name = Some(name_str);
            }
        }

        if name.is_none() {
            println!("failed to retrieve file name for {}", path.display());
            return None;
        }

        // Skip the hidden files if the 'display_all' option is not set.
        if name.unwrap().starts_with('.') && !self.options.display_all {
            return None;
        }

        let meta;
        let mut symlink = None;
        if let Ok(res) = read_link(path) {
            meta = path
                .symlink_metadata()
                .expect("failed to retrieve symlink metadata");
            symlink = Some(
                res.to_str()
                    .expect("failed to convert symlink to str")
                    .to_string(),
            );
        } else {
            meta = match path.metadata() {
                Ok(meta) => meta,
                Err(err) => {
                    println!("err: {}", err);
                    return None;
                }
            }
        }

        let user = get_user_by_uid(meta.uid())
            .expect("failed to get user name")
            .name()
            .to_str()
            .expect("failed to convert user name to str")
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("failed to get the group name")
            .name()
            .to_str()
            .expect("failed to convert group name to str")
            .to_string();

        let size = Size::Bytes(meta.len()).to_string(Base::Base10, Style::Abbreviated);
        let size_parts: Vec<&str> = size.split(' ').collect();

        Some(Meta {
            path: path.to_path_buf(),
            metadata: meta,
            name: String::from(name.unwrap()),
            user,
            group,
            symlink,
            size_value: size_parts[0].to_string(),
            size_unit: size_parts[1].to_string(),
        })
    }

    fn print_long(&self, metas: &[Meta]) {
        let max_user_length = self.detect_user_lenght(&metas);
        let max_group_length = self.detect_group_lenght(&metas);
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts(&metas);

        for meta in metas {
            println!(
                "  {}  {}  {}  {}  {}  {}",
                self.formatter.format_permissions(&meta),
                self.formatter.format_user(&meta.user, max_user_length),
                self.formatter.format_group(&meta.group, max_group_length),
                self.formatter
                    .format_size(&meta, max_size_value_length, max_size_unit_length),
                self.formatter.format_date(&meta),
                self.formatter.format_name(&meta),
            );
        }
    }

    fn detect_user_lenght(&self, paths: &[Meta]) -> usize {
        let mut max: usize = 0;

        for path in paths {
            if path.user.len() > max {
                max = path.user.len();
            }
        }

        max
    }

    fn detect_group_lenght(&self, paths: &[Meta]) -> usize {
        let mut max: usize = 0;

        for path in paths {
            if path.group.len() > max {
                max = path.group.len();
            }
        }

        max
    }

    fn detect_size_lenghts(&self, paths: &[Meta]) -> (usize, usize) {
        let mut max_value_length: usize = 0;
        let mut max_unit_size: usize = 0;

        for path in paths {
            if path.size_value.len() > max_value_length {
                max_value_length = path.size_value.len();
            }

            if path.size_unit.len() > max_unit_size {
                max_unit_size = path.size_unit.len();
            }
        }

        (max_value_length, max_unit_size)
    }
}

fn sort_by_meta(a: &Meta, b: &Meta) -> Ordering {
    if a.path.is_dir() == b.path.is_dir() {
        a.path.cmp(&b.path)
    } else if a.path.is_dir() && b.path.is_file() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
