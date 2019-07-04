use crate::flags::{DirOrderFlag, Flags, SortFlag, SortOrder};
use crate::meta::{FileType, Meta};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::ffi::OsStr;

pub fn by_meta(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match flags.sort_by {
        SortFlag::Name => match flags.directory_order {
            DirOrderFlag::First => by_name_with_dirs_first(a, b, &flags),
            DirOrderFlag::None => by_name(a, b, &flags),
            DirOrderFlag::Last => by_name_with_files_first(a, b, &flags),
        },
        SortFlag::Size => match flags.directory_order {
            DirOrderFlag::First => by_size(a, b, flags),
            DirOrderFlag::None => by_size(a, b, flags),
            DirOrderFlag::Last => by_size(a, b, flags),
        },
        SortFlag::Time => match flags.directory_order {
            DirOrderFlag::First => by_date_with_dirs_first(a, b, &flags),
            DirOrderFlag::None => by_date(a, b, &flags),
            DirOrderFlag::Last => by_date_with_files_first(a, b, &flags),
        },
        _ => panic!("This should not appear unless things are fucked up"),
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

fn by_name_with_dirs_first(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_name(a, b, &flags),
        (FileType::Directory { .. }, _) => Ordering::Less,
        (_, FileType::Directory { .. }) => Ordering::Greater,
        _ => by_name(a, b, &flags),
    }
}

fn by_name_with_files_first(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_name(a, b, &flags),
        (FileType::Directory { .. }, _) => Ordering::Greater,
        (_, FileType::Directory { .. }) => Ordering::Less,
        _ => by_name(a, b, &flags),
    }
}

fn by_date(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    if flags.sort_order == SortOrder::Default {
        b.date.cmp(&a.date).then(a.name.cmp(&b.name))
    } else {
        a.date.cmp(&b.date).then(b.name.cmp(&a.name))
    }
}

fn by_date_with_dirs_first(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_date(a, b, &flags),
        (FileType::Directory { .. }, _) => Ordering::Less,
        (_, FileType::Directory { .. }) => Ordering::Greater,
        _ => by_date(a, b, &flags),
    }
}

fn by_date_with_files_first(a: &Meta, b: &Meta, flags: &Flags) -> Ordering {
    match (a.file_type, b.file_type) {
        (FileType::Directory { .. }, FileType::Directory { .. }) => by_date(a, b, &flags),
        (FileType::Directory { .. }, _) => Ordering::Greater,
        (_, FileType::Directory { .. }) => Ordering::Less,
        _ => by_date(a, b, &flags),
    }
}

// Return's extension and bool to tell if it is a directory.
fn grab_extension(path: &PathBuf) -> (String, bool) {
    let extension = path.as_path().extension().and_then(OsStr::to_str);
    match extension {
        Some(ext) => (ext.to_string(), false),
        None => ("".to_string(), path.is_dir())
    }
}

pub fn by_extension(metas: Vec<Meta>, flags: &Flags) -> Vec<Meta> {
    let mut extensions: Vec<(String, bool)> = Vec::new();
    for m in metas.clone() {
        extensions.push(grab_extension(&m.path));
    }

    extensions.sort();
    
    //(FileExtension, Vec<(name, is_directory)>)
    let mut items: Vec<(String, Vec<(String, bool)> )> = Vec::new();

    'start: for m in metas.clone() {
        for e in extensions.clone() {
            if e == grab_extension(&m.path) {
                extensions.remove_item(&e);
                for i in 0..items.len() {
                    if &items[i].0 == &e.0 && &items[i].1[0].1 == &e.1 {
                        items[i].1.push((m.name.name.clone(), e.1));
                        continue 'start
                    }
                }

                items.push( (e.0.clone(), vec![(m.name.name.clone(), e.1)]) );
                continue 'start
            }
        }
    }

    // Then sort the items inside the items extensions tuples vectors.
    for i in 0..items.len() {
        items[i].1.sort();
    }

    // Do stuff to deal with directory options, by default directories come first.
    let mut dir_index: Option<usize> = None;
    for i in 0..items.len() {
        if items[i].0 == "" && (items[i].1)[0].1 {
            dir_index = Some(i.clone());
            break;
        }
    }

    match dir_index {
        Some(dir_index) => {
            let dirs = items.remove(dir_index);
            if flags.directory_order == DirOrderFlag::Last {
                items.insert(items.len(), dirs);
            } else {
                items.insert(0, dirs);
            }
        },
        None => (), // There are no directories which we have to deal with.
    }

    // Create new Vector of Metas according to the order of things in "items" Vector.
    let mut result: Vec<Meta> = Vec::new();
    for e in items {
        for i in e.1 {
            for mn in &metas {
                if mn.name.name == i.0 {
                    result.push(mn.clone());
                }
            }
        }
    }

    result
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

        //  Sort with the dirs first
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Greater);

        //  Sort with the dirs first (the dirs stay first)
        flags.sort_order = SortOrder::Reverse;
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Greater);
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

        // Sort with file first
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Less);

        // Sort with file first reversed (thie files stay first)
        assert_eq!(by_meta(&meta_a, &meta_z, &flags), Ordering::Less);
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
