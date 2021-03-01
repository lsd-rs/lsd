use std::path::Path;

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
}