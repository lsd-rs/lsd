use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, SizeFlag};
use std::fs::Metadata;

const KIB: u64 = 1024;
const MIB: u64 = 1024_u64.pow(2);
const GIB: u64 = 1024_u64.pow(3);
const TIB: u64 = 1024_u64.pow(4);
const KB: u64 = 1000;
const MB: u64 = 1000_u64.pow(2);
const GB: u64 = 1000_u64.pow(3);
const TB: u64 = 1000_u64.pow(4);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
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

impl From<&Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        Self { bytes: meta.len() }
    }
}

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes
    }

    fn format_size(&self, number: f64) -> String {
        format!("{0:.1$}", number, if number < 10.0 { 1 } else { 0 })
    }

    fn get_unit(&self, flags: &Flags) -> Unit {
        if flags.size == SizeFlag::Bytes {
            return Unit::Byte;
        }

        match self.bytes {
            b if b < KIB => Unit::Byte,
            b if b < MIB => Unit::Kilo,
            b if b < GIB => Unit::Mega,
            b if b < TIB => Unit::Giga,
            _ => Unit::Tera,
        }
    }

    pub fn render(
        &self,
        colors: &Colors,
        flags: &Flags,
        val_alignment: Option<usize>,
    ) -> ColoredString {
        let val_content = self.render_value(colors, flags);
        let unit_content = self.render_unit(colors, flags);

        let left_pad = if let Some(align) = val_alignment {
            " ".repeat(align - val_content.content().len())
        } else {
            "".to_string()
        };

        let mut strings: Vec<ColoredString> = vec![
            ColoredString::new(Colors::default_style(), left_pad),
            val_content,
        ];
        if flags.size != SizeFlag::Short {
            strings.push(ColoredString::new(Colors::default_style(), " ".into()));
        }
        strings.push(unit_content);

        let res = strings
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        ColoredString::new(Colors::default_style(), res)
    }

    fn paint(&self, colors: &Colors, flags: &Flags, content: String) -> ColoredString {
        let unit = self.get_unit(flags);
        let elem = match unit {
            Unit::Byte | Unit::Kilo => &Elem::FileSmall,
            Unit::Mega => &Elem::FileMedium,
            _ => &Elem::FileLarge,
        };

        colors.colorize(content, elem)
    }

    pub fn render_value(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.value_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn value_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match flags.size {
            SizeFlag::Si => match unit {
                Unit::Byte => self.bytes.to_string(),
                Unit::Kilo => {
                    self.format_size(((self.bytes as f64 / KB as f64) * 10.0).round() / 10.0)
                }
                Unit::Mega => {
                    self.format_size(((self.bytes as f64 / MB as f64) * 10.0).round() / 10.0)
                }
                Unit::Giga => {
                    self.format_size(((self.bytes as f64 / GB as f64) * 10.0).round() / 10.0)
                }
                Unit::Tera => {
                    self.format_size(((self.bytes as f64 / TB as f64) * 10.0).round() / 10.0)
                }
            },
            _ => match unit {
                Unit::Byte => self.bytes.to_string(),
                Unit::Kilo => {
                    self.format_size(((self.bytes as f64 / KIB as f64) * 10.0).round() / 10.0)
                }
                Unit::Mega => {
                    self.format_size(((self.bytes as f64 / MIB as f64) * 10.0).round() / 10.0)
                }
                Unit::Giga => {
                    self.format_size(((self.bytes as f64 / GIB as f64) * 10.0).round() / 10.0)
                }
                Unit::Tera => {
                    self.format_size(((self.bytes as f64 / TIB as f64) * 10.0).round() / 10.0)
                }
            },
        }
    }

    pub fn render_unit(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.unit_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn unit_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match flags.size {
            SizeFlag::Short => match unit {
                Unit::Byte => String::from('B'),
                Unit::Kilo => String::from('K'),
                Unit::Mega => String::from('M'),
                Unit::Giga => String::from('G'),
                Unit::Tera => String::from('T'),
            },
            SizeFlag::Iec => match unit {
                Unit::Byte => String::from('B'),
                Unit::Kilo => String::from("KiB"),
                Unit::Mega => String::from("MiB"),
                Unit::Giga => String::from("GiB"),
                Unit::Tera => String::from("TiB"),
            },
            SizeFlag::Bytes => String::from(""),
            _ => match unit {
                Unit::Byte => String::from('B'),
                Unit::Kilo => String::from("KB"),
                Unit::Mega => String::from("MB"),
                Unit::Giga => String::from("GB"),
                Unit::Tera => String::from("TB"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Size, GIB, KIB, MIB, TIB};
    use crate::color::{Colors, ThemeOption};
    use crate::flags::{Flags, SizeFlag};

    #[test]
    fn render_byte() {
        let size = Size::new(42); // == 42 bytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");

        assert_eq!(size.unit_string(&flags), "B");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "B");
        flags.size = SizeFlag::Bytes;
        assert_eq!(size.unit_string(&flags), "");
    }

    #[test]
    fn render_10_minus_kilobyte() {
        let size = Size::new(4 * KIB); // 4 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "4.0");
        assert_eq!(size.unit_string(&flags), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "K");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "KiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "KB");
    }

    #[test]
    fn render_kilobyte() {
        let size = Size::new(42 * KIB); // 42 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "K");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "KiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "KB");
    }

    #[test]
    fn render_100_plus_kilobyte() {
        let size = Size::new(420 * KIB + 420); // 420.4 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "420");
        assert_eq!(size.unit_string(&flags), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "K");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "KiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "KB");
    }

    #[test]
    fn render_10_minus_megabyte() {
        let size = Size::new(4 * MIB); // 4 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "4.0");
        assert_eq!(size.unit_string(&flags), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "M");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "MiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "MB");
    }

    #[test]
    fn render_megabyte() {
        let size = Size::new(42 * MIB); // 42 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "M");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "MiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "MB");
    }

    #[test]
    fn render_100_plus_megabyte() {
        let size = Size::new(420 * MIB + 420 * KIB); // 420.4 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "420");
        assert_eq!(size.unit_string(&flags), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "M");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "MiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "MB");
    }

    #[test]
    fn render_10_minus_gigabyte() {
        let size = Size::new(4 * GIB); // 4 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "4.0");
        assert_eq!(size.unit_string(&flags), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "G");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "GiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "GB");
    }

    #[test]
    fn render_gigabyte() {
        let size = Size::new(42 * GIB); // 42 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "G");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "GiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "GB");
    }

    #[test]
    fn render_100_plus_gigabyte() {
        let size = Size::new(420 * GIB + 420 * MIB); // 420.4 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "420");
        assert_eq!(size.unit_string(&flags), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "G");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "GiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "GB");
    }

    #[test]
    fn render_10_minus_terabyte() {
        let size = Size::new(4 * TIB); // 4 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "4.0");
        assert_eq!(size.unit_string(&flags), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "T");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "TiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "TB");
    }

    #[test]
    fn render_terabyte() {
        let size = Size::new(42 * TIB); // 42 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "T");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "TiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "TB");
    }

    #[test]
    fn render_100_plus_terabyte() {
        let size = Size::new(420 * TIB + 420 * GIB); // 420.4 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags), "420");
        assert_eq!(size.unit_string(&flags), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags), "T");
        flags.size = SizeFlag::Iec;
        assert_eq!(size.unit_string(&flags), "TiB");
        flags.size = SizeFlag::Si;
        assert_eq!(size.unit_string(&flags), "TB");
    }

    #[test]
    fn render_with_a_fraction() {
        let size = Size::new(42 * KIB + 103); // 42.1 kilobytes
        let flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "KB");
    }

    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::new(42 * KIB + 1); // 42.001 kilobytes == 42 kilobytes
        let flags = Flags::default();

        assert_eq!(size.value_string(&flags), "42");
        assert_eq!(size.unit_string(&flags), "KB");
    }

    #[test]
    fn render_short_nospaces() {
        let size = Size::new(42 * KIB); // 42 kilobytes
        let flags = Flags {
            size: SizeFlag::Short,
            ..Default::default()
        };
        let colors = Colors::new(ThemeOption::NoColor);

        assert_eq!(size.render(&colors, &flags, Some(2)).to_string(), "42K");
        assert_eq!(size.render(&colors, &flags, Some(3)).to_string(), " 42K");
    }
}
