mod date;
mod filetype;
mod indicator;
mod name;
mod owner;
mod permissions;
mod size;
mod symlink;

pub use self::date::Date;
pub use self::filetype::FileType;
pub use self::indicator::Indicator;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use crate::icon::Icons;

use std::fs::read_link;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
pub struct Meta {
    pub name: Name,
    pub path: PathBuf,
    pub permissions: Permissions,
    pub date: Date,
    pub owner: Owner,
    pub file_type: FileType,
    pub size: Size,
    pub symlink: SymLink,
    pub indicator: Indicator,
    pub content: Option<Vec<Meta>>,
}

impl Meta {
    pub fn from_path_recursive(
        path: &PathBuf,
        depth: usize,
        list_hidden_files: bool,
    ) -> Result<Self, std::io::Error> {
        let mut meta = Self::from_path(path)?;

        if depth == 0 {
            return Ok(meta);
        }

        match meta.file_type {
            FileType::Directory { .. } => (),
            _ => return Ok(meta),
        }

        if let Err(err) = meta.path.read_dir() {
            println!("cannot access '{}': {}", path.display(), err);
            return Ok(meta);
        }
		let mut content = Vec::new();

		if list_hidden_files {
			let mut current_meta;
			let mut parent_meta;

			let path_str = path.to_str().unwrap();

			if path_str == ".." {
				let temp_path = fs::canonicalize(&PathBuf::from("..")).unwrap();
				let parent_temp_path = match temp_path.parent() {
					None => PathBuf::from("/"),
					Some(path) => PathBuf::from(path),
				};

				current_meta = Self::from_path(&PathBuf::from(temp_path))?;
				current_meta.name.name = ".".to_string();

				parent_meta = Self::from_path(&PathBuf::from(parent_temp_path))?;
				parent_meta.name.name = "..".to_string();
			} else if path_str == "." {
				current_meta = Self::from_path(&PathBuf::from("."))?;
				parent_meta = Self::from_path(&PathBuf::from(".."))?;
			} else {
				let parent_path = match  path.parent() {
					None => PathBuf::from("/"),
					Some(path) => PathBuf::from(path),
				};

				current_meta = Self::from_path(&PathBuf::from(path))?;
				current_meta.name.name = ".".to_string();

				parent_meta = Self::from_path(&PathBuf::from(parent_path))?;
				parent_meta.name.name = "..".to_string();
			}

			content.push(current_meta);
			content.push(parent_meta);
		}

        for entry in meta.path.read_dir()? {
            let path = entry?.path();

            if !list_hidden_files
                && path
                    .file_name()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "invalid file name"))?
                    .to_string_lossy()
                    .starts_with('.')
            {
                continue;
            }

            let entry_meta = match Self::from_path_recursive(
                &path.to_path_buf(),
                depth - 1,
                list_hidden_files,
            ) {
                Ok(res) => res,
                Err(err) => {
                    println!("cannot access '{}': {}", path.display(), err);
                    continue;
                }
            };

            content.push(entry_meta);
        }

        if !content.is_empty() {
            meta.content = Some(content);
        }

        Ok(meta)
    }

    pub fn from_path(path: &PathBuf) -> Result<Self, std::io::Error> {
        let metadata = if read_link(path).is_ok() {
            // If the file is a link, retrieve the metadata without following
            // the link.
            path.symlink_metadata()?
        } else {
            path.metadata()?
        };

        let permissions = Permissions::from(&metadata);
        let file_type = FileType::new(&metadata, &permissions);
        let name = Name::new(&path, file_type);

        Ok(Self {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner: Owner::from(&metadata),
            permissions,
            name,
            file_type,
            content: None,
        })
    }

/*
	pub fn from_current_path(path: &PathBuf) -> Result<Self, std::io::Error> {
		let metadata = if read_link(path).is_ok() {
			path.symlink_metadata()?
		} else {
			path.metadata()?
		};

		let permissions = Permissions::from(&metadata);
        let file_type = FileType::new(&metadata, &permissions);
        let name = Name::new(&PathBuf::from("."), file_type);

		 Ok(Self {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner: Owner::from(&metadata),
            permissions,
            name,
            file_type,
            content: None,
        })
	}

	pub fn from_parent_path(path: &PathBuf) -> Result<Self, std::io::Error> {
		let metadata = if read_link(path).is_ok() {
			path.symlink_metadata()?
		} else {
			path.metadata()?
		};

		let permissions = Permissions::from(&metadata);
        let file_type = FileType::new(&metadata, &permissions);
        let name = Name::new(&PathBuf::from(".."), file_type);

		 Ok(Self {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner: Owner::from(&metadata),
            permissions,
            name,
            file_type,
            content: None,
        })
	}
*/
}
