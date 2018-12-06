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
