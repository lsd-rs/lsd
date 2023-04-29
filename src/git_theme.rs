use crate::flags::GitTheme as GitFlagTheme;
use crate::git::GitStatus;
use crate::theme::git::GitThemeSymbols;
use crate::theme::Theme;
use std::path::Path;

pub struct GitTheme {
    symbols: GitThemeSymbols,
}

impl GitTheme {
    pub fn new(theme: GitFlagTheme) -> GitTheme {
        let git_symbols = match theme {
            GitFlagTheme::Default => GitThemeSymbols::default(),
            GitFlagTheme::Custom(ref file) => Theme::from_path::<GitThemeSymbols>(
                Path::new("themes").join(file).to_str().unwrap_or(file),
            )
            .unwrap_or_default(),
        };

        Self {
            symbols: git_symbols,
        }
    }

    pub fn get_symbol(&self, status: &GitStatus) -> String {
        let symbol = match status {
            GitStatus::Default => &self.symbols.default,
            GitStatus::Unmodified => &self.symbols.unmodified,
            GitStatus::Ignored => &self.symbols.ignored,
            GitStatus::NewInIndex => &self.symbols.new_in_index,
            GitStatus::NewInWorkdir => &self.symbols.new_in_workdir,
            GitStatus::Typechange => &self.symbols.typechange,
            GitStatus::Deleted => &self.symbols.deleted,
            GitStatus::Renamed => &self.symbols.renamed,
            GitStatus::Modified => &self.symbols.modified,
            GitStatus::Conflicted => &self.symbols.conflicted,
        };
        symbol.to_string()
    }
}
