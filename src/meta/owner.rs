use crate::color::{ColoredString, Colors, Elem};
use crate::Flags;
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

#[cfg(unix)]
impl From<&Metadata> for Owner {
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
    pub fn render_user(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        colors.colorize(
            truncate(
                &self.user,
                flags.truncate_owner.after,
                flags.truncate_owner.marker.clone(),
            ),
            &Elem::User,
        )
    }

    pub fn render_group(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        colors.colorize(
            truncate(
                &self.group,
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
