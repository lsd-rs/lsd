use ansi_term::ANSIString;
use color::{Colors, Elem};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SymLink(String);

impl<'a> From<&'a PathBuf> for SymLink {
    fn from(path: &PathBuf) -> Self {
        SymLink(
            path.to_str()
                .expect("failed to convert symlink to str")
                .to_string(),
        )
    }
}

impl SymLink {
    pub fn render(&self) -> ANSIString {
        Colors[&Elem::SymLink].paint(String::from(" ⇒ ") + &self.0)
    }
}
