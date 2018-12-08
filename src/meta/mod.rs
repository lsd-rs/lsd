mod date;
mod filetype;
mod name;
mod owner;
mod permissions;
mod size;
mod symlink;

pub use self::date::Date;
pub use self::filetype::FileType;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;

use std::fs::read_link;
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
}

impl Meta {
    pub fn from_path(path: &PathBuf) -> Option<Self> {
        let metadata = if read_link(path).is_ok() {
            // If the file is a link, retrieve the metadata without following
            // the link.
            path.symlink_metadata()
                .expect("failed to retrieve symlink metadata")
        } else {
            match path.metadata() {
                Ok(res) => res,
                Err(err) => {
                    println!("cannot access '{}': {}", path.display(), err);
                    return None;
                }
            }
        };

        let file_type = FileType::from(&metadata);

        Some(Meta {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            permissions: Permissions::from(&metadata),
            date: Date::from(&metadata),
            name: Name::new(&path, file_type),
            owner: Owner::from(&metadata),
            file_type,
        })
    }
}
