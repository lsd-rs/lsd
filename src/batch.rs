use ansi_term::{ANSIString, ANSIStrings};
use color::Colors;
use flags::{DirOrderFlag, Flags, SortFlag, SortOrder};
use icon::Icons;
use meta::{FileType, Meta};
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

    pub fn sort(&mut self, flags: Flags) {
        self.0.sort_unstable_by(|a, b| sort_by_meta(a, b, flags));
    }

    pub fn get_short_output(&self, colors: &Colors, icons: &Icons, flags: Flags) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        for meta in &self.0 {
            let strings: &[ANSIString] = &[
                meta.name.render(colors, icons),
                meta.indicator.render(flags),
            ];

            res.push(ANSIStrings(strings).to_string());
        }

        res
    }

    pub fn get_long_output(&self, colors: &Colors, icons: &Icons, flags: Flags) -> Vec<String> {
        let mut res = Vec::with_capacity(self.0.len());

        let max_user_length = self.detect_user_lenght();
        let max_group_length = self.detect_group_lenght();
        let (max_size_value_length, max_size_unit_length) = self.detect_size_lenghts();
        let max_date_length = self.detect_date_lenght(flags);

        for meta in &self.0 {
            let strings: &[ANSIString] = &[
                meta.file_type.render(colors),
                meta.permissions.render(colors),
                ANSIString::from("  "),
                meta.owner.render_user(colors, max_user_length),
                ANSIString::from("  "),
                meta.owner.render_group(colors, max_group_length),
                ANSIString::from("  "),
                meta.size
                    .render(colors, max_size_value_length, max_size_unit_length),
                ANSIString::from("  "),
                meta.date.render(colors, max_date_length, flags),
                ANSIString::from("  "),
                meta.name.render(colors, icons),
                meta.indicator.render(flags),
                meta.symlink.render(colors),
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

    fn detect_date_lenght(&self, flags: Flags) -> usize {
        let mut max_value_length: usize = 0;

        for meta in &self.0 {
            if meta.date.date_string(flags).len() > max_value_length {
                max_value_length = meta.date.date_string(flags).len();
            }
        }

        max_value_length
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

fn sort_by_meta(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match flags.sort_by {
        SortFlag::Name => match flags.directory_order {
            DirOrderFlag::First => sort_by_name_with_dirs_first(a, b, flags),
            DirOrderFlag::None => sort_by_name_with_dirs_unordored(a, b, flags),
            DirOrderFlag::Last => sort_by_name_with_files_first(a, b, flags),
        },

        SortFlag::Time => match flags.directory_order {
            DirOrderFlag::First => sort_by_date_with_dirs_first(a, b, flags),
            DirOrderFlag::None => sort_by_date_with_dirs_unordored(a, b, flags),
            DirOrderFlag::Last => sort_by_date_with_files_first(a, b, flags),
        },
    }
}

fn sort_by_name_with_dirs_unordored(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        a.name.cmp(&b.name)
    } else {
        b.name.cmp(&a.name)
    }
}

fn sort_by_name_with_dirs_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if a.file_type == FileType::Directory && b.file_type != FileType::Directory {
        Ordering::Less
    } else if b.file_type == FileType::Directory && a.file_type != FileType::Directory {
        Ordering::Greater
    } else {
        if flags.sort_order == SortOrder::Default {
            a.name.cmp(&b.name)
        } else {
            b.name.cmp(&a.name)
        }
    }
}

fn sort_by_name_with_files_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if a.file_type == FileType::Directory && b.file_type != FileType::Directory {
        Ordering::Greater
    } else if b.file_type == FileType::Directory && a.file_type != FileType::Directory {
        Ordering::Less
    } else {
        if flags.sort_order == SortOrder::Default {
            a.name.cmp(&b.name)
        } else {
            b.name.cmp(&a.name)
        }
    }
}

fn sort_by_date_with_dirs_unordored(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        b.date.cmp(&a.date).then(a.name.cmp(&b.name))
    } else {
        a.date.cmp(&b.date).then(b.name.cmp(&a.name))
    }
}

fn sort_by_date_with_dirs_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if a.file_type == FileType::Directory && b.file_type != FileType::Directory {
        Ordering::Less
    } else if b.file_type == FileType::Directory && a.file_type != FileType::Directory {
        Ordering::Greater
    } else {
        if flags.sort_order == SortOrder::Default {
            b.date.cmp(&a.date).then(a.name.cmp(&b.name))
        } else {
            a.date.cmp(&b.date).then(b.name.cmp(&a.name))
        }
    }
}

fn sort_by_date_with_files_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if a.file_type == FileType::Directory && b.file_type != FileType::Directory {
        Ordering::Greater
    } else if b.file_type == FileType::Directory && a.file_type != FileType::Directory {
        Ordering::Less
    } else {
        if flags.sort_order == SortOrder::Default {
            b.date.cmp(&a.date).then(a.name.cmp(&b.name))
        } else {
            a.date.cmp(&b.date).then(b.name.cmp(&a.name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flags::Flags;
    use std::fs::{create_dir, File};
    use std::process::Command;
    use tempdir::TempDir;

    #[test]
    fn test_sort_by_meta_by_name_with_dirs_first() {
        let tmp_dir = TempDir::new("test_dir").expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.directory_order = DirOrderFlag::First;

        //  Sort with the dirs first
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Greater);

        //  Sort with the dirs first (the dirs stay first)
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
    }

    #[test]
    fn test_sort_by_meta_by_name_with_files_first() {
        let tmp_dir = TempDir::new("test_dir").expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.directory_order = DirOrderFlag::Last;

        // Sort with file first
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort with file first reversed (thie files stay first)
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Less);
    }

    #[test]
    fn test_sort_by_meta_by_name_unordered() {
        let tmp_dir = TempDir::new("test_dir").expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("zzz");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.directory_order = DirOrderFlag::None;

        // Sort by name unordered
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort by name unordered
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
    }

    #[test]
    fn test_sort_by_meta_by_name_unordered_2() {
        let tmp_dir = TempDir::new("test_dir").expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.directory_order = DirOrderFlag::None;

        // Sort by name unordered
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Greater);

        // Sort by name unordered reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Less);
    }

    #[test]
    fn test_sort_by_meta_by_time() {
        let tmp_dir = TempDir::new("test_dir").expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a).expect("failed to get meta");

        // Create the file;
        let path_z = tmp_dir.path().join("zzz");
        File::create(&path_z).expect("failed to create file");
        let success = Command::new("touch")
            .arg("-t")
            .arg("198511160000")
            .arg(&path_z)
            .status()
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec mkfifo");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sort_by = SortFlag::Time;

        // Sort by time
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort by time reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(sort_by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
    }
}
