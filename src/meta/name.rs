use color::{ColoredString, Colors, Elem};
use icon;
use meta::filetype::FileType;
use std::cmp::{Ordering, PartialOrd};
use std::path::Path;

#[derive(Debug, Eq)]
pub struct Name {
    name: String,
    extension: Option<String>,
    file_type: FileType,
}

impl Name {
    pub fn new(path: &Path, file_type: FileType) -> Self {
        let mut name = path
            .file_name()
            .expect("failed to retrieve file name")
            .to_str()
            .expect("failed to encode file name")
            .to_string();

        if file_type == FileType::Directory {
            name.push('/')
        }

        let mut extension = None;
        if let Some(res) = path.extension() {
            extension = Some(
                res.to_str()
                    .expect("failed to encode file name")
                    .to_string(),
            );
        }

        Name {
            name,
            extension,
            file_type,
        }
    }

    pub fn render(&self, colors: &Colors) -> ColoredString {
        let mut content = String::with_capacity(self.name.len() + 3 /* spaces */);

        let elem = if self.file_type == FileType::Directory {
            &Elem::Dir
        } else {
            &Elem::File
        };

        content += icon::from_name(&self);
        content += "  ";
        content += &self.name;

        colors.colorize(content, elem)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn extension(&self) -> Option<String> {
        self.extension.clone()
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    pub fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Name) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Name) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Name) -> bool {
        let mut other_name = other.name.chars();

        if self.name.len() != other.name.len() {
            return false;
        }

        for c in self.name.chars() {
            if let Some(c2) = other_name.next() {
                if c != c2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
