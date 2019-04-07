use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, SizeFlag};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    None,
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    value: u64,
    unit: Unit,
}

impl<'a> From<&'a Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        let len = meta.len();

        if meta.is_file() {
            Self::from_bytes(len)
        } else {
            Self {
                value: 0,
                unit: Unit::None,
            }
        }
    }
}

impl Size {
    fn from_bytes(len: u64) -> Self {
        if len < 1024 {
            Self {
                value: len * 1024,
                unit: Unit::Byte,
            }
        } else if len < 1024 * 1024 {
            Self {
                value: len,
                unit: Unit::Kilo,
            }
        } else if len < 1024 * 1024 * 1024 {
            Self {
                value: len / 1024,
                unit: Unit::Mega,
            }
        } else if len < 1024 * 1024 * 1024 * 1024 {
            Self {
                value: len / (1024 * 1024),
                unit: Unit::Giga,
            }
        } else {
            Self {
                value: len / (1024 * 1024 * 1024),
                unit: Unit::Tera,
            }
        }
    }

    pub fn render(
        &self,
        colors: &Colors,
        value_alignment: usize,
        unit_alignment: usize,
        flags: Flags,
    ) -> ColoredString {
        let mut content = String::with_capacity(value_alignment + unit_alignment + 1);

        let value = self.render_value();
        let unit = self.render_unit(flags);

        for _ in 0..(value_alignment - value.len()) {
            content.push(' ');
        }

        content += &self.render_value();
        if flags.size != SizeFlag::Short {
            content.push(' ');
        }
        content += &self.render_unit(flags);

        for _ in 0..(unit_alignment - unit.len()) {
            content.push(' ');
        }

        self.paint(colors, content)
    }

    fn paint(&self, colors: &Colors, content: String) -> ColoredString {
        if self.unit == Unit::None {
            colors.colorize(content, &Elem::NonFile)
        } else if self.unit == Unit::Byte || self.unit == Unit::Kilo {
            colors.colorize(content, &Elem::FileSmall)
        } else if self.unit == Unit::Mega {
            colors.colorize(content, &Elem::FileMedium)
        } else {
            colors.colorize(content, &Elem::FileLarge)
        }
    }

    pub fn render_value(&self) -> String {
        let size_str = match self.unit {
            Unit::None => "".to_string(),
            _ => (self.value as f64 / 1024.0).to_string(),
        };

        // Check if there is a fraction.
        if let Some(fraction_idx) = size_str.find('.') {
            // If the fraction start with 0 (like 32.01), the result is rounded
            // by removing the fraction.
            if size_str.chars().nth(fraction_idx + 1) == Some('0') {
                let (res, _) = size_str.split_at(fraction_idx); // Split before the fraction
                res.to_string()
            } else {
                //
                let (res, _) = size_str.split_at(fraction_idx + 2); // Split after the '.' and the first fraction digit.
                res.to_string()
            }
        } else {
            size_str
        }
    }

    pub fn render_unit(&self, flags: Flags) -> String {
        match flags.size {
            SizeFlag::Default => match self.unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("KB"),
                Unit::Mega => String::from("MB"),
                Unit::Giga => String::from("GB"),
                Unit::Tera => String::from("TB"),
            },
            SizeFlag::Short => match self.unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("K"),
                Unit::Mega => String::from("M"),
                Unit::Giga => String::from("G"),
                Unit::Tera => String::from("T"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Size;
    use crate::flags::{Flags, SizeFlag};

    #[test]
    fn render_byte() {
        let size = Size::from_bytes(42); // == 42 bytes
        let mut flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");

        assert_eq!(size.render_unit(flags).as_str(), "B");
        flags.size = SizeFlag::Short;
        assert_eq!(size.render_unit(flags).as_str(), "B");
    }

    #[test]
    fn render_kilobyte() {
        let size = Size::from_bytes(42 * 1024); // 42 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit(flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.render_unit(flags).as_str(), "K");
    }

    #[test]
    fn render_megabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024); // 42 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit(flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.render_unit(flags).as_str(), "M");
    }

    #[test]
    fn render_gigabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024 * 1024); // 42 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit(flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.render_unit(flags).as_str(), "G");
    }

    #[test]
    fn render_terabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024 * 1024 * 1024); // 42 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit(flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.render_unit(flags).as_str(), "T");
    }

    #[test]
    fn render_with_a_fraction() {
        let size = Size::from_bytes(42 * 1024 + 103); // 42.1 kilobytes
        let flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42.1");
        assert_eq!(size.render_unit(flags).as_str(), "KB");
    }

    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::from_bytes(42 * 1024 + 1); // 42.001 kilobytes == 42 kilobytes
        let flags = Flags::default();

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit(flags).as_str(), "KB");
    }
}
