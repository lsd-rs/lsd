use meta::FileType;
use meta::Meta;
use std::cmp::Ordering;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;

pub struct Batch(Vec<Meta>);

impl From<Vec<Meta>> for Batch {
    fn from(metas: Vec<Meta>) -> Self {
        Batch(metas)
    }
}

impl Batch {
    pub fn sort(&mut self) {
        self.0.sort_unstable_by(sort_by_meta);
    }

    pub fn print_short(&self) {
        let term_width = match terminal_size() {
            Some((w, _)) => w.0 as usize,
            None => panic!("failed to retrieve terminal size"),
        };

        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        });

        for meta in &self.0 {
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

    pub fn print_long(&self) {
        let max_user_length = self.detect_user_lenght();
        let max_group_length = self.detect_group_lenght();
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts();

        for meta in &self.0 {
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

    fn detect_user_lenght(&self) -> usize {
        let mut max: usize = 0;

        for meta in &self.0 {
            let user = meta.owner.render_user();
            if user.len() > max {
                max = user.len();
            }
        }

        max
    }

    fn detect_group_lenght(&self) -> usize {
        let mut max: usize = 0;

        for meta in &self.0 {
            let group = meta.owner.render_group();
            if group.len() > max {
                max = group.len();
            }
        }

        max
    }

    fn detect_size_lenghts(&self) -> (usize, usize) {
        let mut max_value_length: usize = 0;
        let mut max_unit_size: usize = 0;

        for meta in &self.0 {
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
