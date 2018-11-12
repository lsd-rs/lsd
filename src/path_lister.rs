use std::fs::Metadata;
use std::io::ErrorKind;
use std::path::{self, PathBuf};
use Options;

pub struct Path {
    pub path: PathBuf,
    pub metadata: Metadata,
}

pub struct PathLister<'a> {
    options: &'a Options,
}

impl<'a> PathLister<'a> {
    pub fn new(options: &'a Options) -> PathLister<'a> {
        PathLister { options: options }
    }

    pub fn list_paths_to_print(&self, inputs: Vec<&'a str>) -> Vec<Path> {
        let mut res = Vec::new();

        for input in inputs.iter() {
            let path = path::Path::new(input);

            let res_meta = path.metadata();

            if let Err(err) = res_meta {
                match err.kind() {
                    ErrorKind::NotFound => println!("Specified path \"{}\" doesn't exists.", input),
                    ErrorKind::PermissionDenied => {
                        println!("Cannot open \"{}\": Permission denied", input)
                    }
                    _ => println!("Cannot open \"{}\": {}", input, err),
                }

                continue;
            }

            if let Ok(meta) = res_meta {
                let file_type = meta.file_type();

                if file_type.is_file() {
                    self.add_path_to_list(&mut res, path.to_path_buf(), meta);
                    continue;
                }

                if file_type.is_dir() {
                    self.read_paths_from_dir(&mut res, path.to_path_buf());
                    continue;
                }

                if file_type.is_symlink() {}
            }
        }

        res
    }

    fn read_paths_from_dir(&self, path_list: &mut Vec<Path>, path: PathBuf) {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                self.add_path_to_list(path_list, entry.path(), entry.metadata().unwrap());
            }
        }
    }

    fn add_path_to_list(&self, path_list: &mut Vec<Path>, path: PathBuf, meta: Metadata) {
        // Skip the hidden files if the 'display_all' option is not set.
        if path.file_name().unwrap().to_str().unwrap().starts_with(".") && !self.options.display_all
        {
            return;
        }

        path_list.push(Path {
            path: path.to_path_buf(),
            metadata: meta,
        });
    }
}
