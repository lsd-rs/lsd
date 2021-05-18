use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{DateFlag, Flags};
use chrono::{DateTime, Duration, Local};
use chrono_humanize::HumanTime;
use std::{fs::Metadata, time::SystemTime};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(SystemTime);

impl<'a> From<&'a Metadata> for Date {
    fn from(meta: &'a Metadata) -> Self {
        Date(meta.modified().expect("failed to retrieve modified date"))
    }
}

impl Date {
    pub fn render(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let time: DateTime<Local> = self.0.into();
        let ago = Local::now() - time;

        let elem = if ago < Duration::hours(1) {
            Elem::HourOld
        } else if ago < Duration::days(1) {
            Elem::DayOld
        } else {
            Elem::Older
        };

        let date_string = match &flags.date {
            DateFlag::Date => time.format("%c").to_string(),
            DateFlag::Relative => format!("{}", HumanTime::from(-ago)),
            DateFlag::ISO => {
                // 15778476 seconds is 6 months
                if ago < Duration::seconds(15_778_476) {
                    time.format("%m-%d %R").to_string()
                } else {
                    time.format("%F").to_string()
                }
            }
            DateFlag::Formatted(format) => time.format(&format).to_string(),
        };

        colors.colorize(date_string, &elem)
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use crate::color::{Colors, Theme};
    use crate::flags::{DateFlag, Flags};
    use ansi_term::Colour;
    use chrono::{DateTime, Duration, Local};
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};
    use std::{env, fs};

    #[cfg(unix)]
    fn cross_platform_touch(path: &Path, date: &DateTime<Local>) -> io::Result<ExitStatus> {
        Command::new("touch")
            .arg("-t")
            .arg(date.format("%Y%m%d%H%M.%S").to_string())
            .arg(&path)
            .status()
    }

    #[cfg(windows)]
    fn cross_platform_touch(path: &Path, date: &DateTime<Local>) -> io::Result<ExitStatus> {
        use std::process::Stdio;

        let copy_success = Command::new("cmd")
            .arg("/C")
            .arg("copy")
            .arg("NUL")
            .arg(path)
            .stdout(Stdio::null()) // Windows doesn't have a quiet flag
            .status()?
            .success();

        assert!(copy_success, "failed to create empty file");

        Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                r#"$(Get-Item {}).lastwritetime=$(Get-Date "{}")"#,
                path.display(),
                date.to_rfc3339()
            ))
            .status()
    }

    #[test]
    fn test_an_hour_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_an_hour_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::seconds(4);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(40).paint(creation_date.format("%c").to_string()),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_day_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_day_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::hours(4);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(42).paint(creation_date.format("%c").to_string()),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_several_days_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_several_days_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(36).paint(creation_date.format("%c").to_string()),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date.tmp");

        let creation_date = Local::now() - chrono::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(
            Colour::Fixed(36).paint("2 days ago"),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date_now() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date_now.tmp");

        let creation_date = Local::now();
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(Colour::Fixed(40).paint("now"), date.render(&colors, &flags));

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_iso_format_now() {
        let mut file_path = env::temp_dir();
        file_path.push("test_iso_format_now.tmp");

        let creation_date = Local::now();
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::ISO;

        assert_eq!(
            Colour::Fixed(40).paint(creation_date.format("%m-%d %R").to_string()),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_iso_format_year_old() {
        let mut file_path = env::temp_dir();
        file_path.push("test_iso_format_year_old.tmp");

        let creation_date = Local::now() - Duration::days(400);
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::ISO;

        assert_eq!(
            Colour::Fixed(36).paint(creation_date.format("%F").to_string()),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }
}
