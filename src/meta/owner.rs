use color::{Colors, Elem};
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
            .to_str()
            .expect("failed to convert user name to str")
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("failed to get the group name")
            .name()
            .to_str()
            .expect("failed to convert group name to str")
            .to_string();

        Owner { user, group }
    }
}

impl Owner {
    pub fn render(&self, user_alignment: usize, group_alignment: usize) -> String {
        let mut content =
            String::with_capacity(user_alignment + group_alignment + 2 /* spaces */);

        content += &Colors[&Elem::User].paint(&self.user).to_string();
        for _ in 0..(user_alignment - self.user.len()) {
            content.push(' ');
        }

        // the space between the name and the group.
        content.push(' ');

        content += &Colors[&Elem::Group].paint(&self.group).to_string();
        for _ in 0..(group_alignment - self.group.len()) {
            content.push(' ');
        }

        content
    }

    pub fn render_user(&self) -> String {
        self.user.clone()
    }

    pub fn render_group(&self) -> String {
        self.group.clone()
    }
}
