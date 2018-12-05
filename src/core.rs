use batch::Batch;
use color::Colors;
use display::Display;
use meta::{FileType, Meta};
use std::path::{Path, PathBuf};
use Options;

pub struct Core<'a> {
    options: &'a Options,
    display: Display<'a>,
    colors: Colors,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core {
            options,
            display: Display::new(options),
            colors: Colors::new(),
        }
    }

    pub fn run(self, paths: Vec<PathBuf>) {
        self.run_inner(paths, 0);
    }

    fn run_inner(&self, paths: Vec<PathBuf>, depth: usize) {
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

        if !files.is_empty() && !self.options.display_tree {
            let mut file_batch = Batch::from(files);
            file_batch.sort();
            self.display
                .print_outputs(self.get_batch_outputs(&file_batch));
        }

        dirs.sort_unstable();

        for dir in dirs {
            if let Some(folder_batch) = self.list_folder_content(dir.as_path()) {
                if (print_folder_name || self.options.recursive) && !self.options.display_tree {
                    println!("\n{}:", dir.display())
                }

                if self.options.display_tree {
                    self.display_as_tree(folder_batch, depth);
                } else if self.options.recursive {
                    self.display
                        .print_outputs(self.get_batch_outputs(&folder_batch));

                    let folder_dirs = folder_batch
                        .into_iter()
                        .filter_map(|x| {
                            if x.file_type == FileType::Directory {
                                Some(x.path)
                            } else {
                                None
                            }
                        }).collect();

                    self.run_inner(folder_dirs, depth);
                } else {
                    self.display
                        .print_outputs(self.get_batch_outputs(&folder_batch));
                }
            }
        }
    }

    pub fn display_as_tree(&self, batch: Batch, depth: usize) {
        let last_idx = batch.len();

        for (idx, elem) in batch.into_iter().enumerate() {
            let last = idx + 1 != last_idx;

            if elem.file_type == FileType::Directory {
                self.display.print_tree_row(
                    elem.name.render(&self.colors).to_string(),
                    depth,
                    last,
                );
                self.run_inner(vec![elem.path], depth + 1);
            } else {
                self.display.print_tree_row(
                    elem.name.render(&self.colors).to_string(),
                    depth,
                    last,
                );
            }
        }
    }

    pub fn get_batch_outputs<'b>(&self, batch: &'b Batch) -> Vec<String> {
        if self.options.display_long {
            batch.get_long_output(&self.colors)
        } else {
            batch.get_short_output(&self.colors)
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
