use crate::flags::{DirGrouping, Flags, SortColumn, SortOrder};
use crate::meta::Meta;
use std::cmp::Ordering;
use vsort::compare;

pub type SortFn = fn(&Meta, &Meta) -> Ordering;

pub fn assemble_sorters(flags: &Flags) -> Vec<(SortOrder, SortFn)> {
    let mut sorters: Vec<(SortOrder, SortFn)> = vec![];
    match flags.sorting.dir_grouping {
        DirGrouping::First => {
            sorters.push((SortOrder::Default, with_dirs_first));
        }
        DirGrouping::Last => {
            sorters.push((SortOrder::Reverse, with_dirs_first));
        }
        DirGrouping::None => {}
    };

    match flags.sorting.column {
        SortColumn::Name => sorters.push((flags.sorting.order, by_name)),
        SortColumn::Size => sorters.push((flags.sorting.order, by_size)),
        SortColumn::Time => sorters.push((flags.sorting.order, by_date)),
        SortColumn::Version => sorters.push((flags.sorting.order, by_version)),
        SortColumn::Extension => sorters.push((flags.sorting.order, by_extension)),
        SortColumn::GitStatus => sorters.push((flags.sorting.order, by_git_status)),
        SortColumn::None => {}
    }
    sorters
}

pub fn by_meta(sorters: &[(SortOrder, SortFn)], a: &Meta, b: &Meta) -> Ordering {
    for (direction, sorter) in sorters.iter() {
        match (sorter)(a, b) {
            Ordering::Equal => continue,
            ordering => {
                return match direction {
                    SortOrder::Reverse => ordering.reverse(),
                    SortOrder::Default => ordering,
                }
            }
        }
    }
    Ordering::Equal
}

fn with_dirs_first(a: &Meta, b: &Meta) -> Ordering {
    b.file_type.is_dirlike().cmp(&a.file_type.is_dirlike())
}

fn by_size(a: &Meta, b: &Meta) -> Ordering {
    match (&a.size, &b.size) {
        (Some(a_size), Some(b_size)) => b_size.get_bytes().cmp(&a_size.get_bytes()),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

fn by_name(a: &Meta, b: &Meta) -> Ordering {
    a.name.cmp(&b.name)
}

fn by_date(a: &Meta, b: &Meta) -> Ordering {
    b.date.cmp(&a.date).then(a.name.cmp(&b.name))
}

fn by_version(a: &Meta, b: &Meta) -> Ordering {
    compare(&a.name.name, &b.name.name)
}

fn by_extension(a: &Meta, b: &Meta) -> Ordering {
    a.name.extension().cmp(&b.name.extension())
}

fn by_git_status(a: &Meta, b: &Meta) -> Ordering {
    a.git_status.cmp(&b.git_status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::{Flags, PermissionFlag};
    use std::fs::{create_dir, File};
    use std::io::prelude::*;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn test_sort_assemble_sorters_by_name_with_dirs_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::First;

        //  Sort with the dirs first
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);

        //  Sort with the dirs first (the dirs stay first)
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_with_files_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::Last;

        // Sort with file first
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort with file first reversed (this files stay first)
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_unordered() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("zzz");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;

        // Sort by name unordered
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort by name unordered
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_unordered_2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;

        // Sort by name unordered
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);

        // Sort by name unordered reversed
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_time() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create the file;
        let path_z = tmp_dir.path().join("zzz");
        File::create(&path_z).expect("failed to create file");

        #[cfg(unix)]
        let success = Command::new("touch")
            .arg("-t")
            .arg("198511160000")
            .arg(&path_z)
            .status()
            .unwrap()
            .success();

        #[cfg(windows)]
        let success = Command::new("powershell")
            .arg("-Command")
            .arg("$(Get-Item")
            .arg(&path_z)
            .arg(").lastwritetime=$(Get-Date \"1985-11-16\")")
            .status()
            .unwrap()
            .success();

        assert!(success, "failed to change file timestamp");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Time;

        // Sort by time
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort by time reversed
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file with rs extension;
        let path_a = tmp_dir.path().join("aaa.rs");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create the file with rs extension;
        let path_z = tmp_dir.path().join("zzz.rs");
        File::create(&path_z).expect("failed to create file");
        let meta_z =
            Meta::from_path(&path_z, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create the file with js extension;
        let path_j = tmp_dir.path().join("zzz.js");
        File::create(&path_j).expect("failed to create file");
        let meta_j =
            Meta::from_path(&path_j, false, PermissionFlag::Rwx).expect("failed to get meta");

        // Create the file with txt extension;
        let path_t = tmp_dir.path().join("zzz.txt");
        File::create(&path_t).expect("failed to create file");
        let meta_t =
            Meta::from_path(&path_t, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Extension;

        // Sort by extension
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_j), Ordering::Greater);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_t), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_version() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let path_a = tmp_dir.path().join("2");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_b = tmp_dir.path().join("11");
        File::create(&path_b).expect("failed to create file");
        let meta_b =
            Meta::from_path(&path_b, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_c = tmp_dir.path().join("12");
        File::create(&path_c).expect("failed to create file");
        let meta_c =
            Meta::from_path(&path_c, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Version;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_a), Ordering::Greater);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_c), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_no_sort() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let path_a = tmp_dir.path().join("aaa.aa");
        File::create(&path_a).expect("failed to create file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_b = tmp_dir.path().join("aaa");
        create_dir(&path_b).expect("failed to create dir");
        let meta_b =
            Meta::from_path(&path_b, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_c = tmp_dir.path().join("zzz.zz");
        File::create(&path_c).expect("failed to create file");
        let meta_c =
            Meta::from_path(&path_c, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_d = tmp_dir.path().join("zzz");
        create_dir(&path_d).expect("failed to create dir");
        let meta_d =
            Meta::from_path(&path_d, false, PermissionFlag::Rwx).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::None;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_b), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_c), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_d), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_c), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_d), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_c, &meta_d), Ordering::Equal);
    }

    #[test]
    fn test_sort_by_size() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let path_a = tmp_dir.path().join("aaa.aa");
        File::create(&path_a)
            .expect("failed to create file")
            .write_all(b"1, 2, 3")
            .expect("failed to write to file");
        let meta_a =
            Meta::from_path(&path_a, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_b = tmp_dir.path().join("bbb.bb");
        File::create(&path_b)
            .expect("failed to create file")
            .write_all(b"1, 2, 3, 4, 5, 6, 7, 8, 9, 10")
            .expect("failed to write file");
        let meta_b =
            Meta::from_path(&path_b, false, PermissionFlag::Rwx).expect("failed to get meta");

        let path_c = tmp_dir.path().join("ccc.cc");
        let path_d = tmp_dir.path().join("ddd.dd");

        #[cfg(unix)]
        std::os::unix::fs::symlink(path_d, &path_c).expect("failed to create broken symlink");

        // this needs to be tested on Windows
        // likely to fail because of permission issue
        // see https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(path_d, &path_c)
            .expect("failed to create broken symlink");

        let meta_c =
            Meta::from_path(&path_c, true, PermissionFlag::Rwx).expect("failed to get meta");

        assert_eq!(by_size(&meta_a, &meta_a), Ordering::Equal);
        assert_eq!(by_size(&meta_a, &meta_b), Ordering::Greater);
        assert_eq!(by_size(&meta_a, &meta_c), Ordering::Greater);

        assert_eq!(by_size(&meta_b, &meta_a), Ordering::Less);
        assert_eq!(by_size(&meta_b, &meta_b), Ordering::Equal);
        assert_eq!(by_size(&meta_b, &meta_c), Ordering::Greater);

        assert_eq!(by_size(&meta_c, &meta_a), Ordering::Less);
        assert_eq!(by_size(&meta_c, &meta_b), Ordering::Less);
        assert_eq!(by_size(&meta_c, &meta_c), Ordering::Equal);
    }
}
