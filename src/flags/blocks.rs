//! This module defines the [Blocks] struct. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Blocks::configure_from) method.

use super::Configurable;
use crate::config_file::Config;
use crate::print_error;

use std::convert::TryFrom;

use clap::ArgMatches;

/// A struct to hold a [Vec] of [Block]s and to provide methods to create it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blocks(pub Vec<Block>);

impl Blocks {
    /// This returns a Blocks struct for the long format.
    ///
    /// It contains the [Block]s [Permission](Block::Permission), [User](Block::User),
    /// [Group](Block::Group), [Size](Block::Size), [Date](Block::Date) and [Name](Block::Name).
    fn long() -> Self {
        Self(vec![
            Block::Permission,
            Block::User,
            Block::Group,
            Block::Size,
            Block::Date,
            Block::Name,
        ])
    }

    /// Checks whether `self` already contains a [Block] of variant [INode](Block::INode).
    fn contains_inode(&self) -> bool {
        self.0.contains(&Block::INode)
    }

    /// Prepends a [Block] of variant [INode](Block::INode) to `self`.
    fn prepend_inode(&mut self) {
        self.0.insert(0, Block::INode);
    }

    /// Prepends a [Block] of variant [INode](Block::INode), if `self` does not already contain a
    /// Block of that variant.
    fn optional_prepend_inode(&mut self) {
        if !self.contains_inode() {
            self.prepend_inode()
        }
    }

    pub fn displays_size(&self) -> bool {
        self.0.contains(&Block::Size)
    }

    /// Inserts a [Block] of variant [INode](Block::Context), if `self` does not already contain a
    /// [Block] of that variant. The positioning will be best-effort approximation of coreutils
    /// ls position for a security context
    fn optional_insert_context(&mut self) {
        if self.0.contains(&Block::Context) {
            return;
        }
        let mut pos = self.0.iter().position(|elem| *elem == Block::Group);
        if pos.is_none() {
            pos = self.0.iter().position(|elem| *elem == Block::User);
        }
        match pos {
            Some(pos) => self.0.insert(pos + 1, Block::Context),
            None => self.0.insert(0, Block::Context),
        }
    }
}

impl Configurable<Self> for Blocks {
    /// Returns a value from either [ArgMatches], a [Config] or a default value.
    /// Unless the "long" argument is passed, this returns [Default::default]. Otherwise the first
    /// value, that is not [None], is used. The order of precedence for the value used is:
    /// - [from_arg_matches](Blocks::from_arg_matches)
    /// - [from_config](Blocks::from_config)
    /// - [long](Blocks::long)
    ///
    /// No matter if the "long" argument was passed, if the "inode" argument is passed and the
    /// `Blocks` does not contain a [Block] of variant [INode](Block::INode) yet, one is prepended
    /// to the returned value.
    fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let mut blocks = if matches.is_present("long") {
            Self::long()
        } else {
            Default::default()
        };

        if matches.is_present("long") {
            if let Some(value) = Self::from_config(config) {
                blocks = value;
            }
        }

        if let Some(value) = Self::from_arg_matches(matches) {
            blocks = value;
        }

        if matches.is_present("context") {
            blocks.optional_insert_context();
        }
        if matches.is_present("inode") {
            blocks.optional_prepend_inode();
        }

        blocks
    }

    /// Get a potential `Blocks` struct from [ArgMatches].
    ///
    /// If the "blocks" argument is passed, then this returns a `Blocks` containing the parameter
    /// values in a [Some]. Otherwise if the "long" argument is passed, this returns
    /// [Blocks::long]. Finally if none of the previous happened, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.occurrences_of("blocks") == 0 {
            return None;
        }

        if let Some(values) = matches.values_of("blocks") {
            let mut blocks: Vec<Block> = Vec::with_capacity(values.len());
            for value in values {
                blocks.push(Block::try_from(value).unwrap_or_else(|_| {
                    // Invalid value should be handled by `clap` when building an `ArgMatches`
                    unreachable!("Invalid value '{value}' for 'blocks'")
                }));
            }
            Some(Self(blocks))
        } else {
            None
        }
    }

    /// Get a potential `Blocks` struct from a [Config].
    ///
    /// If the [Config] contains an array of blocks values,
    /// its [String] values is returned as `Blocks` in a [Some].
    /// Otherwise it returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(c) = &config.blocks {
            let mut blocks: Vec<Block> = vec![];
            for b in c.iter() {
                match Block::try_from(b.as_str()) {
                    Ok(block) => blocks.push(block),
                    Err(err) => print_error!("{}.", err),
                }
            }
            if blocks.is_empty() {
                None
            } else {
                Some(Self(blocks))
            }
        } else {
            None
        }
    }
}

/// The default value for `Blocks` contains a [Vec] of [Name](Block::Name).
impl Default for Blocks {
    fn default() -> Self {
        Self(vec![Block::Name])
    }
}

/// A block of data to show.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Block {
    Permission,
    User,
    Group,
    Context,
    Size,
    SizeValue,
    Date,
    Name,
    INode,
    Links,
}

impl Block {
    pub fn get_header(&self) -> &'static str {
        match self {
            Block::INode => "INode",
            Block::Links => "Links",
            Block::Permission => "Permissions",
            Block::User => "User",
            Block::Group => "Group",
            Block::Context => "Context",
            Block::Size => "Size",
            Block::SizeValue => "SizeValue",
            Block::Date => "Date Modified",
            Block::Name => "Name",
        }
    }
}

impl TryFrom<&str> for Block {
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "permission" => Ok(Self::Permission),
            "user" => Ok(Self::User),
            "group" => Ok(Self::Group),
            "context" => Ok(Self::Context),
            "size" => Ok(Self::Size),
            "size_value" => Ok(Self::SizeValue),
            "date" => Ok(Self::Date),
            "name" => Ok(Self::Name),
            "inode" => Ok(Self::INode),
            "links" => Ok(Self::Links),
            _ => Err(format!("Not a valid block name: {string}")),
        }
    }
}

#[cfg(test)]
mod test_blocks {
    use super::Block;
    use super::Blocks;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_configure_from_without_long() {
        let argv = ["lsd"];
        let target = Blocks::default();

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_with_long() {
        let argv = ["lsd", "--long"];
        let target = Blocks::long();

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_with_blocks_and_without_long() {
        let argv = ["lsd", "--blocks", "permission"];
        let target = Blocks(vec![Block::Permission]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_with_blocks_and_long() {
        let argv = ["lsd", "--long", "--blocks", "permission"];
        let target = Blocks(vec![Block::Permission]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_with_inode() {
        let argv = ["lsd", "--inode"];
        let target = Blocks(vec![Block::INode, Block::Name]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_prepend_inode_without_long() {
        let argv = ["lsd", "--blocks", "permission", "--inode"];
        let target = Blocks(vec![Block::INode, Block::Permission]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_prepend_inode_with_long() {
        let argv = ["lsd", "--long", "--blocks", "permission", "--inode"];
        let target = Blocks(vec![Block::INode, Block::Permission]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_ignore_prepend_inode_without_long() {
        let argv = ["lsd", "--blocks", "permission,inode", "--inode"];
        let target = Blocks(vec![Block::Permission, Block::INode]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_configure_from_ignore_prepend_inode_with_long() {
        let argv = ["lsd", "--long", "--blocks", "permission,inode", "--inode"];
        let target = Blocks(vec![Block::Permission, Block::INode]);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq!(result, target);
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = ["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(matches!(Blocks::from_arg_matches(&matches), None));
    }

    #[test]
    fn test_from_arg_matches_one() {
        let argv = ["lsd", "--blocks", "permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission]);
        assert_eq!(Blocks::from_arg_matches(&matches), Some(test_blocks));
    }

    #[test]
    fn test_from_arg_matches_multi_occurences() {
        let argv = ["lsd", "--blocks", "permission", "--blocks", "name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert_eq!(Blocks::from_arg_matches(&matches), Some(test_blocks));
    }

    #[test]
    fn test_from_arg_matches_multi_values() {
        let argv = ["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert_eq!(Blocks::from_arg_matches(&matches), Some(test_blocks));
    }

    #[test]
    fn test_from_arg_matches_reversed_default() {
        let argv = ["lsd", "--blocks", "name,date,size,group,user,permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        assert_eq!(Blocks::from_arg_matches(&matches), Some(test_blocks));
    }

    #[test]
    fn test_from_arg_matches_every_second_one() {
        let argv = ["lsd", "--blocks", "permission,group,date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert_eq!(Blocks::from_arg_matches(&matches), Some(test_blocks));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Blocks::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into()]);

        let blocks = Blocks(vec![Block::Permission]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_reversed_default() {
        let target = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        let mut c = Config::with_none();
        c.blocks = Some(vec![
            "name".into(),
            "date".into(),
            "size".into(),
            "group".into(),
            "user".into(),
            "permission".into(),
        ]);

        assert_eq!(Some(target), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_every_second_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "group".into(), "date".into()]);
        let blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_invalid_is_ignored() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "foo".into(), "date".into()]);
        let blocks = Blocks(vec![Block::Permission, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_context_not_present_on_cli() {
        let argv = ["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none());
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|&x| *x == Block::Context).count(), 0);
    }

    #[test]
    fn test_context_present_if_context_on() {
        let argv = ["lsd", "--context"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none());
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|&x| *x == Block::Context).count(), 1);
    }

    #[test]
    fn test_only_one_context_no_other_blocks_affected() {
        let argv = [
            "lsd",
            "--context",
            "--blocks",
            "name,date,size,context,group,user,permission",
        ];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Context,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        let parsed_blocks = Blocks::from_arg_matches(&matches).unwrap();
        assert_eq!(test_blocks, parsed_blocks);
    }
}

#[cfg(test)]
mod test_block {
    use super::Block;

    use std::convert::TryFrom;

    #[test]
    fn test_err() {
        assert_eq!(
            Err(String::from("Not a valid block name: foo")),
            Block::try_from("foo")
        );
    }

    #[test]
    fn test_permission() {
        assert_eq!(Ok(Block::Permission), Block::try_from("permission"));
    }

    #[test]
    fn test_user() {
        assert_eq!(Ok(Block::User), Block::try_from("user"));
    }

    #[test]
    fn test_group() {
        assert_eq!(Ok(Block::Group), Block::try_from("group"));
    }

    #[test]
    fn test_size() {
        assert_eq!(Ok(Block::Size), Block::try_from("size"));
    }

    #[test]
    fn test_size_value() {
        assert_eq!(Ok(Block::SizeValue), Block::try_from("size_value"));
    }

    #[test]
    fn test_date() {
        assert_eq!(Ok(Block::Date), Block::try_from("date"));
    }

    #[test]
    fn test_name() {
        assert_eq!(Ok(Block::Name), Block::try_from("name"));
    }

    #[test]
    fn test_inode() {
        assert_eq!(Ok(Block::INode), Block::try_from("inode"));
    }

    #[test]
    fn test_links() {
        assert_eq!(Ok(Block::Links), Block::try_from("links"));
    }

    #[test]
    fn test_context() {
        assert_eq!(Ok(Block::Context), Block::try_from("context"));
    }

    #[test]
    fn test_block_headers() {
        assert_eq!(Block::INode.get_header(), "INode");
        assert_eq!(Block::Links.get_header(), "Links");
        assert_eq!(Block::Permission.get_header(), "Permissions");
        assert_eq!(Block::User.get_header(), "User");
        assert_eq!(Block::Group.get_header(), "Group");
        assert_eq!(Block::Context.get_header(), "Context");
        assert_eq!(Block::Size.get_header(), "Size");
        assert_eq!(Block::SizeValue.get_header(), "SizeValue");
        assert_eq!(Block::Date.get_header(), "Date Modified");
        assert_eq!(Block::Name.get_header(), "Name");
    }
}
