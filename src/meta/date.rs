use color::{ColoredString, Colors, Elem};
use std::fs::Metadata;
use std::time::UNIX_EPOCH;
use time::{Duration, Timespec};

#[derive(Debug)]
pub struct Date(time::Tm);

impl<'a> From<&'a Metadata> for Date {
    fn from(meta: &'a Metadata) -> Self {
        let modified_time = meta.modified().expect("failed to retrieve modified date");

        let modified_time_since_epoch = modified_time
            .duration_since(UNIX_EPOCH)
            .expect("failed to convert modified time to timestamp");

        let time = time::at(Timespec::new(
            modified_time_since_epoch.as_secs() as i64,
            modified_time_since_epoch.subsec_nanos() as i32,
        ));

        Date(time)
    }
}

impl Date {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        let now = time::now();

        let elem;
        if self.0 > now - Duration::hours(1) {
            elem = &Elem::HourOld;
        } else if self.0 > now - Duration::days(1) {
            elem = &Elem::DayOld;
        } else {
            elem = &Elem::Older;
        }

        colors.colorize(self.0.ctime().to_string(), elem)
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use ansi_term::Colour;
    use color::{Colors, Theme};
    use std::env;
    use std::fs;
    use std::process::Command;
    use time;

    #[test]
    fn test_an_hour_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_an_hour_old_file_color.tmp");

        let creation_date = (time::now() - time::Duration::seconds(4)).to_local();

        let success = Command::new("touch")
            .arg("-t")
            .arg(creation_date.strftime("%Y%m%d%H%M.%S").unwrap().to_string())
            .arg(&file_path)
            .status()
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        assert_eq!(
            Colour::Fixed(40).paint(creation_date.ctime().to_string()),
            date.render(&colors)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_day_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_day_old_file_color.tmp");

        let creation_date = (time::now() - time::Duration::hours(4)).to_local();

        let success = Command::new("touch")
            .arg("-t")
            .arg(creation_date.strftime("%Y%m%d%H%M.%S").unwrap().to_string())
            .arg(&file_path)
            .status()
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(Theme::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        assert_eq!(
            Colour::Fixed(42).paint(creation_date.ctime().to_string()),
            date.render(&colors)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_several_days_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_several_days_old_file_color.tmp");

        let creation_date = time::now() - time::Duration::days(2);

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

        assert_eq!(
            Colour::Fixed(36).paint(creation_date.ctime().to_string()),
            date.render(&colors)
        );

        fs::remove_file(file_path).unwrap();
    }
}
