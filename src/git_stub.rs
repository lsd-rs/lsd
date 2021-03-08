use crate::meta::GitFileStatus;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GitStatus {
    /// No status info
    Default,
}

pub struct GitCache;

impl GitCache {
    pub fn new(_: &Path) -> Self {
        Self {}
    }

    pub fn get(&self, _filepath: &PathBuf, _is_directory: bool) -> Option<GitFileStatus> {
        None
    }
}
