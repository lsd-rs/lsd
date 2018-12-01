mod filetype;
mod owner;
mod permissions;
mod size;
mod symlink;

pub use self::filetype::FileType;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;

use failure::*;
use std::fs::read_link;
use std::fs::Metadata;
use std::path::{Path, PathBuf};

#[derive(Debug, Fail)]
pub enum MetaError {
    #[fail(display = "file name not readable for {}", path)]
    UnreadableName { path: String },
    #[fail(display = "invalid file name encoding for {}", path)]
    Encoding { path: String },
}

#[derive(Debug)]
pub struct Meta {
    pub path: PathBuf,
    pub name: String,
    pub permissions: Permissions,
    pub metadata: Metadata,
    pub owner: Owner,
    pub file_type: FileType,
    pub size: Size,
    pub symlink: Option<SymLink>,
}

impl Meta {
    pub fn from_path(path: &Path) -> Result<Self, MetaError> {
        // Retrieve and convert the name into an utf-8 String.
        let name = match path.file_name() {
            Some(os_str_name) => match os_str_name.to_str() {
                Some(name) => name,
                None => {
                    return Err(MetaError::Encoding {
                        path: path.display().to_string(),
                    })
                }
            },
            None => {
                return Err(MetaError::UnreadableName {
                    path: path.display().to_string(),
                })
            }
        };

        let mut metadata = path.metadata().expect("failed to retrieve metadata");
        let mut symlink = None;
        if let Ok(target) = read_link(path) {
            // If the file is a link, retrieve the metadata without following
            // the link.
            metadata = path
                .symlink_metadata()
                .expect("failed to retrieve symlink metadata");
            symlink = Some(SymLink::from(&target));
        }

        let file_type = FileType::from(&metadata);
        let owner = Owner::from(&metadata);

        Ok(Meta {
            symlink,
            size: Size::from(&metadata),
            permissions: Permissions::from(&metadata),
            path: path.to_path_buf(),
            metadata,
            name: String::from(name),
            owner,
            file_type,
        })
    }
}
