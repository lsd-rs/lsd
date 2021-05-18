use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(unix)]
#[derive(Clone, Debug)]
pub struct Owner {
    user: u32,
    group: u32,
}

#[cfg(windows)]
#[derive(Clone, Debug)]
pub struct Owner {
    user: String,
    group: String,
}

#[cfg(unix)]
impl From<&Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        Self {
            user: meta.uid(),
            group: meta.gid(),
        }
    }
}

impl Owner {
    #[cfg(windows)]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }

    #[cfg(windows)]
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.user.clone(), &Elem::User)
    }

    #[cfg(windows)]
    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.group.clone(), &Elem::Group)
    }

    #[cfg(unix)]
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        let user = match users::get_user_by_uid(self.user) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => self.user.to_string(),
        };
        colors.colorize(user, &Elem::User)
    }

    #[cfg(unix)]
    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        let group = match users::get_group_by_gid(self.group) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => self.group.to_string(),
        };
        colors.colorize(group, &Elem::Group)
    }
}
