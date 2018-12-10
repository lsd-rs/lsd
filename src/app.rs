use clap::{App, Arg};

pub fn build_app() -> App<'static, 'static> {
    App::new("lsd")
        .version(crate_version!())
        .about("An ls comment with a lot of pretty colors and some other stuff.")
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Do not ignore entries starting with ."),
        ).arg(
            Arg::with_name("color")
                .long("color")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .help("When to use terminal colours"),
        ).arg(
            Arg::with_name("indicators")
                .short("F")
                .long("classify")
                .help("Append indicator (one of */=>@|) at the end of the file names"),
        ).arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .help("Display extended file metadata as a table"),
        ).arg(
            Arg::with_name("oneline")
                .short("1")
                .long("oneline")
                .help("Display one entry per line"),
        ).arg(
            Arg::with_name("recursive")
                .short("R")
                .long("recursive")
                .help("Recurse into directories"),
        ).arg(
            Arg::with_name("tree")
                .long("tree")
                .help("Recurse into directories and present the result as a tree"),
        )
}
