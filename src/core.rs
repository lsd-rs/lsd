use formatter::*;
use meta::Meta;
use std::cmp::Ordering;
use std::path::Path;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use Options;

pub struct Core<'a> {
    formatter: Formatter,
    options: &'a Options,
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Core<'a> {
        Core {
            options,
            formatter: Formatter::new(),
        }
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
                match Meta::from_path(path) {
                    Ok(meta) => files.push(meta),
                    Err(err) => println!("err : {}", err),
                };
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
            content = content + &self.formatter.format_name(&meta);
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
            println!(
                "{}  {}  {}  {}  {}  {}{}",
                self.formatter.format_permissions(&meta),
                self.formatter.format_user(&meta.user, max_user_length),
                self.formatter.format_group(&meta.group, max_group_length),
                self.formatter
                    .format_size(&meta, max_size_value_length, max_size_unit_length),
                self.formatter.format_date(&meta),
                self.formatter.format_name(&meta),
                self.formatter.format_symlink(&meta),
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
                match Meta::from_path(entry.path().as_path()) {
                    Ok(meta) => {
                        if !meta.name.starts_with('.') || self.options.display_all {
                            content.push(meta);
                        }
                    }
                    Err(err) => println!("err 2: {}", err),
                }
            }
        }

        content.sort_unstable_by(sort_by_meta);

        content
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
