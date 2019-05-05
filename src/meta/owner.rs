use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;

#[derive(Debug)]
pub struct Owner {
    user: String,
    group: String,
}

impl Owner {
    #[cfg_attr(unix, allow(dead_code))]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }
}

#[cfg(unix)]
impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        use users::{get_group_by_gid, get_user_by_uid};

        let user = match get_user_by_uid(meta.uid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        };

        let group = match get_group_by_gid(meta.gid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        };

        Self { user, group }
    }
}

impl Owner {
    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn group(&self) -> String {
        self.group.clone()
    }

    pub fn render_user(&self, colors: &Colors, user_alignment: usize) -> ColoredString {
        let mut res = String::with_capacity(user_alignment - self.user.len());

        for _ in 0..(user_alignment - self.user.len()) {
            res.push(' ');
        }

        res += &self.user;
        colors.colorize(res, &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors, group_alignment: usize) -> ColoredString {
        let mut res = String::with_capacity(group_alignment - self.group.len());

        for _ in 0..(group_alignment - self.group.len()) {
            res.push(' ');
        }

        res += &self.group;
        colors.colorize(res, &Elem::Group)
    }
}
