use clap::ArgMatches;

#[derive(Clone, Debug, Copy)]
pub struct Flags {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub recursive: bool,
}

impl<'a> From<ArgMatches<'a>> for Flags {
    fn from(matches: ArgMatches) -> Self {
        Flags {
            display_all: matches.is_present("all"),
            display_long: matches.is_present("long"),
            display_online: matches.is_present("oneline"),
            display_tree: matches.is_present("tree"),
            recursive: matches.is_present("recursive"),
        }
    }
}
