use crate::flags::{DirOrderFlag, Flags, SortFlag, SortOrder};
use crate::meta::{FileType, Meta};
use std::cmp::Ordering;

pub fn call(flags: &Flags, mut metas: Vec<Meta>) -> Vec<Meta> {
    sort_with_dir(metas, &flags)
}

fn sort_with_dir(mut metas: Vec<Meta>, flags: &Flags) -> Vec<Meta> {
    match flags.directory_order {
        DirOrderFlag::First => {
            println!("first");
            let (mut directories, mut other) = split_dirs(metas, flags);
            directories.append(&mut other);
            return directories
        },
        DirOrderFlag::Last => {
            println!("last");

            let (mut directories, mut other) = split_dirs(metas, flags);
            other.append(&mut directories);
            return other
        },
        DirOrderFlag::None => {
            sort(flags, &mut metas);
            return metas
        },
    }
}

fn split_dirs(metas: Vec<Meta>, flags: &Flags) -> (Vec<Meta>, Vec<Meta>) {
    let mut directories = Vec::new();
    let mut other = Vec::new();

    for meta in metas.into_iter() {
        match meta.file_type {
            FileType::Directory{uid: _} => directories.push(meta),
            _ => other.push(meta),
        }
    }

    sort(flags, &mut directories);
    sort(flags, &mut other);

    (directories, other)
}


pub fn sort(flags: &Flags, metas: &mut Vec<Meta>) {
    metas.sort_unstable_by(|a, b| by_meta(a, b, flags));

    for meta in metas {
        if let Some(ref mut content) = meta.content {
            sort(flags, content);
        }
    }
}

pub fn by_meta(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match flags.sort_by {
        SortFlag::Name => by_name(a, b, &flags),
        SortFlag::Size => by_size(a, b, &flags),
        SortFlag::Time => by_date(a, b, &flags),
    }
}

fn by_size(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        b.size.get_bytes().cmp(&a.size.get_bytes())
    } else {
        a.size.get_bytes().cmp(&b.size.get_bytes())
    }
}

fn by_name(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        a.name.cmp(&b.name)
    } else {
        b.name.cmp(&a.name)
    }
}

fn by_date(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        b.date.cmp(&a.date).then(a.name.cmp(&b.name))
    } else {
        a.date.cmp(&b.date).then(b.name.cmp(&a.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flags;
    use std::fs::{create_dir, File};
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn test_sort_by_meta_by_name_with_dirs_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

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

        let mut sorted = call(&flags, vec![meta_a, meta_z]);
        if let FileType::Directory { .. } = sorted.pop().unwrap().file_type {
            panic!("Wasn't a directory")
        }
    }

    #[test]
    fn test_sort_by_meta_by_name_with_files_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

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

        let mut sorted = call(&flags, vec![meta_a, meta_z]);

        if let FileType::File { .. } = sorted.pop().unwrap().file_type {
            panic!("Wasn't a directory")
        }
    }

    #[test]
    fn test_sort_by_meta_by_name_unordered() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

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
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Less);

        // Sort by name unordered
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Greater);
    }

    #[test]
    fn test_sort_by_meta_by_name_unordered_2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

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
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Greater);

        // Sort by name unordered reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Less);
    }

    #[test]
    fn test_sort_by_meta_by_time() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

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
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Less);

        // Sort by time reversed
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Greater);
    }
}
