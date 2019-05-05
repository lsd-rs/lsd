use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{DateFlag, Flags};
use chrono_humanize::HumanTime;
use std::fs::Metadata;
use std::time::UNIX_EPOCH;
use time::{Duration, Timespec};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(time::Tm);

impl<'a> From<&'a Metadata> for Date {
    fn from(meta: &'a Metadata) -> Self {
        let modified_time = meta.modified().expect("failed to retrieve modified date");

        let modified_time_since_epoch =
            modified_time.duration_since(UNIX_EPOCH).unwrap_or_default();

        let time = time::at(Timespec::new(
            modified_time_since_epoch.as_secs() as i64,
            modified_time_since_epoch.subsec_nanos() as i32,
        ));

        Date(time)
    }
}

impl Date {
    pub fn render(&self, colors: &Colors, date_alignment: usize, flags: Flags) -> ColoredString {
        let mut content = String::with_capacity(date_alignment + 1);
        let now = time::now();

        let elem;
        if self.0 > now - Duration::hours(1) {
            elem = &Elem::HourOld;
        } else if self.0 > now - Duration::days(1) {
            elem = &Elem::DayOld;
        } else {
            elem = &Elem::Older;
        }

        let date_string = &self.date_string(flags);
        content += date_string;

        for _ in 0..(date_alignment - date_string.len()) {
            content.push(' ');
        }
        colors.colorize(content, elem)
    }

    pub fn date_string(&self, flags: Flags) -> String {
        match flags.date {
            DateFlag::Date => self.0.ctime().to_string(),
            DateFlag::Relative => format!("{}", HumanTime::from(self.0 - time::now())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use crate::color::{Colors, Theme};
    use crate::flags::{DateFlag, Flags};
    use ansi_term::Colour;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};
    use std::{env, fs};
    use time;

    #[cfg(unix)]
    fn cross_platform_touch(path: &Path, date: &time::Tm) -> io::Result<ExitStatus> {
        Command::new("touch")
            .arg("-t")
            .arg(date.strftime("%Y%m%d%H%M.%S").unwrap().to_string())
            .arg(&path)
            .status()
    }

    #[cfg(windows)]
    fn cross_platform_touch(path: &Path, date: &time::Tm) -> io::Result<ExitStatus> {
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
            .arg("$(Get-Item")
            .arg(path)
            .arg(").lastwritetime=$(Get-Date \"")
            .arg(date.strftime("%m/%d/%Y %H:%M:%S").unwrap().to_string())
            .arg("\")")
            .status()
    }

    #[test]
    fn test_an_hour_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_an_hour_old_file_color.tmp");

        let creation_date = (time::now() - time::Duration::seconds(4)).to_local();

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(40).paint(creation_date.ctime().to_string()),
            date.render(&colors, creation_date.ctime().to_string().len(), flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_day_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_day_old_file_color.tmp");

        let creation_date = (time::now() - time::Duration::hours(4)).to_local();

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(42).paint(creation_date.ctime().to_string()),
            date.render(&colors, creation_date.ctime().to_string().len(), flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_several_days_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_several_days_old_file_color.tmp");

        let creation_date = time::now_utc() - time::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date.to_local())
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            Colour::Fixed(36).paint(creation_date.ctime().to_string()),
            date.render(&colors, creation_date.ctime().to_string().len(), flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date.tmp");

        let creation_date = time::now() - time::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date.to_local())
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(
            Colour::Fixed(36).paint("2 days ago  "),
            date.render(&colors, 12, flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date_now() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date_now.tmp");

        let creation_date = time::now();

        let success = Command::new("touch")
            .arg("-t")
            .arg(
                creation_date
                    .to_local()
                    .strftime("%Y%m%d%H%M.%S")
                    .unwrap()
                    .to_string(),
            )
            .arg(&file_path)
            .status()
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(
            Colour::Fixed(40).paint("now  "),
            date.render(&colors, 5, flags)
        );

        fs::remove_file(file_path).unwrap();
    }
}
