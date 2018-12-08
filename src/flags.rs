use clap::ArgMatches;

#[derive(Clone, Debug, Copy)]
pub struct Flags {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub recursive: bool,
    pub color: WhenFlag,
}

impl<'a> From<ArgMatches<'a>> for Flags {
    fn from(matches: ArgMatches) -> Self {
        Flags {
            display_all: matches.is_present("all"),
            display_long: matches.is_present("long"),
            display_online: matches.is_present("oneline"),
            display_tree: matches.is_present("tree"),
            recursive: matches.is_present("recursive"),
            color: WhenFlag::from(matches.value_of("color").unwrap()),
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
