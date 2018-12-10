use ansi_term::ANSIString;
use color::ColoredString;
use flags::Flags;
use meta::FileType;

#[derive(Debug)]
pub struct Indicator(&'static str);

impl From<FileType> for Indicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory => "/",
            FileType::ExecutableFile => "*",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::SymLink => "@",
            _ => "",
        };

        Indicator(res)
    }
}

impl Indicator {
    pub fn render(&self, flags: Flags) -> ColoredString {
        if flags.display_indicators {
            ANSIString::from(self.0)
        } else {
            ANSIString::from("")
        }
    }
}
