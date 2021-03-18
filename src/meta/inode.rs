use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct INode {
    #[cfg(unix)]
    index: u64,
}

impl<'a> From<&'a Metadata> for INode {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        Self { index: meta.ino() }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self {}
    }
}

impl INode {
    #[cfg(unix)]
    pub fn render(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.index.to_string(), &Elem::INode { valid: true })
    }

    #[cfg(windows)]
    pub fn render(&self, colors: &Colors) -> ColoredString {
        colors.colorize(String::from("-"), &Elem::INode { valid: false })
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::INode;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_inode_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let inode = INode::from(&file_path.metadata().unwrap());
        assert!(inode.index > 0);
    }
}
