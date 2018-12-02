use batch::Batch;
use meta::Meta;
use std::path::Path;
use Options;

pub struct Core<'a> {
    options: &'a Options,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core { options }
    }

    pub fn run(&self, inputs: Vec<&str>) {
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for input in inputs {
            let path = Path::new(input);

            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(Meta::from(path));
            } else {
                let err = path.metadata().unwrap_err();
                println!("cannot access '{}': {}", path.display(), err);
            }
        }

        let print_folder_name: bool = dirs.len() + files.len() > 1;

        let mut file_batch = Batch::from(files);
        file_batch.sort();
        self.print(&file_batch);

        dirs.sort_unstable();

        for dir in dirs {
            if let Some(folder_batch) = self.list_folder_content(dir) {
                if print_folder_name {
                    println!("\n{}:", dir.display())
                }

                self.print(&folder_batch);
            }
        }
    }

    pub fn print<'b>(&self, batch: &'b Batch) {
        if self.options.display_long {
            batch.print_long();
        } else {
            batch.print_short();
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
                let meta = Meta::from(entry.path().as_path());
                if !meta.name.is_hidden() || self.options.display_all {
                    metas.push(meta);
                }
            }
        }

        let mut batch = Batch::from(metas);
        batch.sort();

        Some(batch)
    }
}
