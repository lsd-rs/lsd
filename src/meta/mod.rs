mod date;
mod filetype;
mod indicator;
mod inode;
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
pub use self::inode::INode;
pub use self::name::{DisplayOption, Name};
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use crate::icon::Icons;

use crate::flags::{Display, Flags, Layout};
use crate::print_error;

use std::fs::read_link;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
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
    pub inode: INode,
    pub content: Option<Vec<Meta>>,
}

impl Meta {
    pub fn recurse_into(
        &self,
        depth: usize,
        flags: &Flags,
    ) -> Result<Option<Vec<Meta>>, std::io::Error> {
        if depth == 0 || flags.display == Display::DirectoryItself {
            return Ok(None);
        }

        match self.file_type {
            FileType::Directory { .. } => (),
            FileType::SymLink { is_dir: true } => {
                if flags.layout == Layout::OneLine {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        }

        let entries = match self.path.read_dir() {
            Ok(entries) => entries,
            Err(err) => {
                print_error!("lsd: {}: {}\n", self.path.display(), err);
                return Ok(None);
            }
        };

        let mut content: Vec<Meta> = Vec::new();

        if let Display::All = flags.display {
            let mut current_meta = self.clone();
            current_meta.name.set_name(".".to_owned());

            let parent_meta =
                Self::from_path(&self.path.join(Component::ParentDir), flags.dereference.0)?;

            content.push(current_meta);
            content.push(parent_meta);
        }

        for entry in entries {
            let entry = entry?;
            let name = entry.file_name();

            if flags.ignore_globs.0.is_match(&name) {
                continue;
            }

            if let Display::DisplayOnlyVisible = flags.display {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            let mut entry_meta = match Self::from_path(&entry.path(), flags.dereference.0) {
                Ok(res) => res,
                Err(err) => {
                    print_error!("lsd: {:?}: {}\n", entry.path(), err);
                    continue;
                }
            };

            match entry_meta.recurse_into(depth - 1, &flags) {
                Ok(content) => entry_meta.content = content,
                Err(err) => {
                    print_error!("lsd: {:?}: {}\n", entry.path(), err);
                    continue;
                }
            };

            content.push(entry_meta);
        }

        Ok(Some(content))
    }

    pub fn calculate_total_size(&mut self) {
        if let FileType::Directory { .. } = self.file_type {
            if let Some(metas) = &mut self.content {
                let mut size_accumulated = self.size.get_bytes();
                for x in &mut metas.iter_mut() {
                    x.calculate_total_size();
                    size_accumulated += x.size.get_bytes();
                }
                self.size = Size::new(size_accumulated);
            } else {
                // possibility that 'depth' limited the recursion in 'recurse_into'
                self.size = Size::new(Meta::calculate_total_file_size(&self.path));
            }
        }
    }

    fn calculate_total_file_size(path: &PathBuf) -> u64 {
        let metadata = if read_link(&path).is_ok() {
            // If the file is a link, retrieve the metadata without following
            // the link.
            path.symlink_metadata()
        } else {
            path.metadata()
        };
        let metadata = match metadata {
            Ok(meta) => meta,
            Err(err) => {
                print_error!("lsd: {}: {}\n", path.display(), err);
                return 0;
            }
        };
        let file_type = metadata.file_type();
        if file_type.is_file() {
            metadata.len()
        } else if file_type.is_dir() {
            let mut size = metadata.len();

            let entries = match path.read_dir() {
                Ok(entries) => entries,
                Err(err) => {
                    print_error!("lsd: {}: {}\n", path.display(), err);
                    return size;
                }
            };
            for entry in entries {
                let path = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => {
                        print_error!("lsd: {}: {}\n", path.display(), err);
                        continue;
                    }
                };
                size += Meta::calculate_total_file_size(&path);
            }
            size
        } else {
            0
        }
    }

    pub fn from_path(path: &Path, dereference: bool) -> Result<Self, std::io::Error> {
        // If the file is a link then retrieve link metadata instead with target metadata (if present).
        let (metadata, symlink_meta) = match path.symlink_metadata() {
            Ok(metadata) if !dereference => (metadata, path.metadata().ok()),
            _ => (path.metadata()?, None),
        };

        #[cfg(unix)]
        let owner = Owner::from(&metadata);
        #[cfg(unix)]
        let permissions = Permissions::from(&metadata);

        #[cfg(windows)]
        let (owner, permissions) = windows_utils::get_file_data(&path)?;

        let file_type = FileType::new(&metadata, symlink_meta.as_ref(), &permissions);
        let name = Name::new(&path, file_type);
        let inode = INode::from(&metadata);

        Ok(Self {
            inode,
            path: path.to_path_buf(),
            symlink: SymLink::from(path),
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
