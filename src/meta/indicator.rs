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

#[cfg(test)]
mod test {
    use super::Indicator;
    use flags::{DateFlag, Flags, SortFlag, SortOrder, WhenFlag};
    use meta::FileType;

    #[test]
    fn test_directory_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::Directory);

        assert_eq!("/", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_executable_file_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        let file_type = Indicator::from(FileType::ExecutableFile);

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
        let flags = Flags {
            display_all: true,
            display_long: true,
            display_online: true,
            display_tree: true,
            display_indicators: true,
            recursive: true,
            sort_by: SortFlag::Name,
            sort_order: SortOrder::Default,
            date: DateFlag::Relative,
            color: WhenFlag::Always,
            icon: WhenFlag::Always,
        };

        let file_type = Indicator::from(FileType::SymLink);

        assert_eq!("@", file_type.render(flags).to_string().as_str());
    }

    #[test]
    fn test_not_represented_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = true;

        // The File type doesn't have any indicator
        let file_type = Indicator::from(FileType::File);

        assert_eq!("", file_type.render(flags).to_string().as_str());
    }
}
