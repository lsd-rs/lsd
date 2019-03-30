use crate::color::ColoredString;
use crate::flags::Flags;
use crate::meta::FileType;
use ansi_term::ANSIString;

#[derive(Clone, Debug)]
pub struct Indicator(&'static str);

impl From<FileType> for Indicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory { .. } => "/",
            FileType::File { exec: true, .. } => "*",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::SymLink => "@",
            _ => "",
        };

        Indicator(res)
    }
}

impl Indicator {
    pub fn render(&self, flags: &Flags) -> ColoredString {
        if flags.display_indicators {
            ANSIString::from(self.0)
        } else {
            ANSIString::from("")
        }
    }
}

#[cfg(test)]
mod test {
    use super::Indicator;
    use crate::flags::Flags;
    use crate::meta::FileType;

    #[test]
    fn test_directory_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::Directory { uid: false });

        assert_eq!("/", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_executable_file_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::File {
            uid: false,
            exec: true,
        });

        assert_eq!("*", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_socket_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::Socket);

        assert_eq!("=", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_symlink_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::SymLink);

        assert_eq!("@", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_not_represented_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        // The File type doesn't have any indicator
        let file_type = Indicator::from(FileType::File {
            exec: false,
            uid: false,
        });

        assert_eq!("", file_type.render(flags).to_string().as_str());
    }
}
