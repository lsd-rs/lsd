use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct INodeCount {
    nlink: Option<u64>,
}

impl<'a> From<&'a Metadata> for INodeCount {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let nlink = meta.nlink();

        Self { nlink: Some(nlink) }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { nlink: None }
    }
}

impl INodeCount {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.nlink {
            Some(i) => colors.colorize(i.to_string(), &Elem::INodeCount { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::INodeCount { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::INodeCount;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_hardlinks_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let inode = INodeCount::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(inode.nlink.is_some());
        #[cfg(windows)]
        assert!(inode.nlink.is_none());
    }
}
