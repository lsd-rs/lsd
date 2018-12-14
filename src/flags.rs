use clap::ArgMatches;

#[derive(Clone, Debug, Copy)]
pub struct Flags {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub display_indicators: bool,
    pub recursive: bool,
    pub sort: (SortFlag, SortOrder),
    pub date: DateFlag,
    pub color: WhenFlag,
    pub icon: WhenFlag,
}

impl<'a> From<ArgMatches<'a>> for Flags {
    fn from(matches: ArgMatches) -> Self {
        let color_inputs: Vec<&str> = matches.values_of("color").unwrap().collect();
        let icon_inputs: Vec<&str> = matches.values_of("icon").unwrap().collect();
        let date_inputs: Vec<&str> = matches.values_of("date").unwrap().collect();

        let sort_flag = if matches.is_present("timesort") {
            SortFlag::Time
        } else {
            SortFlag::Lexicographical
        };
        let sort_order = if matches.is_present("reverse") {
            SortOrder::Reverse
        } else {
            SortOrder::Default
        };

        Self {
            display_all: matches.is_present("all"),
            display_long: matches.is_present("long"),
            display_online: matches.is_present("oneline"),
            display_tree: matches.is_present("tree"),
            display_indicators: matches.is_present("indicators"),
            recursive: matches.is_present("recursive"),
            sort: (sort_flag, sort_order),
            // Take only the last value
            date: DateFlag::from(date_inputs[date_inputs.len() - 1]),
            color: WhenFlag::from(color_inputs[color_inputs.len() - 1]),
            icon: WhenFlag::from(icon_inputs[icon_inputs.len() - 1]),
        }
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            display_all: false,
            display_long: false,
            display_online: false,
            display_tree: false,
            display_indicators: false,
            recursive: false,
            sort: (SortFlag::Lexicographical, SortOrder::Default),
            date: DateFlag::Date,
            color: WhenFlag::Auto,
            icon: WhenFlag::Auto,
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
    Lexicographical,
    Time,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Default,
    Reverse,
}
