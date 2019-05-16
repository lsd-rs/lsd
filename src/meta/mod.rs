mod date;
mod filetype;
mod indicator;
mod name;
mod owner;
mod permissions;
mod size;
mod symlink;

#[cfg(windows)]
mod windows_utils;

pub use self::date::Date;
pub use self::filetype::FileType;
pub use self::indicator::Indicator;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use crate::flags::Display;
pub use crate::icon::Icons;

use std::fs::read_link;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
        display: Display,
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
            eprintln!("cannot access '{}': {}", path.display(), err);
            return Ok(meta);
        }
        let mut content = Vec::new();

        if let Display::DisplayAll = display {
            let mut current_meta;
            let mut parent_meta;

            let parent_path = match path.parent() {
                None => PathBuf::from("/"),
                Some(path) => PathBuf::from(path),
            };

            current_meta = Self::from_path(&path)?;
            current_meta.name.name = ".".to_string();

            parent_meta = Self::from_path(&parent_path)?;
            parent_meta.name.name = "..".to_string();

            content.push(current_meta);
            content.push(parent_meta);
        }

        for entry in meta.path.read_dir()? {
            let path = entry?.path();

            if let Display::DisplayOnlyVisible = display {
                if path
                    .file_name()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "invalid file name"))?
                    .to_string_lossy()
                    .starts_with('.')
                {
                    continue;
                }
            }

            let entry_meta =
                match Self::from_path_recursive(&path.to_path_buf(), depth - 1, display) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("cannot access '{}': {}", path.display(), err);
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

        #[cfg(unix)]
        let owner = Owner::from(&metadata);
        #[cfg(unix)]
        let permissions = Permissions::from(&metadata);

        #[cfg(windows)]
        let (owner, permissions) = windows_utils::get_file_data(&path)?;

        let file_type = FileType::new(&metadata, &permissions);
        let name = Name::new(&path, file_type);

        Ok(Self {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner,
            permissions,
            name,
            file_type,
            content: None,
        })
    }
}
