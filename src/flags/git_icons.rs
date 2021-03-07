use crate::git::GitStatus;
use crate::icon::Theme;

pub struct GitIcons {
    theme: Theme,
}

impl GitIcons {
    pub fn new(theme: Theme) -> GitIcons {
        GitIcons { theme }
    }

    pub fn get(&self, status: &GitStatus) -> String {
        match self.theme {
            Theme::NoIcon => self.get_text(status),
            Theme::Fancy => self.get_icon(status),
            Theme::Unicode => self.get_unicode(status),
        }
    }

    fn get_text(&self, status: &GitStatus) -> String {
        match status {
            GitStatus::Default => "-",
            GitStatus::Unmodified => "-",
            GitStatus::NewInIndex => "N",
            GitStatus::NewInWorkdir => "?",
            GitStatus::Deleted => "D",
            GitStatus::Modified => "M",
            GitStatus::Renamed => "R",
            GitStatus::Ignored => "!",
            GitStatus::Typechange => "T",
            GitStatus::Conflicted => "C",
        }
        .to_string()
    }

    fn get_icon(&self, status: &GitStatus) -> String {
        match status {
            GitStatus::Default => "_",
            GitStatus::Unmodified => "_", // "\u{f00c}"
            GitStatus::NewInIndex => "\u{f067}",
            GitStatus::NewInWorkdir => "?",
            GitStatus::Deleted => "\u{f014}", // or f068
            GitStatus::Modified => "\u{f8ea}",
            GitStatus::Renamed => "\u{f8ea}",
            GitStatus::Ignored => "!",
            GitStatus::Typechange => "\u{f0ec}",
            GitStatus::Conflicted => "\u{f071}",
        }
        .to_string()
    }

    fn get_unicode(&self, status: &GitStatus) -> String {
        self.get_text(status)
    }
}
