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

    // On each unicode icon, add its value in a comment like "\ue5fb" (cf https://www.nerdfonts.com/cheat-sheet)
    // and then run the command below in vim:
    // s#\\u[0-9a-f]\{4}#\=eval('"'.submatch(0).'"')#
    fn get_icon(&self, status: &GitStatus) -> String {
        match status {
            GitStatus::Default => "_",
            GitStatus::Unmodified => "_",
            GitStatus::NewInIndex => "\u{f067}", // ""
            GitStatus::NewInWorkdir => "?",
            GitStatus::Deleted => "\u{f014}",  // ""
            GitStatus::Modified => "\u{f8ea}", // ""
            GitStatus::Renamed => "\u{f02b}",  // ""
            GitStatus::Ignored => "!",
            GitStatus::Typechange => "\u{f0ec}", // ""
            GitStatus::Conflicted => "\u{f071}", // ""
        }
        .to_string()
    }

    fn get_unicode(&self, status: &GitStatus) -> String {
        self.get_text(status)
    }
}

#[cfg(test)]
mod test {
    use super::Theme;
    use crate::flags::git_icons::GitIcons;
    use crate::git::GitStatus;
    use std::collections::HashMap;
    use strum::IntoEnumIterator;

    fn test_non_duplicated(icons: &GitIcons) {
        assert_eq!(
            icons.get(&GitStatus::Default),
            icons.get(&GitStatus::Unmodified)
        );
        let mut m = HashMap::new();
        for status in GitStatus::iter() {
            if status == GitStatus::Default {
                continue;
            }
            assert_eq!(m.insert(icons.get(&status), status), None);
        }
    }

    #[cfg(not(any(all(target_os = "linux", target_arch = "arm"), all(windows, target_arch = "x86", target_env = "gnu"))))]
    #[test]
    fn test_non_duplicated_noicon() {
        let icons = GitIcons::new(Theme::NoIcon);
        test_non_duplicated(&icons);
    }

    #[cfg(not(any(all(target_os = "linux", target_arch = "arm"), all(windows, target_arch = "x86", target_env = "gnu"))))]
    #[test]
    fn test_non_duplicated_unicode() {
        let icons = GitIcons::new(Theme::Unicode);
        test_non_duplicated(&icons);
    }

    #[cfg(not(any(all(target_os = "linux", target_arch = "arm"), all(windows, target_arch = "x86", target_env = "gnu"))))]
    #[test]
    fn test_non_duplicated_fancy() {
        let icons = GitIcons::new(Theme::Fancy);
        test_non_duplicated(&icons);
    }
}
