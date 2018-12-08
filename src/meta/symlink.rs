use ansi_term::{ANSIString, ANSIStrings};
use color::{ColoredString, Colors, Elem};
use std::fs::read_link;
use std::path::Path;

#[derive(Debug)]
pub struct SymLink {
    target: Option<String>,
    valid: bool,
}

impl<'a> From<&'a Path> for SymLink {
    fn from(path: &'a Path) -> Self {
        if let Ok(target) = read_link(path) {
            if target.is_absolute() || path.parent() == None {
                return SymLink {
                    valid: target.exists(),
                    target: Some(
                        target
                            .to_str()
                            .expect("failed to convert symlink to str")
                            .to_string(),
                    ),
                };
            }

            return SymLink {
                target: Some(
                    target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                ),
                valid: path.parent().unwrap().join(target).exists(),
            };
        }

        return SymLink {
            target: None,
            valid: false,
        };
    }
}

impl SymLink {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        if let Some(ref target) = self.target {
            let elem = if self.valid {
                &Elem::SymLink
            } else {
                &Elem::BrokenSymLink
            };

            let strings: &[ColoredString] = &[
                ColoredString::from(" ⇒ "),
                colors.colorize(target.to_string(), elem),
            ];

            let res = ANSIStrings(strings).to_string();
            ColoredString::from(res)
        } else {
            ANSIString::from("")
        }
    }
}
