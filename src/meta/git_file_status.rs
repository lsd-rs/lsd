use crate::color::{self, ColoredString, Colors};
use crate::git::GitStatus;
use crate::git_theme::GitTheme;

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

impl GitFileStatus {
    #[cfg(not(feature = "no-git"))]
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

    pub fn render(&self, colors: &Colors, git_theme: &GitTheme) -> ColoredString {
        let res = [
            colors.colorize(
                git_theme.get_symbol(&self.index),
                &color::Elem::GitStatus { status: self.index },
            ),
            colors.colorize(
                git_theme.get_symbol(&self.workdir),
                &color::Elem::GitStatus {
                    status: self.workdir,
                },
            ),
        ]
        .into_iter()
        // From the experiment, the maximum string size is 153 bytes
        .fold(String::with_capacity(160), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        });
        ColoredString::new(Colors::default_style(), res)
    }
}
