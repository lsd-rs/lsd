use crate::color::{ColoredString, Colors};
use crate::flags::Flags;
use crate::meta::FileType;

#[derive(Clone, Debug)]
pub struct SlashIndicator(&'static str);

impl From<FileType> for SlashIndicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory { .. } => "/",
            _ => "",
        };

        SlashIndicator(res)
    }
}

impl SlashIndicator {
    pub fn render(&self, flags: &Flags) -> ColoredString {
        if flags.slash_indicator.0 && !flags.display_indicators.0{
            ColoredString::new(Colors::default_style(), self.0.to_string())
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::SlashIndicator;
    use crate::flags::{Flags, SlashIndicator as SlashIndicatorFlag};
    use crate::meta::FileType;

    #[test]
    fn test_directory_slash_indicator() {
        let flags = Flags {
            slash_indicator: SlashIndicatorFlag(true),
            ..Default::default()
        };

        let file_type = SlashIndicator::from(FileType::Directory { uid: false });

        assert_eq!("/", file_type.render(&flags).to_string());
    }

    #[test]
    fn test_not_represented_indicators() {
    let flags = Flags {
        slash_indicator: SlashIndicatorFlag(true),
        ..Default::default()
    };

    // Test multiple non-directory file types
    let file_types = vec![
        FileType::File { exec: false, uid: false },
        FileType::Pipe,
        FileType::Socket,
        FileType::SymLink { is_dir: false },
    ];

    for file_type in file_types {
        let indicator = SlashIndicator::from(file_type);
        assert_eq!("", indicator.render(&flags).to_string());
    }
    }
    
}
