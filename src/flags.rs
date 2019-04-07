use clap::{ArgMatches, Error, ErrorKind};

#[derive(Clone, Debug, Copy)]
pub struct Flags {
    pub display: Display,
    pub layout: Layout,
    pub display_indicators: bool,
    pub recursive: bool,
    pub sort_by: SortFlag,
    pub sort_order: SortOrder,
    pub directory_order: DirOrderFlag,
    pub size: SizeFlag,
    pub date: DateFlag,
    pub color: WhenFlag,
    pub icon: WhenFlag,
    pub icon_theme: IconTheme,
    pub recursion_depth: usize,
}

impl Flags {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let classic_mode = matches.is_present("classic");
        let color_inputs: Vec<&str> = matches.values_of("color").unwrap().collect();
        let icon_inputs: Vec<&str> = matches.values_of("icon").unwrap().collect();
        let icon_theme_inputs: Vec<&str> = matches.values_of("icon-theme").unwrap().collect();
        let size_inputs: Vec<&str> = matches.values_of("size").unwrap().collect();
        let date_inputs: Vec<&str> = matches.values_of("date").unwrap().collect();
        let dir_order_inputs: Vec<&str> = matches.values_of("group-dirs").unwrap().collect();

        let display = if matches.is_present("all") {
            Display::DisplayAll
        } else if matches.is_present("almost-all") {
            Display::DisplayAlmostAll
        } else {
            Display::DisplayOnlyVisible
        };

        let sort_by = if matches.is_present("timesort") {
            SortFlag::Time
        } else {
            SortFlag::Name
        };
        let sort_order = if matches.is_present("reverse") {
            SortOrder::Reverse
        } else {
            SortOrder::Default
        };
        let layout = if matches.is_present("tree") {
            Layout::Tree
        } else if matches.is_present("long") {
            Layout::OneLine { long: true }
        } else if matches.is_present("oneline") {
            Layout::OneLine { long: false }
        } else {
            Layout::Grid
        };
        let recursive = matches.is_present("recursive");
        let recursion_depth = match matches.value_of("depth") {
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

        Ok(Self {
            display,
            layout,
            display_indicators: matches.is_present("indicators"),
            recursive,
            recursion_depth,
            sort_by,
            sort_order,
            size: SizeFlag::from(size_inputs[size_inputs.len() - 1]),
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
            directory_order: if classic_mode {
                DirOrderFlag::None
            } else {
                DirOrderFlag::from(dir_order_inputs[dir_order_inputs.len() - 1])
            },
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
            sort_by: SortFlag::Name,
            sort_order: SortOrder::Default,
            directory_order: DirOrderFlag::None,
            size: SizeFlag::Default,
            date: DateFlag::Date,
            color: WhenFlag::Auto,
            icon: WhenFlag::Auto,
            icon_theme: IconTheme::Fancy,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Display {
    DisplayAll,
    DisplayAlmostAll,
    DisplayOnlyVisible,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SizeFlag {
    Default,
    Short,
}

impl<'a> From<&'a str> for SizeFlag {
    fn from(size: &'a str) -> Self {
        match size {
            "default" => SizeFlag::Default,
            "short" => SizeFlag::Short,
            _ => panic!("invalid \"size\" flag: {}", size),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DateFlag {
    Date,
    Relative,
}

impl<'a> From<&'a str> for DateFlag {
    fn from(time: &'a str) -> Self {
        match time {
            "date" => DateFlag::Date,
            "relative" => DateFlag::Relative,
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
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Default,
    Reverse,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DirOrderFlag {
    None,
    First,
    Last,
}

impl<'a> From<&'a str> for DirOrderFlag {
    fn from(when: &'a str) -> Self {
        match when {
            "none" => DirOrderFlag::None,
            "first" => DirOrderFlag::First,
            "last" => DirOrderFlag::Last,
            _ => panic!("invalid \"when\" flag: {}", when),
        }
    }
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
    OneLine { long: bool },
}

#[cfg(test)]
mod test {
    use super::Flags;
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
}
