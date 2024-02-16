use crate::{
    color::{ColoredString, Colors, Elem},
    flags::Flags,
};

use std::os::windows::fs::MetadataExt;

#[derive(Debug, Clone)]
pub struct WindowsAttributes {
    pub archive: bool,
    pub readonly: bool,
    pub hidden: bool,
    pub system: bool,
}

pub fn get_attributes(metadata: &std::fs::Metadata) -> WindowsAttributes {
    use windows::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_READONLY,
        FILE_ATTRIBUTE_SYSTEM, FILE_FLAGS_AND_ATTRIBUTES,
    };

    let bits = metadata.file_attributes();
    let has_bit = |bit: FILE_FLAGS_AND_ATTRIBUTES| bits & bit.0 == bit.0;

    // https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
    WindowsAttributes {
        archive: has_bit(FILE_ATTRIBUTE_ARCHIVE),
        readonly: has_bit(FILE_ATTRIBUTE_READONLY),
        hidden: has_bit(FILE_ATTRIBUTE_HIDDEN),
        system: has_bit(FILE_ATTRIBUTE_SYSTEM),
    }
}

impl WindowsAttributes {
    pub fn render(&self, colors: &Colors, _flags: &Flags) -> ColoredString {
        let res = [
            match self.archive {
                true => colors.colorize("a", &Elem::Archive),
                false => colors.colorize('-', &Elem::NoAccess),
            },
            match self.readonly {
                true => colors.colorize("r", &Elem::AttributeRead),
                false => colors.colorize('-', &Elem::NoAccess),
            },
            match self.hidden {
                true => colors.colorize("h", &Elem::Hidden),
                false => colors.colorize('-', &Elem::NoAccess),
            },
            match self.system {
                true => colors.colorize("s", &Elem::System),
                false => colors.colorize('-', &Elem::NoAccess),
            },
        ]
        .into_iter()
        .fold(String::with_capacity(4), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        });
        ColoredString::new(Colors::default_style(), res)
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write;
    use std::process::Command;

    use crate::{
        color::{Colors, ThemeOption},
        flags::Flags,
    };

    use super::get_attributes;
    use tempfile::tempdir;

    #[test]
    pub fn archived_file() {
        let attribute_string = create_and_process_file_with_attributes("archived_file.txt", "+A");
        assert_eq!("a---", attribute_string);
    }

    #[test]
    pub fn readonly_file() {
        let attribute_string = create_and_process_file_with_attributes("readonly_file.txt", "+R");
        assert_eq!("ar--", attribute_string);
    }

    #[test]
    pub fn hidden_file() {
        let attribute_string = create_and_process_file_with_attributes("hidden_file.txt", "+H");
        assert_eq!("a-h-", attribute_string);
    }

    #[test]
    pub fn system_file() {
        let attribute_string = create_and_process_file_with_attributes("system_file.txt", "+S");
        assert_eq!("a--s", attribute_string);
    }

    fn create_and_process_file_with_attributes(name: &str, attrs: &str) -> String {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path = tmp_dir.path().join(name);
        let mut file = fs::File::create(path.clone()).unwrap();
        writeln!(file, "Test content").unwrap();
        Command::new("attrib")
            .arg(attrs)
            .arg(&path)
            .output()
            .expect("able to set attributes");
        let metadata = file.metadata().expect("able to get metadata");

        let colors = Colors::new(ThemeOption::NoColor);

        let attributes = get_attributes(&metadata);
        attributes
            .render(&colors, &Flags::default())
            .content()
            .to_string()
    }
}
