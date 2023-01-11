use clap::{Arg, ArgAction, Command, ValueHint};

pub fn build() -> Command<'static> {
    Command::new("lsd")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("FILE")
                .multiple(true)
                .default_value(".")
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .overrides_with("almost-all")
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Do not ignore entries starting with ."),
        )
        .arg(
            Arg::new("almost-all")
                .short('A')
                .overrides_with("all")
                .long("almost-all")
                .action(ArgAction::SetTrue)
                .help("Do not list implied . and .."),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .value_parser(["always", "auto", "never"])
                .default_value("auto")
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("When to use terminal colours"),
        )
        .arg(
            Arg::with_name("icon")
                .long("icon")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .help("When to print the icons"),
        )
        .arg(
            Arg::with_name("icon-theme")
                .long("icon-theme")
                .default_value("fancy")
                .possible_value("fancy")
                .possible_value("unicode")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .help("Whether to use fancy or unicode icons"),
        )
        .arg(
            Arg::with_name("indicators")
                .short('F')
                .long("classify")
                .multiple_occurrences(true)
                .help("Append indicator (one of */=>@|) at the end of the file names"),
        )
        .arg(
            Arg::with_name("long")
                .short('l')
                .long("long")
                .multiple_occurrences(true)
                .help("Display extended file metadata as a table"),
        )
        .arg(
            Arg::with_name("ignore-config")
                .long("ignore-config")
                .help("Ignore the configuration file"),
        )
        .arg(
            Arg::with_name("config-file")
                .long("config-file")
                .help("Provide a custom lsd configuration file")
                .value_name("config-file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("oneline")
                .short('1')
                .long("oneline")
                .multiple_occurrences(true)
                .help("Display one entry per line"),
        )
        .arg(
            Arg::with_name("recursive")
                .short('R')
                .long("recursive")
                .multiple_occurrences(true)
                .conflicts_with("tree")
                .help("Recurse into directories"),
        )
        .arg(
            Arg::with_name("human_readable")
                .short('h')
                .long("human-readable")
                .multiple_occurrences(true)
                .help("For ls compatibility purposes ONLY, currently set by default"),
        )
        .arg(
            Arg::with_name("tree")
                .long("tree")
                .multiple_occurrences(true)
                .conflicts_with("recursive")
                .help("Recurse into directories and present the result as a tree"),
        )
        .arg(
            Arg::new("depth")
                .long("depth")
                .action(ArgAction::Append)
                .takes_value(true)
                .value_name("num")
                .help("Stop recursing into directories after reaching specified depth"),
        )
        .arg(
            Arg::new("directory-only")
                .short('d')
                .long("directory-only")
                .action(ArgAction::SetTrue)
                .conflicts_with("depth")
                .conflicts_with("recursive")
                .help("Display directories themselves, and not their contents (recursively when used with --tree)"),
        )
        .arg(
            Arg::with_name("permission")
                .long("permission")
                .default_value("rwx")
                .possible_value("rwx")
                .possible_value("octal")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .help("How to display permissions"),
        )
        .arg(
            Arg::with_name("size")
                .long("size")
                .possible_value("default")
                .possible_value("short")
                .possible_value("bytes")
                .default_value("default")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .help("How to display size"),
        )
        .arg(
            Arg::with_name("total-size")
                .long("total-size")
                .multiple_occurrences(true)
                .help("Display the total size of directories"),
        )
        .arg(
            Arg::new("date")
                .long("date")
                .value_parser(validate_date_argument)
                .default_value("date")
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("How to display date [possible values: date, relative, +date-time-format]"),
        )
        .arg(
            Arg::with_name("timesort")
                .short('t')
                .long("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple_occurrences(true)
                .help("Sort by time modified"),
        )
        .arg(
            Arg::with_name("sizesort")
                .short('S')
                .long("sizesort")
                .overrides_with("timesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple_occurrences(true)
                .help("Sort by size"),
        )
        .arg(
            Arg::with_name("extensionsort")
                .short('X')
                .long("extensionsort")
                .overrides_with("sizesort")
                .overrides_with("timesort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple_occurrences(true)
                .help("Sort by file extension"),
        )
        .arg(
            Arg::with_name("versionsort")
                .short('v')
                .long("versionsort")
                .multiple_occurrences(true)
                .overrides_with("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .help("Natural sort of (version) numbers within text"),
        )
        .arg(
            Arg::with_name("sort")
                .long("sort")
                .multiple_occurrences(true)
                .possible_values(&["size", "time", "version", "extension", "none"])
                .takes_value(true)
                .value_name("WORD")
                .overrides_with("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("no-sort")
                .help("sort by WORD instead of name")
        )
        .arg(
            Arg::with_name("no-sort")
            .short('U')
            .long("no-sort")
            .multiple_occurrences(true)
            .overrides_with("timesort")
            .overrides_with("sizesort")
            .overrides_with("extensionsort")
            .overrides_with("sort")
            .overrides_with("versionsort")
            .help("Do not sort. List entries in directory order")
        )
        .arg(
            Arg::with_name("reverse")
                .short('r')
                .long("reverse")
                .multiple_occurrences(true)
                .help("Reverse the order of the sort"),
        )
        .arg(
            Arg::with_name("group-dirs")
                .long("group-dirs")
                .possible_value("none")
                .possible_value("first")
                .possible_value("last")
                .multiple_occurrences(true)
                .number_of_values(1)
                .help("Sort the directories then the files"),
        )
        .arg(
            Arg::with_name("group-directories-first")
                .long("group-directories-first")
                .help("Groups the directories at the top before the files. Same as --group-dirs=first")
        )
        .arg(
            Arg::new("blocks")
                .long("blocks")
                .action(ArgAction::Append)
                .multiple_values(true)
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .value_parser([
                    "permission",
                    "user",
                    "group",
                    "context",
                    "size",
                    "date",
                    "name",
                    "inode",
                    "links",
                ])
                .help("Specify the blocks that will be displayed and in what order"),
        )
        .arg(
            Arg::with_name("classic")
                .long("classic")
                .help("Enable classic mode (display output similar to ls)"),
        )
        .arg(
            Arg::with_name("no-symlink")
                .long("no-symlink")
                .multiple_occurrences(true)
                .help("Do not display symlink target"),
        )
        .arg(
            Arg::with_name("ignore-glob")
                .short('I')
                .long("ignore-glob")
                .multiple_occurrences(true)
                .number_of_values(1)
                .value_name("pattern")
                .default_value("")
                .help("Do not display files/directories with names matching the glob pattern(s). More than one can be specified by repeating the argument"),
        )
        .arg(
            Arg::with_name("inode")
                .short('i')
                .long("inode")
                .multiple_occurrences(true)
                .help("Display the index number of each file"),
        )
        .arg(
            Arg::new("dereference")
                .short('L')
                .long("dereference")
                .action(ArgAction::SetTrue)
                .help("When showing file information for a symbolic link, show information for the file the link references rather than for the link itself"),
        )
        .arg(
            Arg::with_name("context")
                .short('Z')
                .long("context")
                .required(false)
                .takes_value(false)
                .help("Print security context (label) of each file"),
        )
        .arg(
            Arg::with_name("hyperlink")
                .long("hyperlink")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("never")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .help("Attach hyperlink to filenames"),
        )
        .arg(
            Arg::with_name("header")
                .long("header")
                .help("Display block headers"),
        )
        .arg(
            Arg::new("system-protected")
                .long("system-protected")
                .action(ArgAction::SetTrue)
                .help("Includes files with the windows system protection flag set. This is the same as --all on other platforms")
                .hide(!cfg!(windows)),
        )
}

fn validate_date_argument(arg: &str) -> Result<String, String> {
    if arg.starts_with('+') {
        validate_time_format(arg)
    } else if arg == "date" || arg == "relative" {
        Result::Ok(arg.to_owned())
    } else {
        Result::Err("possible values: date, relative, +date-time-format".to_owned())
    }
}

pub fn validate_time_format(formatter: &str) -> Result<String, String> {
    let mut chars = formatter.chars();
    loop {
        match chars.next() {
            Some('%') => match chars.next() {
                Some('.') => match chars.next() {
                    Some('f') => (),
                    Some(n @ ('3' | '6' | '9')) => match chars.next() {
                        Some('f') => (),
                        Some(c) => return Err(format!("invalid format specifier: %.{}{}", n, c)),
                        None => return Err("missing format specifier".to_owned()),
                    },
                    Some(c) => return Err(format!("invalid format specifier: %.{}", c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ (':' | '#')) => match chars.next() {
                    Some('z') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ ('-' | '_' | '0')) => match chars.next() {
                    Some(
                        'C' | 'd' | 'e' | 'f' | 'G' | 'g' | 'H' | 'I' | 'j' | 'k' | 'l' | 'M' | 'm'
                        | 'S' | 's' | 'U' | 'u' | 'V' | 'W' | 'w' | 'Y' | 'y',
                    ) => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(
                    'A' | 'a' | 'B' | 'b' | 'C' | 'c' | 'D' | 'd' | 'e' | 'F' | 'f' | 'G' | 'g'
                    | 'H' | 'h' | 'I' | 'j' | 'k' | 'l' | 'M' | 'm' | 'n' | 'P' | 'p' | 'R' | 'r'
                    | 'S' | 's' | 'T' | 't' | 'U' | 'u' | 'V' | 'v' | 'W' | 'w' | 'X' | 'x' | 'Y'
                    | 'y' | 'Z' | 'z' | '+' | '%',
                ) => (),
                Some(n @ ('3' | '6' | '9')) => match chars.next() {
                    Some('f') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(c) => return Err(format!("invalid format specifier: %{}", c)),
                None => return Err("missing format specifier".to_owned()),
            },
            None => break,
            _ => continue,
        }
    }
    Ok(formatter.to_owned())
}
