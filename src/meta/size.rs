use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, SizeFlag};
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
        Self { bytes: bytes }
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
        value_alignment: usize,
        unit_alignment: usize,
        flags: &Flags,
    ) -> ColoredString {
        let mut content = String::with_capacity(value_alignment + unit_alignment + 1);

        let unit = self.get_unit(flags);

        let value_str = self.render_value(&unit);
        let unit_str = Size::render_unit(&unit, &flags);

        for _ in 0..(value_alignment - value_str.len()) {
            content.push(' ');
        }

        content += &self.render_value(&unit);
        if flags.size == SizeFlag::Default {
            content.push(' ');
        }
        content += &Size::render_unit(&unit, &flags);

        for _ in 0..(unit_alignment - unit_str.len()) {
            content.push(' ');
        }

        self.paint(&unit, colors, content)
    }

    fn paint(&self, unit: &Unit, colors: &Colors, content: String) -> ColoredString {
        if unit == &Unit::None {
            colors.colorize(content, &Elem::NonFile)
        } else if unit == &Unit::Byte || unit == &Unit::Kilo {
            colors.colorize(content, &Elem::FileSmall)
        } else if unit == &Unit::Mega {
            colors.colorize(content, &Elem::FileMedium)
        } else {
            colors.colorize(content, &Elem::FileLarge)
        }
    }

    pub fn render_value(&self, unit: &Unit) -> String {
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

    pub fn render_unit(unit: &Unit, flags: &Flags) -> String {
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
