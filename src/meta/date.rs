use color::{Colors, Elem};
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
    pub fn render(&self) -> String {
        let now = time::now();

        let color;
        if self.0 > now - Duration::hours(1) {
            color = Colors[&Elem::HourOld];
        } else if self.0 > now - Duration::days(1) {
            color = Colors[&Elem::DayOld];
        } else {
            color = Colors[&Elem::Older];
        }

        color.paint(self.0.ctime().to_string()).to_string()
    }
}
