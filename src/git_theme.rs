use crate::git::GitStatus;
use crate::theme::git::GitThemeSymbols;

pub struct GitTheme {
    symbols: GitThemeSymbols,
}

impl GitTheme {
    pub fn new() -> GitTheme {
        let git_symbols = GitThemeSymbols::default();
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

#[test]
fn test_git_get_symbol() {
    let git_theme = GitTheme::new();
    let mut git_status = GitStatus::Default;
    assert_eq!(git_theme.get_symbol(&git_status), "-".to_string());
    git_status = GitStatus::Unmodified;
    assert_eq!(git_theme.get_symbol(&git_status), ".".to_string());
    git_status = GitStatus::Ignored;
    assert_eq!(git_theme.get_symbol(&git_status), "I".to_string());
    git_status = GitStatus::NewInIndex;
    assert_eq!(git_theme.get_symbol(&git_status), "N".to_string());
    git_status = GitStatus::NewInWorkdir;
    assert_eq!(git_theme.get_symbol(&git_status), "?".to_string());
    git_status = GitStatus::Typechange;
    assert_eq!(git_theme.get_symbol(&git_status), "T".to_string());
    git_status = GitStatus::Deleted;
    assert_eq!(git_theme.get_symbol(&git_status), "D".to_string());
    git_status = GitStatus::Renamed;
    assert_eq!(git_theme.get_symbol(&git_status), "R".to_string());
    git_status = GitStatus::Modified;
    assert_eq!(git_theme.get_symbol(&git_status), "M".to_string());
    git_status = GitStatus::Conflicted;
    assert_eq!(git_theme.get_symbol(&git_status), "C".to_string());
}
