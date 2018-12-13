use clap::ArgMatches;

#[derive(Clone, Debug, Copy)]
pub struct Flags {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub display_indicators: bool,
    pub recursive: bool,
    pub date: DateFlag,
    pub color: WhenFlag,
    pub icon: WhenFlag,
}

impl<'a> From<ArgMatches<'a>> for Flags {
    fn from(matches: ArgMatches) -> Self {
        Flags {
            display_all: matches.is_present("all"),
            display_long: matches.is_present("long"),
            display_online: matches.is_present("oneline"),
            display_tree: matches.is_present("tree"),
            display_indicators: matches.is_present("indicators"),
            recursive: matches.is_present("recursive"),
            date: DateFlag::from(matches.value_of("date").unwrap()),
            color: WhenFlag::from(matches.value_of("color").unwrap()),
            icon: WhenFlag::from(matches.value_of("icon").unwrap()),
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
