use ansi_term::ANSIString;
use color::{Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    value: usize,
    unit: Unit,
}

impl<'a> From<&'a Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        let len = meta.len() as usize;

        Size::from_bytes(len)
    }
}

impl Size {
    fn from_bytes(len: usize) -> Self {
        if len < 1024 {
            Size {
                value: len * 1024,
                unit: Unit::Byte,
            }
        } else if len < 1024 * 1024 {
            Size {
                value: len,
                unit: Unit::Kilo,
            }
        } else if len < 1024 * 1024 * 1024 {
            Size {
                value: len / 1024,
                unit: Unit::Mega,
            }
        } else if len < 1024 * 1024 * 1024 * 1024 {
            Size {
                value: len / (1024 * 1024),
                unit: Unit::Giga,
            }
        } else {
            Size {
                value: len / (1024 * 1024 * 1024),
                unit: Unit::Tera,
            }
        }
    }

    pub fn render(&self, value_alignment: usize, unit_alignment: usize) -> ANSIString {
        let mut content = String::with_capacity(value_alignment + unit_alignment + 1);

        let value = self.render_value();
        let unit = self.render_unit();

        for _ in 0..(value_alignment - value.len()) {
            content.push(' ');
        }

        content += &self.render_value();
        content.push(' ');
        content += &self.render_unit();

        for _ in 0..(unit_alignment - unit.len()) {
            content.push(' ');
        }

        self.paint(content)
    }

    fn paint(&self, content: String) -> ANSIString {
        if self.unit == Unit::Byte || self.unit == Unit::Kilo {
            Colors[&Elem::FileSmall].paint(content)
        } else if self.unit == Unit::Mega {
            Colors[&Elem::FileMedium].paint(content)
        } else {
            Colors[&Elem::FileLarge].paint(content)
        }
    }

    pub fn render_value(&self) -> String {
        let size_str = (self.value as f32 / 1024.0).to_string();

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

    pub fn render_unit(&self) -> String {
        match self.unit {
            Unit::Byte => String::from("B"),
            Unit::Kilo => String::from("KB"),
            Unit::Mega => String::from("MB"),
            Unit::Giga => String::from("GB"),
            Unit::Tera => String::from("TB"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn render_byte() {
        let size = Size::from_bytes(42); // == 42 bytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "B");
    }

    #[test]
    fn render_kilobyte() {
        let size = Size::from_bytes(42 * 1024); // 42 kilobytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "KB");
    }

    #[test]
    fn render_megabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024); // 42 megabytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "MB");
    }

    #[test]
    fn render_gigabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024 * 1024); // 42 gigabytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "GB");
    }

    #[test]
    fn render_terabyte() {
        let size = Size::from_bytes(42 * 1024 * 1024 * 1024 * 1024); // 42 terabytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "TB");
    }

    #[test]
    fn render_with_a_fraction() {
        let size = Size::from_bytes(42 * 1024 + 103); // 42.1 kilobytes

        assert_eq!(size.render_value().as_str(), "42.1");
        assert_eq!(size.render_unit().as_str(), "KB");
    }

    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::from_bytes(42 * 1024 + 1); // 42.001 kilobytes == 42 kilobytes

        assert_eq!(size.render_value().as_str(), "42");
        assert_eq!(size.render_unit().as_str(), "KB");
    }
}
