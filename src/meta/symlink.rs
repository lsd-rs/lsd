use ansi_term::ANSIString;
use color::{Colors, Elem};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SymLink {
    target: String,
    valid: bool,
}

impl<'a> From<&'a PathBuf> for SymLink {
    fn from(target: &PathBuf) -> Self {
        SymLink {
            valid: target.exists(),
            target: target
                .to_str()
                .expect("failed to convert symlink to str")
                .to_string(),
        }
    }
}

impl SymLink {
    pub fn render(&self) -> ANSIString {
        let color = if self.valid {
            Colors[&Elem::SymLink]
        } else {
            Colors[&Elem::BrokenSymLink]
        };

        color.paint(String::from(" ⇒ ") + &self.target)
    }
}
