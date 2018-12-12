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
pub use icon::Icons;

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
    pub indicator: Indicator,
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

        let permissions = Permissions::from(&metadata);
        let file_type = FileType::new(&metadata, &permissions);
        let name = Name::new(&path, file_type);

        Some(Meta {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner: Owner::from(&metadata),
            permissions,
            name,
            file_type,
        })
    }
}
