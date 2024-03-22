use crate::color::{ColoredString, Colors};
use crate::flags::Flags;
use crate::flags::IndicatorStyle;
use crate::meta::FileType;

#[derive(Clone, Debug)]
pub struct Indicator(&'static str);

impl From<FileType> for Indicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory { .. } => "/",
            FileType::File { exec: true, .. } => "*",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::SymLink { .. } => "@",
            _ => "",
        };

        Indicator(res)
    }
}

impl Indicator {
    pub fn render(&self, flags: &Flags) -> ColoredString {
        let ind = if flags.display_indicators.0 {
            match (flags.indicator_style, self.0) {
                (IndicatorStyle::Classify, c) => c,
                (IndicatorStyle::FileType, c) if c != "*" => c,
                (IndicatorStyle::Slash, c) if c == "/" => c,
                _ => "",
            }
        } else {
            ""
        };
        ColoredString::new(Colors::default_style(), ind.into())
    }
}

#[cfg(test)]
mod test {
    use super::Indicator;
    use crate::flags::{Flags, IndicatorStyle, Indicators};
    use crate::meta::FileType;

    #[test]
    fn test_directory_indicator() {
        let flags = Flags {
            display_indicators: Indicators(true),
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::Directory { uid: false });

        assert_eq!("/", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_executable_file_indicator() {
        let flags = Flags {
            display_indicators: Indicators(true),
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::File {
            uid: false,
            exec: true,
        });

        assert_eq!("*", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_executable_file_indicator_slash() {
        let flags = Flags {
            display_indicators: Indicators(true),
            indicator_style: IndicatorStyle::Slash,
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::File {
            uid: false,
            exec: true,
        });

        assert_eq!("", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_socket_indicator() {
        let flags = Flags {
            display_indicators: Indicators(true),
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::Socket);

        assert_eq!("=", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_socket_indicator_slash() {
        let flags = Flags {
            display_indicators: Indicators(true),
            indicator_style: IndicatorStyle::Slash,
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::Socket);

        assert_eq!("", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_symlink_indicator() {
        let flags = Flags {
            display_indicators: Indicators(true),
            ..Default::default()
        };

        let file_type = Indicator::from(FileType::SymLink { is_dir: false });
        assert_eq!("@", file_type.render(&flags).to_string());

        let file_type = Indicator::from(FileType::SymLink { is_dir: true });
        assert_eq!("@", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_not_represented_indicator() {
        let flags = Flags {
            display_indicators: Indicators(true),
            ..Default::default()
        };

        // The File type doesn't have any indicator
        let file_type = Indicator::from(FileType::File {
            exec: false,
            uid: false,
        });

        assert_eq!("", file_type.render(&flags).to_string());
    }
}
