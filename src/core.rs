use meta::{FileType, Meta};
use std::cmp::Ordering;
use std::path::Path;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use Options;

pub struct Core<'a> {
    options: &'a Options,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core { options }
    }

    pub fn run(&self, inputs: Vec<&str>) {
        let print_folder_name: bool = inputs.len() > 1;

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for input in inputs {
            let path = Path::new(input);

            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(Meta::from(path));
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
            self.print(&files);
        }

        for dir in dirs {
            let folder_metas = self.list_folder_content(dir);
            if folder_metas.is_empty() {
                continue;
            }

            if print_folder_name {
                println!("\n{}:", dir.display())
            }
            self.print(&folder_metas);
        }
    }

    fn print(&self, metas: &[Meta]) {
        if self.options.display_long {
            self.print_long(metas)
        } else {
            self.print_short(metas)
        }
    }

    fn print_short(&self, metas: &[Meta]) {
        let term_width = match terminal_size() {
            Some((w, _)) => w.0 as usize,
            None => panic!("failed to retrieve terminal size"),
        };

        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        });

        for meta in metas {
            let mut content = String::from("    ");
            content += &meta.name.render();
            grid.add(Cell {
                width: content.len(),
                contents: content,
            });
        }

        println!(
            "{}",
            grid.fit_into_width(term_width * 2)
                .expect("failed to print the grid")
        );
    }

    fn print_long(&self, metas: &[Meta]) {
        let max_user_length = self.detect_user_lenght(&metas);
        let max_group_length = self.detect_group_lenght(&metas);
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts(&metas);

        for meta in metas {
            let mut link_str = String::new();
            if let Some(ref symlink) = meta.symlink {
                link_str = symlink.render();
            }

            println!(
                "{}{}  {}  {}  {}  {}{}",
                meta.file_type.render(),
                meta.permissions.render(),
                meta.owner.render(max_user_length, max_group_length),
                meta.size
                    .render(max_size_value_length, max_size_unit_length),
                meta.date.render(),
                meta.name.render(),
                link_str,
            );
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
                let meta = Meta::from(entry.path().as_path());
                if !meta.name.is_hidden() || self.options.display_all {
                    content.push(meta);
                }
            }
        }

        content.sort_unstable_by(sort_by_meta);

        content
    }

    fn detect_user_lenght(&self, metas: &[Meta]) -> usize {
        let mut max: usize = 0;

        for meta in metas {
            let user = meta.owner.render_user();
            if user.len() > max {
                max = user.len();
            }
        }

        max
    }

    fn detect_group_lenght(&self, metas: &[Meta]) -> usize {
        let mut max: usize = 0;

        for meta in metas {
            let group = meta.owner.render_group();
            if group.len() > max {
                max = group.len();
            }
        }

        max
    }

    fn detect_size_lenghts(&self, metas: &[Meta]) -> (usize, usize) {
        let mut max_value_length: usize = 0;
        let mut max_unit_size: usize = 0;

        for meta in metas {
            if meta.size.render_value().len() > max_value_length {
                max_value_length = meta.size.render_value().len();
            }

            if meta.size.render_unit().len() > max_unit_size {
                max_unit_size = meta.size.render_unit().len();
            }
        }

        (max_value_length, max_unit_size)
    }
}

fn sort_by_meta(a: &Meta, b: &Meta) -> Ordering {
    if a.file_type == FileType::Directory && b.file_type != FileType::Directory {
        Ordering::Less
    } else if b.file_type == FileType::Directory && a.file_type != FileType::Directory {
        Ordering::Greater
    } else {
        a.name.cmp(&b.name)
    }
}
