use crate::color::{ColoredString, Colors, Elem};
use crate::Flags;
#[cfg(unix)]
use std::fs::Metadata;
#[cfg(unix)]
use users::{Groups, Users, UsersCache};

#[derive(Default)]
pub struct Cache {
    #[cfg(unix)]
    users: UsersCache,
    #[cfg(unix)]
    groups: UsersCache,
}

#[cfg(unix)]
#[derive(Clone, Debug, Default)]
pub struct Owner {
    user: u32,
    group: u32,
}

#[cfg(windows)]
#[derive(Clone, Debug, Default)]
pub struct Owner {
    user: String,
    group: String,
}

impl Owner {
    #[cfg(windows)]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }
}

#[cfg(unix)]
impl From<&Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            user: meta.uid(),
            group: meta.gid(),
        }
    }
}

fn truncate(input: &str, after: Option<usize>, marker: Option<String>) -> String {
    let mut output = input.to_string();

    if let Some(after) = after {
        if output.len() > after {
            output.truncate(after);

            if let Some(marker) = marker {
                output.push_str(&marker);
            }
        }
    }

    output
}

impl Owner {
    // allow unused variables because cache is used in unix, maybe we can cache for windows in the future
    #[allow(unused_variables)]
    pub fn render_user(&self, colors: &Colors, cache: &Cache, flags: &Flags) -> ColoredString {
        #[cfg(unix)]
        let user = &match cache.users.get_user_by_uid(self.user) {
            Some(user) => user.name().to_string_lossy().to_string(),
            None => self.user.to_string(),
        };
        #[cfg(windows)]
        let user = &self.user;

        colors.colorize(
            truncate(
                user,
                flags.truncate_owner.after,
                flags.truncate_owner.marker.clone(),
            ),
            &Elem::User,
        )
    }

    // allow unused variables because cache is used in unix, maybe we can cache for windows in the future
    #[allow(unused_variables)]
    pub fn render_group(&self, colors: &Colors, cache: &Cache, flags: &Flags) -> ColoredString {
        #[cfg(unix)]
        let group = &match cache.groups.get_group_by_gid(self.group) {
            Some(group) => group.name().to_string_lossy().to_string(),
            None => self.group.to_string(),
        };
        #[cfg(windows)]
        let group = &self.group;

        colors.colorize(
            truncate(
                group,
                flags.truncate_owner.after,
                flags.truncate_owner.marker.clone(),
            ),
            &Elem::Group,
        )
    }
}

#[cfg(test)]
mod test_truncate {
    use crate::meta::owner::truncate;

    #[test]
    fn test_none() {
        assert_eq!("a", truncate("a", None, None));
    }

    #[test]
    fn test_unchanged_without_marker() {
        assert_eq!("a", truncate("a", Some(1), None));
    }

    #[test]
    fn test_unchanged_with_marker() {
        assert_eq!("a", truncate("a", Some(1), Some("…".to_string())));
    }

    #[test]
    fn test_truncated_without_marker() {
        assert_eq!("a", truncate("ab", Some(1), None));
    }

    #[test]
    fn test_truncated_with_marker() {
        assert_eq!("a…", truncate("ab", Some(1), Some("…".to_string())));
    }
}
