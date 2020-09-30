use clap::{ArgMatches, Error, ErrorKind};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Flags {
    pub display: Display,
    pub layout: Layout,
    pub display_indicators: bool,
    pub recursive: bool,
    pub sorters: Vec<(SortOrder, SortFlag)>,
    pub size: SizeFlag,
    pub date: DateFlag,
    pub color: WhenFlag,
    pub icon: WhenFlag,
    pub icon_theme: IconTheme,
    pub inode: bool,
    pub recursion_depth: usize,
    pub blocks: Vec<Block>,
    pub no_symlink: bool,
    pub total_size: bool,
    pub ignore_globs: GlobSet,
    pub dereference: bool,
}

impl Flags {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let classic_mode = matches.is_present("classic");
        let color_inputs: Vec<&str> = matches.values_of("color").unwrap().collect();
        let icon_inputs: Vec<&str> = matches.values_of("icon").unwrap().collect();
        let icon_theme_inputs: Vec<&str> = matches.values_of("icon-theme").unwrap().collect();
        let size_inputs: Vec<&str> = matches.values_of("size").unwrap().collect();
        let date_inputs: Vec<&str> = matches.values_of("date").unwrap().collect();
        let ignore_globs_inputs: Vec<&str> = matches.values_of("ignore-glob").unwrap().collect();
        let dereference = matches.is_present("dereference");
        // inode set layout to oneline and blocks to inode,name
        let inode = matches.is_present("inode");
        let blocks_inputs: Vec<&str> = if let Some(blocks) = matches.values_of("blocks") {
            blocks.collect()
        } else {
            vec![]
        };

        let display = if matches.is_present("all") {
            Display::DisplayAll
        } else if matches.is_present("almost-all") {
            Display::DisplayAlmostAll
        } else if matches.is_present("directory-only") {
            Display::DisplayDirectoryItself
        } else {
            Display::DisplayOnlyVisible
        };

        let sorting_result: Result<_, Error> = {
            let mut sorting: Vec<(SortOrder, SortFlag)> = vec![];

            if let Some(dir_val) = matches.value_of("group-dirs") {
                match dir_val {
                    "first" => sorting.push((SortOrder::Default, SortFlag::Directory)),
                    "last" => sorting.push((SortOrder::Reverse, SortFlag::Directory)),
                    "none" => {}
                    _ => {
                        return Err(Error {
                            message: "unknown field in group-dirs".to_string(),
                            info: None,
                            kind: clap::ErrorKind::InvalidValue,
                        })
                    }
                }
            };

            let sort_by = if matches.is_present("timesort") {
                SortFlag::Time
            } else if matches.is_present("sizesort") {
                SortFlag::Size
            } else if matches.is_present("extensionsort") {
                SortFlag::Extension
            } else if matches.is_present("versionsort") {
                SortFlag::Version
            } else {
                SortFlag::Name
            };

            let sort_order = if matches.is_present("reverse") {
                SortOrder::Reverse
            } else {
                SortOrder::Default
            };

            sorting.push((sort_order, sort_by));
            Ok(sorting)
        };

        let layout = if matches.is_present("tree") {
            Layout::Tree
        } else if matches.is_present("long")
            || matches.is_present("oneline")
            || blocks_inputs.len() > 1
            || inode
        {
            Layout::OneLine
        } else {
            Layout::Grid
        };

        let recursive = matches.is_present("recursive");
        let recursion_input = matches.values_of("depth").and_then(Iterator::last);
        let recursion_depth = match recursion_input {
            Some(str) if recursive || layout == Layout::Tree => match str.parse::<usize>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(Error::with_description(
                        "The argument '--depth' requires a valid positive number",
                        ErrorKind::ValueValidation,
                    ));
                }
            },
            Some(_) => {
                return Err(Error::with_description(
                    "The argument '--depth' requires '--tree' or '--recursive'",
                    ErrorKind::MissingRequiredArgument,
                ));
            }
            None => usize::max_value(),
        };

        let mut blocks: Vec<Block> = if !blocks_inputs.is_empty() {
            blocks_inputs.into_iter().map(Block::from).collect()
        } else if matches.is_present("long") {
            vec![
                Block::Permission,
                Block::User,
                Block::Group,
                Block::Size,
                Block::Date,
                Block::Name,
            ]
        } else {
            vec![Block::Name]
        };

        // Add inode as first column if with inode flag
        if inode && !blocks.contains(&Block::INode) {
            blocks.insert(0, Block::INode);
        }

        let mut ignore_globs_builder = GlobSetBuilder::new();
        for pattern in ignore_globs_inputs {
            let glob = match Glob::new(pattern) {
                Ok(g) => g,
                Err(e) => {
                    return Err(Error::with_description(
                        &e.to_string(),
                        ErrorKind::ValueValidation,
                    ));
                }
            };
            ignore_globs_builder.add(glob);
        }

        let ignore_globs = match ignore_globs_builder.build() {
            Ok(globs) => globs,
            Err(e) => {
                return Err(Error::with_description(
                    &e.to_string(),
                    ErrorKind::ValueValidation,
                ));
            }
        };

        Ok(Self {
            display,
            layout,
            display_indicators: matches.is_present("indicators"),
            recursive,
            recursion_depth,
            sorters: sorting_result?,
            size: SizeFlag::from(size_inputs[size_inputs.len() - 1]),
            ignore_globs,
            blocks,
            // Take only the last value
            date: if classic_mode {
                DateFlag::Date
            } else {
                DateFlag::from(date_inputs[date_inputs.len() - 1])
            },
            color: if classic_mode {
                WhenFlag::Never
            } else {
                WhenFlag::from(color_inputs[color_inputs.len() - 1])
            },
            icon: if classic_mode {
                WhenFlag::Never
            } else {
                WhenFlag::from(icon_inputs[icon_inputs.len() - 1])
            },
            icon_theme: IconTheme::from(icon_theme_inputs[icon_theme_inputs.len() - 1]),
            no_symlink: matches.is_present("no-symlink"),
            total_size: matches.is_present("total-size"),
            inode,
            dereference,
        })
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            display: Display::DisplayOnlyVisible,
            layout: Layout::Grid,
            display_indicators: false,
            recursive: false,
            recursion_depth: usize::max_value(),
            sorters: vec![(SortOrder::Default, SortFlag::Name)],
            size: SizeFlag::Default,
            date: DateFlag::Date,
            color: WhenFlag::Auto,
            icon: WhenFlag::Auto,
            icon_theme: IconTheme::Fancy,
            blocks: vec![],
            no_symlink: false,
            total_size: false,
            ignore_globs: GlobSet::empty(),
            inode: false,
            dereference: false,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Block {
    Permission,
    User,
    Group,
    Size,
    SizeValue,
    Date,
    Name,
    INode,
}
impl<'a> From<&'a str> for Block {
    fn from(block: &'a str) -> Self {
        match block {
            // "filetype" => Block::FileType,
            "permission" => Block::Permission,
            "user" => Block::User,
            "group" => Block::Group,
            "size" => Block::Size,
            "size_value" => Block::SizeValue,
            "date" => Block::Date,
            "name" => Block::Name,
            "inode" => Block::INode,
            _ => panic!("invalid \"time\" flag: {}", block),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Display {
    DisplayAll,
    DisplayAlmostAll,
    DisplayDirectoryItself,
    DisplayOnlyVisible,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SizeFlag {
    Default,
    Short,
    Bytes,
}

impl<'a> From<&'a str> for SizeFlag {
    fn from(size: &'a str) -> Self {
        match size {
            "default" => SizeFlag::Default,
            "short" => SizeFlag::Short,
            "bytes" => SizeFlag::Bytes,
            _ => panic!("invalid \"size\" flag: {}", size),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFlag {
    Date,
    Relative,
    Formatted(String),
}

impl<'a> From<&'a str> for DateFlag {
    fn from(time: &'a str) -> Self {
        match time {
            "date" => DateFlag::Date,
            "relative" => DateFlag::Relative,
            time if time.starts_with('+') => DateFlag::Formatted(time[1..].to_owned()),
            _ => panic!("invalid \"time\" flag: {}", time),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum WhenFlag {
    Always,
    Auto,
    Never,
}
impl<'a> From<&'a str> for WhenFlag {
    fn from(when: &'a str) -> Self {
        match when {
            "always" => WhenFlag::Always,
            "auto" => WhenFlag::Auto,
            "never" => WhenFlag::Never,
            _ => panic!("invalid \"when\" flag: {}", when),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortFlag {
    Name,
    Time,
    Size,
    Version,
    Extension,
    Directory,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Default,
    Reverse,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum IconTheme {
    Unicode,
    Fancy,
}

impl<'a> From<&'a str> for IconTheme {
    fn from(theme: &'a str) -> Self {
        match theme {
            "fancy" => IconTheme::Fancy,
            "unicode" => IconTheme::Unicode,
            _ => panic!("invalid \"icon-theme\" flag: {}", theme),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Layout {
    Grid,
    Tree,
    OneLine,
}

#[cfg(test)]
mod test {
    use super::Flags;
    use super::SortFlag;
    use super::SortOrder;
    use crate::app;
    use clap::ErrorKind;

    #[test]
    fn test_validate_depth_value() {
        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "--tree", "--depth", "xx"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn test_useless_depth() {
        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "--depth", "10"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_duplicate_depth() {
        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "--tree", "--depth", "1", "--depth", "2"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().recursion_depth, 2);
    }

    #[test]
    fn test_missing_depth() {
        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "--tree"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().recursion_depth, usize::max_value());
    }

    #[test]
    fn test_multi_sort_use_last() {
        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "-t", "-S"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().sorters,
            vec![(SortOrder::Default, SortFlag::Size)]
        );

        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "-S", "-t"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().sorters,
            vec![(SortOrder::Default, SortFlag::Time)]
        );

        let matches = app::build()
            .get_matches_from_safe(vec!["lsd", "-t", "-S", "-X"])
            .unwrap();
        let res = Flags::from_matches(&matches);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().sorters,
            vec![(SortOrder::Default, SortFlag::Extension)]
        );
    }
}
