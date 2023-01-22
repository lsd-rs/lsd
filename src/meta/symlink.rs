use crate::color::{ColoredString, Colors, Elem};
use crate::flags::Flags;
use crate::meta::{FileType, Permissions};
use std::fs::read_link;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SymLink {
    target: Option<String>,
    target_type: Option<FileType>,
    valid: bool,
}

impl From<&Path> for SymLink {
    fn from(path: &Path) -> Self {
        if let Ok(target) = read_link(path) {
            // Extract the symlink's target type if possible, so it can have proper colors.
            let target_type = match target.metadata() {
                Ok(metadata) => Some(FileType::new(
                    &metadata,
                    None,
                    &Permissions::from(&metadata),
                )),
                Err(_) => None,
            };

            if target.is_absolute() || path.parent() == None {
                return Self {
                    valid: target.exists(),
                    target: Some(
                        target
                            .to_str()
                            .expect("failed to convert symlink to str")
                            .to_string(),
                    ),
                    target_type,
                };
            }

            return Self {
                target: Some(
                    target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                ),
                target_type,
                valid: path.parent().unwrap().join(target).exists(),
            };
        }

        Self {
            target: None,
            target_type: None,
            valid: false,
        }
    }
}

impl SymLink {
    pub fn symlink_string(&self) -> Option<String> {
        self.target.as_ref().map(|target| target.to_string())
    }

    pub fn render(&self, colors: &Colors, flag: &Flags) -> ColoredString {
        if let Some(target_string) = self.symlink_string() {
            let elem = if self.valid {
                // Proper colors for symlink target file types.
                match self.target_type {
                    Some(FileType::BlockDevice) => &Elem::BlockDevice,
                    Some(FileType::CharDevice) => &Elem::CharDevice,
                    Some(FileType::Directory { uid: _ }) => &Elem::Dir { uid: false },
                    Some(FileType::File { uid: _, exec: _ }) => &Elem::File {
                        uid: false,
                        exec: false,
                    },
                    Some(FileType::Pipe) => &Elem::Pipe,
                    Some(FileType::Socket) => &Elem::Socket,
                    Some(FileType::Special) => &Elem::Special,
                    _ => &Elem::SymLink,
                }
            } else {
                &Elem::MissingSymLinkTarget
            };

            let strings: &[ColoredString] = &[
                ColoredString::new(Colors::default_style(), format!(" {} ", flag.symlink_arrow)), // ⇒ \u{21d2}
                colors.colorize(target_string, elem),
            ];

            let res = strings
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("");
            ColoredString::new(Colors::default_style(), res)
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymLink;
    use crate::app;
    use crate::color::{Colors, ThemeOption};
    use crate::config_file::Config;
    use crate::flags::Flags;
    use crate::meta::FileType;

    #[test]
    fn test_symlink_render_default_valid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            target_type: None,
            valid: true,
        };
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(ThemeOption::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            target_type: None,
            valid: false,
        };
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(ThemeOption::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_withcolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            target_type: Some(FileType::SymLink { is_dir: false }),
            valid: false,
        };
        let argv = ["lsd"];
        let matches = app::build().try_get_matches_from(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ \u{1b}[38;5;124m/target\u{1b}[39m"),
            link.render(
                &Colors::new(ThemeOption::NoLscolors),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }
}
