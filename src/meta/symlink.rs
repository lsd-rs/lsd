use crate::color::{ColoredString, Colors, Elem};
use crate::flags::Flags;
use ansi_term::ANSIStrings;
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct SymLink {
    target: Option<String>,
    valid: bool,
}

impl<P: AsRef<Path>> From<&P> for SymLink {
    fn from(path: &P) -> Self {
        path.as_ref()
            .read_link()
            .map(|target| Self {
                target: Some(target.to_string_lossy().into()),
                valid: match (path.as_ref().parent(), target.is_absolute()) {
                    (Some(p), false) => p.join(target).exists(),
                    _ => target.exists(),
                },
            })
            .unwrap_or_default()
    }
}

impl SymLink {
    pub fn symlink_string(&self) -> Option<String> {
        self.target.as_ref().map(Into::into)
    }

    pub fn render(&self, colors: &Colors, flag: &Flags) -> ColoredString {
        if let Some(target_string) = self.symlink_string() {
            let elem = if self.valid {
                &Elem::SymLink
            } else {
                &Elem::BrokenSymLink
            };

            ANSIStrings(&[
                format!(" {} ", flag.symlink_arrow).into(), // ⇒ \u{21d2}
                colors.colorize(target_string, elem),
            ])
            .to_string()
            .into()
        } else {
            "".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymLink;
    use crate::app;
    use crate::color::{Colors, Theme};
    use crate::config_file::Config;
    use crate::flags::Flags;

    #[test]
    fn test_symlink_render_default_valid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: true,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(Theme::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(Theme::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }
}
