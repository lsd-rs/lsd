use batch::Batch;
use display::Display;
use meta::{FileType, Meta};
use std::path::{Path, PathBuf};
use Options;

pub struct Core<'a> {
    options: &'a Options,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core { options }
    }

    pub fn run(&self, paths: Vec<PathBuf>) {
        let display = Display::new(self.options);

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for path in paths {
            if path.is_dir() {
                dirs.push(path);
            } else if let Some(meta) = Meta::from_path(&path) {
                files.push(meta);
            }
        }

        let print_folder_name: bool = dirs.len() + files.len() > 1;

        if !files.is_empty() {
            let mut file_batch = Batch::from(files);
            file_batch.sort();
            display.print_outputs(self.get_batch_outputs(&file_batch));
        }

        dirs.sort_unstable();

        for dir in dirs {
            if let Some(folder_batch) = self.list_folder_content(dir.as_path()) {
                if print_folder_name || self.options.recursive {
                    println!("\n{}:", dir.display())
                }

                display.print_outputs(self.get_batch_outputs(&folder_batch));

                if self.options.recursive {
                    let folder_dirs = folder_batch
                        .into_iter()
                        .filter_map(|x| {
                            if x.file_type == FileType::Directory {
                                Some(x.path)
                            } else {
                                None
                            }
                        }).collect();

                    self.run(folder_dirs);
                }
            }
        }
    }

    pub fn get_batch_outputs<'b>(&self, batch: &'b Batch) -> Vec<String> {
        if self.options.display_long {
            batch.get_long_output()
        } else {
            batch.get_short_output()
        }
    }

    pub fn list_folder_content(&self, folder: &Path) -> Option<Batch> {
        let mut metas: Vec<Meta> = Vec::new();

        let dir = match folder.read_dir() {
            Ok(dir) => dir,
            Err(err) => {
                println!("cannot open directory'{}': {}", folder.display(), err);
                return None;
            }
        };

        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(meta) = Meta::from_path(&entry.path()) {
                    if !meta.name.is_hidden() || self.options.display_all {
                        metas.push(meta);
                    }
                }
            }
        }

        let mut batch = Batch::from(metas);
        batch.sort();

        Some(batch)
    }
}
