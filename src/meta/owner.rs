use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;

#[derive(Clone, Debug)]
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

impl Default for Owner {
    fn default() -> Owner
    {
        Owner { user: String::from(""), group: String::from("") }
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
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        if self.user.len() == 0 {
            colors.colorize("?".to_owned(), &Elem::User)
        } else {
            colors.colorize(self.user.clone(), &Elem::User)
        }
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        if self.group.len() == 0 {
            colors.colorize("?".to_owned(), &Elem::Group)
        } else {
            colors.colorize(self.group.clone(), &Elem::Group)
        }
    }
}
