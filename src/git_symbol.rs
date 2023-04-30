use crate::flags::{IconTheme as FlagTheme, IconTheme};
use crate::theme::git_symbol::GitSymbolTheme;

pub struct GitSymbols {
    theme: GitSymbolTheme,
}

impl GitSymbols {
    pub fn new(theme: FlagTheme) -> GitSymbols {
        let git_theme = match theme {
            IconTheme::Unicode => GitSymbolTheme::unicode(),
            IconTheme::Fancy => GitSymbolTheme::default(),
        };

        Self { theme: git_theme }
    }

    pub fn get_status(&self, status: &crate::git::GitStatus) -> String {
        use crate::git::GitStatus;
        let icon = match status {
            GitStatus::Default => &self.theme.default,
            GitStatus::Unmodified => &self.theme.unmodified,
            GitStatus::Ignored => &self.theme.ignored,
            GitStatus::NewInIndex => &self.theme.new_in_index,
            GitStatus::NewInWorkdir => &self.theme.new_in_workdir,
            GitStatus::Typechange => &self.theme.typechange,
            GitStatus::Deleted => &self.theme.deleted,
            GitStatus::Renamed => &self.theme.renamed,
            GitStatus::Modified => &self.theme.modified,
            GitStatus::Conflicted => &self.theme.conflicted,
        };
        icon.to_string()
    }
}
