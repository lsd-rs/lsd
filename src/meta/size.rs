use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, SizeFlag};
use ansi_term::ANSIStrings;
use std::fs::Metadata;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    None,
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Size {
    bytes: u64,
}

impl<'a> From<&'a Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        let len = meta.len();
        Self { bytes: len }
    }
}

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes
    }

    pub fn get_unit(&self, flags: &Flags) -> Unit {
        if self.bytes < 1024 || flags.size == SizeFlag::Bytes {
            Unit::Byte
        } else if self.bytes < 1024 * 1024 {
            Unit::Kilo
        } else if self.bytes < 1024 * 1024 * 1024 {
            Unit::Mega
        } else if self.bytes < 1024 * 1024 * 1024 * 1024 {
            Unit::Giga
        } else {
            Unit::Tera
        }
    }

    pub fn render(
        &self,
        colors: &Colors,
        flags: &Flags,
        val_alignment: usize,
        unit_alignment: usize,
    ) -> ColoredString {
        let val_content = self.render_value(colors, flags);
        let unit_content = self.render_unit(colors, flags);

        let mut left_pad = String::with_capacity(val_alignment - val_content.len());
        for _ in 0..left_pad.capacity() {
            left_pad.push(' ');
        }

        let mut right_pad = String::with_capacity(unit_alignment - unit_content.len());
        for _ in 0..right_pad.capacity() {
            right_pad.push(' ');
        }

        let strings: &[ColoredString] = &[
            ColoredString::from(left_pad),
            val_content,
            unit_content,
            ColoredString::from(right_pad),
        ];

        let res = ANSIStrings(strings).to_string();
        ColoredString::from(res)
    }

    fn paint(&self, colors: &Colors, flags: &Flags, content: String) -> ColoredString {
        let unit = self.get_unit(flags);

        if unit == Unit::None {
            colors.colorize(content, &Elem::NonFile)
        } else if unit == Unit::Byte || unit == Unit::Kilo {
            colors.colorize(content, &Elem::FileSmall)
        } else if unit == Unit::Mega {
            colors.colorize(content, &Elem::FileMedium)
        } else {
            colors.colorize(content, &Elem::FileLarge)
        }
    }

    pub fn render_value(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.size_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn size_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match unit {
            Unit::None => "".to_string(),
            Unit::Byte => self.bytes.to_string(),
            Unit::Kilo => (((self.bytes as f64) / 1024.0 * 10.0).round() / 10.0).to_string(),
            Unit::Mega => {
                (((self.bytes as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0).to_string()
            }
            Unit::Giga => (((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0) * 10.0).round()
                / 10.0)
                .to_string(),
            Unit::Tera => {
                (((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0)
                    .to_string()
            }
        }
    }

    pub fn render_unit(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.unit_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn unit_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match flags.size {
            SizeFlag::Default => match unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("KB"),
                Unit::Mega => String::from("MB"),
                Unit::Giga => String::from("GB"),
                Unit::Tera => String::from("TB"),
            },
            SizeFlag::Short => match unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("K"),
                Unit::Mega => String::from("M"),
                Unit::Giga => String::from("G"),
                Unit::Tera => String::from("T"),
            },
            SizeFlag::Bytes => String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Size;
    use crate::flags::{Flags, SizeFlag};

    #[test]
    fn render_byte() {
        let size = Size::new(42); // == 42 bytes
        let mut flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");

        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "B");
        flags.size = SizeFlag::Short;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "B");
        flags.size = SizeFlag::Bytes;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "");
    }

    #[test]
    fn render_kilobyte() {
        let size = Size::new(42 * 1024); // 42 kilobytes
        let mut flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "K");
    }

    #[test]
    fn render_megabyte() {
        let size = Size::new(42 * 1024 * 1024); // 42 megabytes
        let mut flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "M");
    }

    #[test]
    fn render_gigabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024); // 42 gigabytes
        let mut flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "G");
    }

    #[test]
    fn render_terabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024 * 1024); // 42 terabytes
        let mut flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "T");
    }

    #[test]
    fn render_with_a_fraction() {
        let size = Size::new(42 * 1024 + 103); // 42.1 kilobytes
        let flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42.1");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "KB");
    }

    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::new(42 * 1024 + 1); // 42.001 kilobytes == 42 kilobytes
        let flags = Flags::default();
        let unit = size.get_unit(&flags);

        assert_eq!(size.render_value(&unit).as_str(), "42");
        assert_eq!(Size::render_unit(&unit, &flags).as_str(), "KB");
    }
}
