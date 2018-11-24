use failure::*;
use size::*;
use std::fs::{read_link, Metadata};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Fail)]
pub enum MetaError {
    #[fail(display = "file name not readable for {}", path)]
    UnreadableName { path: String },
    #[fail(display = "invalid file name encoding for {}", path)]
    Encoding { path: String },
    #[fail(display = "unreachable metadatas for {}", path)]
    UnreadableMetadatas { path: String, err: io::Error },
}

#[derive(Debug)]
pub struct Meta {
    pub path: PathBuf,
    pub name: String,
    pub metadata: Metadata,
    pub group: String,
    pub user: String,
    pub symlink_target: Option<String>,
    pub size_value: String,
    pub size_unit: String,
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
        // metadatas.
        let (meta, symlink_target) = match read_link(path) {
            Ok(res) => {
                // This path is a symlink.
                //
                // Retrieve the symlink metadatas and return the link target.
                let meta = path
                    .symlink_metadata()
                    .expect("failed to retrieve symlink metadata");

                let symlink = res
                    .to_str()
                    .expect("failed to convert symlink to str")
                    .to_string();

                (meta, Some(symlink))
            }
            _ => {
                // This path is a file.
                //
                // Retireve the metadate and return no link target.
                let meta = match path.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        return Err(MetaError::UnreadableMetadatas {
                            path: path.display().to_string(),
                            err,
                        })
                    }
                };

                (meta, None)
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

        let size = Size::Bytes(meta.len()).to_string(Base::Base10, Style::Abbreviated);
        let size_parts: Vec<&str> = size.split(' ').collect();

        Ok(Meta {
            path: path.to_path_buf(),
            metadata: meta,
            name: String::from(name),
            user,
            group,
            symlink_target,
            size_value: size_parts[0].to_string(),
            size_unit: size_parts[1].to_string(),
        })
    }
}
