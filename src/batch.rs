use ansi_term::{ANSIString, ANSIStrings};
use meta::FileType;
use meta::Meta;
use std::cmp::Ordering;
use std::iter::IntoIterator;
use std::vec::IntoIter;

pub struct Batch(Vec<Meta>);

impl From<Vec<Meta>> for Batch {
    fn from(metas: Vec<Meta>) -> Self {
        Batch(metas)
    }
}

impl IntoIterator for Batch {
    type Item = Meta;
    type IntoIter = IntoIter<Meta>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Batch {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn sort(&mut self) {
        self.0.sort_unstable_by(sort_by_meta);
    }

    pub fn get_short_output(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        for meta in &self.0 {
            res.push(meta.name.render().to_string());
        }

        res
    }

    pub fn get_long_output(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        let max_user_length = self.detect_user_lenght();
        let max_group_length = self.detect_group_lenght();
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts();

        for meta in &self.0 {
            let mut link_str = ANSIString::from("");
            if let Some(ref symlink) = meta.symlink {
                link_str = symlink.render();
            }

            let strings: &[ANSIString] = &[
                meta.file_type.render(),
                meta.permissions.render(),
                ANSIString::from("  "),
                meta.owner.render_user(max_user_length),
                ANSIString::from("  "),
                meta.owner.render_group(max_group_length),
                ANSIString::from("  "),
                meta.size
                    .render(max_size_value_length, max_size_unit_length),
                ANSIString::from("  "),
                meta.date.render(),
                ANSIString::from("  "),
                meta.name.render(),
                link_str,
            ];

            res.push(ANSIStrings(strings).to_string());
        }

        res
    }

    fn detect_user_lenght(&self) -> usize {
        let mut max: usize = 0;

        for meta in &self.0 {
            let user = meta.owner.user();
            if user.len() > max {
                max = user.len();
            }
        }

        max
    }

    fn detect_group_lenght(&self) -> usize {
        let mut max: usize = 0;

        for meta in &self.0 {
            let group = meta.owner.group();
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
