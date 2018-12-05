use color::{ColoredString, Colors, Elem};
use std::fs::read_link;
use std::path::Path;

#[derive(Debug)]
pub struct SymLink {
    target: String,
    valid: bool,
}

impl SymLink {
    pub fn from_path(path: &Path) -> Option<Self> {
        if let Ok(target) = read_link(path) {
            if target.is_absolute() || path.parent() == None {
                return Some(SymLink {
                    valid: target.exists(),
                    target: target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                });
            }

            return Some(SymLink {
                target: target
                    .to_str()
                    .expect("failed to convert symlink to str")
                    .to_string(),
                valid: path.parent().unwrap().join(target).exists(),
            });
        }

        None
    }

    pub fn render(&self, colors: &Colors) -> ColoredString {
        let elem = if self.valid {
            &Elem::SymLink
        } else {
            &Elem::BrokenSymLink
        };

        let mut res = String::from(" ⇒ ");
        res += &self.target;
        colors.colorize(res, elem)
    }
}
