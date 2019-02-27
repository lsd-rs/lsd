use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("lsd")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .multiple(true)
                .help("Do not ignore entries starting with ."),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .multiple(true)
                .help("When to use terminal colours"),
        )
        .arg(
            Arg::with_name("icon")
                .long("icon")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .multiple(true)
                .help("When to print the icons"),
        )
        .arg(
            Arg::with_name("icon-theme")
                .long("icon-theme")
                .possible_value("fancy")
                .possible_value("unicode")
                .default_value("fancy")
                .multiple(true)
                .help("Whether to use fancy or unicode icons"),
        )
        .arg(
            Arg::with_name("indicators")
                .short("F")
                .long("classify")
                .multiple(true)
                .help("Append indicator (one of */=>@|) at the end of the file names"),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .multiple(true)
                .help("Display extended file metadata as a table"),
        )
        .arg(
            Arg::with_name("oneline")
                .short("1")
                .long("oneline")
                .multiple(true)
                .help("Display one entry per line"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("R")
                .long("recursive")
                .multiple(true)
                .conflicts_with("tree")
                .help("Recurse into directories"),
        )
        .arg(
            Arg::with_name("tree")
                .long("tree")
                .multiple(true)
                .conflicts_with("recursive")
                .help("Recurse into directories and present the result as a tree"),
        )
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .takes_value(true)
                .value_name("num")
                .help("Stop recursing into directories after reaching specified depth"),
        )
        .arg(
            Arg::with_name("date")
                .long("date")
                .possible_value("date")
                .possible_value("relative")
                .default_value("date")
                .multiple(true)
                .help("How to display date"),
        )
        .arg(
            Arg::with_name("timesort")
                .short("t")
                .long("timesort")
                .multiple(true)
                .help("Sort by time modified"),
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .multiple(true)
                .help("Reverse the order of the sort"),
        )
        .arg(
            Arg::with_name("group-dirs")
                .long("group-dirs")
                .possible_value("none")
                .possible_value("first")
                .possible_value("last")
                .default_value("none")
                .multiple(true)
                .help("Sort the directories then the files"),
        )
        .arg(
            Arg::with_name("classic")
                .long("classic")
                .help("Enable classic mode (no colors or icons)"),
        )
}
