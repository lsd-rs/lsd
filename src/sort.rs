use crate::flags::{DirOrderFlag, Flags, SortFlag, SortOrder};
use crate::meta::{FileType, Meta};
use std::cmp::Ordering;

pub fn by_meta(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match flags.sort_by {
        SortFlag::Name => match flags.directory_order {
            DirOrderFlag::First => by_name_with_dirs_first(a, b, flags),
            DirOrderFlag::None => by_name(a, b, flags),
            DirOrderFlag::Last => by_name_with_files_first(a, b, flags),
        },

        SortFlag::Time => match flags.directory_order {
            DirOrderFlag::First => by_date_with_dirs_first(a, b, flags),
            DirOrderFlag::None => by_date(a, b, flags),
            DirOrderFlag::Last => by_date_with_files_first(a, b, flags),
        },
    }
}

fn by_name(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        a.name.cmp(&b.name)
    } else {
        b.name.cmp(&a.name)
    }
}

fn by_name_with_dirs_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_name(a, b, flags),
        (FileType::Directory { .. }, _) => Ordering::Less,
        (_, FileType::Directory { .. }) => Ordering::Greater,
        _ => by_name(a, b, flags),
    }
}

fn by_name_with_files_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_name(a, b, flags),
        (FileType::Directory { .. }, _) => Ordering::Greater,
        (_, FileType::Directory { .. }) => Ordering::Less,
        _ => by_name(a, b, flags),
    }
}

fn by_date(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        b.date.cmp(&a.date).then(a.name.cmp(&b.name))
    } else {
        a.date.cmp(&b.date).then(b.name.cmp(&a.name))
    }
}

fn by_date_with_dirs_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_date(a, b, flags),
        (FileType::Directory { .. }, _) => Ordering::Less,
        (_, FileType::Directory { .. }) => Ordering::Greater,
        _ => by_date(a, b, flags),
    }
}

fn by_date_with_files_first(a: &Meta, b: &Meta, flags: Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_date(a, b, flags),
        (FileType::Directory { .. }, _) => Ordering::Greater,
        (_, FileType::Directory { .. }) => Ordering::Less,
        _ => by_date(a, b, flags),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flags;
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
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Greater);

        //  Sort with the dirs first (the dirs stay first)
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
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
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort with file first reversed (thie files stay first)
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Less);
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
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort by name unordered
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
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
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Greater);

        // Sort by name unordered reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Less);
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
            .arg(").lastwritetime=$(Get-Date \"11/16/1985\")")
            .status()
            .unwrap()
            .success();

        assert_eq!(true, success, "failed to change file timestamp");
        let meta_z = Meta::from_path(&path_z).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sort_by = SortFlag::Time;

        // Sort by time
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Less);

        // Sort by time reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, flags), Ordering::Greater);
    }
}
