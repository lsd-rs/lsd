use self::size::Size;
use failure::*;
use std::fs::{read_link, Metadata};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

mod size;

#[derive(Debug, Fail)]
pub enum MetaError {
    #[fail(display = "file name not readable for {}", path)]
    UnreadableName { path: String },
    #[fail(display = "invalid file name encoding for {}", path)]
    Encoding { path: String },
    #[fail(display = "unreachable metadatas for {}", path)]
    UnreadableMetadatas { path: String, err: io::Error },
}

#[derive(Debug, PartialEq)]
pub enum Type {
    SymLink(String),
    File,
    Directory,
}

impl<'a> From<&'a Metadata> for Type {
    fn from(meta: &'a Metadata) -> Self {
        if meta.is_dir() {
            Type::Directory
        } else {
            Type::File
        }
    }
}

#[derive(Debug)]
pub struct Meta {
    pub path: PathBuf,
    pub name: String,
    pub metadata: Metadata,
    pub group: String,
    pub user: String,
    pub node_type: Type,
    pub size: Size,
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

        // Check if the path is a symlink or not and retrieve the corresponding
        // metadatas, and type.
        let (meta, node_type) = match read_link(path) {
            Ok(res) => {
                // This path is a symlink.
                //
                // Retrieve the symlink metadatas and return the link target.
                let meta = path
                    .symlink_metadata()
                    .expect("failed to retrieve symlink metadata");

                let target = res
                    .to_str()
                    .expect("failed to convert symlink to str")
                    .to_string();

                (meta, Type::SymLink(target))
            }
            _ => {
                // This path is a file.
                //
                // Retireve the metadata and return the node_type.
                let meta = match path.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        return Err(MetaError::UnreadableMetadatas {
                            path: path.display().to_string(),
                            err,
                        })
                    }
                };

                let node_type = Type::from(&meta);
                (meta, node_type)
            }
        };

        let user = get_user_by_uid(meta.uid())
            .expect("failed to get user name")
            .name()
            .to_str()
            .expect("failed to convert user name to str")
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("failed to get the group name")
            .name()
            .to_str()
            .expect("failed to convert group name to str")
            .to_string();

        let size = meta.len();
        Ok(Meta {
            path: path.to_path_buf(),
            metadata: meta,
            name: String::from(name),
            user,
            group,
            node_type: node_type,
            size: Size::from_bytes(size as usize),
        })
    }
}
