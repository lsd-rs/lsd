use meta::FileType;
use meta::Meta;
use std::cmp::Ordering;

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

    pub fn get_short_output(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        for meta in &self.0 {
            res.push(meta.name.render());
        }

        res
    }

    pub fn get_long_output(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        let max_user_length = self.detect_user_lenght();
        let max_group_length = self.detect_group_lenght();
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts();

        for meta in &self.0 {
            let mut link_str = String::new();
            if let Some(ref symlink) = meta.symlink {
                link_str = symlink.render();
            }

            res.push(format!(
                "{}{}  {}  {}  {}  {}{}",
                meta.file_type.render(),
                meta.permissions.render(),
                meta.owner.render(max_user_length, max_group_length),
                meta.size
                    .render(max_size_value_length, max_size_unit_length),
                meta.date.render(),
                meta.name.render(),
                link_str,
            ));
        }

        res
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
