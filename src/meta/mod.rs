mod date;
mod filetype;
mod indicator;
mod inode;
mod links;
pub mod name;
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
pub use self::links::Links;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use crate::icon::Icons;

use crate::flags::{Display, Flags, Layout};
use crate::{print_error, ExitCode};

use std::fs::read_link;
use std::io::{Error, ErrorKind};
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
    pub links: Links,
    pub content: Option<Vec<Meta>>,
}

impl Meta {
    pub fn recurse_into(
        &self,
        depth: usize,
        flags: &Flags,
    ) -> Result<(Option<Vec<Meta>>, ExitCode), std::io::Error> {
        if depth == 0 {
            return Ok((None, ExitCode::OK));
        }

        if flags.display == Display::DirectoryOnly && flags.layout != Layout::Tree {
            return Ok((None, ExitCode::OK));
        }

        match self.file_type {
            FileType::Directory { .. } => (),
            FileType::SymLink { is_dir: true } => {
                if flags.layout == Layout::OneLine {
                    return Ok((None, ExitCode::OK));
                }
            }
            _ => return Ok((None, ExitCode::OK)),
        }

        let entries = match self.path.read_dir() {
            Ok(entries) => entries,
            Err(err) => {
                print_error!("{}: {}.", self.path.display(), err);
                return Ok((None, ExitCode::MinorIssue));
            }
        };

        let mut content: Vec<Meta> = Vec::new();

        if Display::All == flags.display && flags.layout != Layout::Tree {
            let mut current_meta;

            current_meta = self.clone();
            current_meta.name.name = ".".to_owned();

            let mut parent_meta =
                Self::from_path(&self.path.join(Component::ParentDir), flags.dereference.0)?;
            parent_meta.name.name = "..".to_owned();

            content.push(current_meta);
            content.push(parent_meta);
        }

        let mut exit_code = ExitCode::OK;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            let name = path
                .file_name()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "invalid file name"))?;

            if flags.ignore_globs.0.is_match(&name) {
                continue;
            }

            if let Display::VisibleOnly = flags.display {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            let mut entry_meta = match Self::from_path(&path, flags.dereference.0) {
                Ok(res) => res,
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    exit_code.set_if_greater(ExitCode::MinorIssue);
                    continue;
                }
            };

            // skip files for --tree -d
            if flags.layout == Layout::Tree {
                if let Display::DirectoryOnly = flags.display {
                    if !entry.file_type()?.is_dir() {
                        continue;
                    }
                }
            }

            match entry_meta.recurse_into(depth - 1, &flags) {
                Ok((content, rec_exit_code)) => {
                    entry_meta.content = content;
                    exit_code.set_if_greater(rec_exit_code);
                }
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    exit_code.set_if_greater(ExitCode::MinorIssue);
                    continue;
                }
            };

            content.push(entry_meta);
        }

        Ok((Some(content), exit_code))
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
                print_error!("{}: {}.", path.display(), err);
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
                    print_error!("{}: {}.", path.display(), err);
                    return size;
                }
            };
            for entry in entries {
                let path = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => {
                        print_error!("{}: {}.", path.display(), err);
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
        let (metadata, symlink_meta) = if read_link(path).is_ok() && !dereference {
            (path.symlink_metadata()?, path.metadata().ok())
        } else {
            (path.metadata()?, None)
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
        let links = Links::from(&metadata);

        Ok(Self {
            inode,
            links,
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
