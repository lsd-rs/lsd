use color::{ColoredString, Colors, Elem};
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub struct Owner {
    user: String,
    group: String,
}

impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        let user = get_user_by_uid(meta.uid())
            .expect("failed to get user name")
            .name()
            .to_string_lossy()
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("failed to get the group name")
            .name()
            .to_string_lossy()
            .to_string();

        Owner { user, group }
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
