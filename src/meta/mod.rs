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
use std::path::Path;

#[derive(Debug)]
pub struct Meta {
    pub name: Name,
    pub permissions: Permissions,
    pub date: Date,
    pub owner: Owner,
    pub file_type: FileType,
    pub size: Size,
    pub symlink: Option<SymLink>,
}

impl<'a> From<&'a Path> for Meta {
    fn from(path: &Path) -> Self {
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

        Meta {
            symlink,
            size: Size::from(&metadata),
            permissions: Permissions::from(&metadata),
            date: Date::from(&metadata),
            name: Name::new(&path, file_type),
            owner,
            file_type,
        }
    }
}
