#[cfg(all(
    feature = "git",
    not(any(
        all(target_os = "linux", target_arch = "arm"),
        all(windows, target_arch = "x86", target_env = "gnu")
    ))
))]
use crate::git::GitStatus;
#[cfg(any(
    not(feature = "git"),
    all(target_os = "linux", target_arch = "arm"),
    all(windows, target_arch = "x86", target_env = "gnu")
))]
use crate::git_stub::GitStatus;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GitFileStatus {
    pub index: GitStatus,
    pub workdir: GitStatus,
}

impl Default for GitFileStatus {
    fn default() -> Self {
        Self {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        }
    }
}

#[cfg(all(
    feature = "git",
    not(any(
        all(target_os = "linux", target_arch = "arm"),
        all(windows, target_arch = "x86", target_env = "gnu")
    ))
))]
impl GitFileStatus {
    pub fn new(status: git2::Status) -> Self {
        Self {
            index: match status {
                s if s.contains(git2::Status::INDEX_NEW) => GitStatus::NewInIndex,
                s if s.contains(git2::Status::INDEX_DELETED) => GitStatus::Deleted,
                s if s.contains(git2::Status::INDEX_MODIFIED) => GitStatus::Modified,
                s if s.contains(git2::Status::INDEX_RENAMED) => GitStatus::Renamed,
                s if s.contains(git2::Status::INDEX_TYPECHANGE) => GitStatus::Typechange,
                _ => GitStatus::Unmodified,
            },

            workdir: match status {
                s if s.contains(git2::Status::WT_NEW) => GitStatus::NewInWorkdir,
                s if s.contains(git2::Status::WT_DELETED) => GitStatus::Deleted,
                s if s.contains(git2::Status::WT_MODIFIED) => GitStatus::Modified,
                s if s.contains(git2::Status::WT_RENAMED) => GitStatus::Renamed,
                s if s.contains(git2::Status::IGNORED) => GitStatus::Ignored,
                s if s.contains(git2::Status::WT_TYPECHANGE) => GitStatus::Typechange,
                s if s.contains(git2::Status::CONFLICTED) => GitStatus::Conflicted,
                _ => GitStatus::Unmodified,
            },
        }
    }

    pub fn render(
        &self,
        colors: &crate::color::Colors,
        icons: &crate::icon::Icons,
    ) -> crate::color::ColoredString {
        let strings = &[
            colors.colorize(
                icons.get_status(&self.index),
                &crate::color::Elem::GitStatus { status: self.index },
            ),
            crate::color::ColoredString::from(" "),
            colors.colorize(
                icons.get_status(&self.workdir),
                &crate::color::Elem::GitStatus {
                    status: self.workdir,
                },
            ),
        ];
        let res = ansi_term::ANSIStrings(strings).to_string();
        crate::color::ColoredString::from(res)
    }
}
