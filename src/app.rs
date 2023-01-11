use clap::{Arg, ArgAction, Command, ValueHint};

pub fn build() -> Command<'static> {
    Command::new("lsd")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("FILE")
                .action(ArgAction::Append)
                .multiple_values(true)
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
            Arg::new("icon")
                .long("icon")
                .value_parser(["always", "auto", "never"])
                .default_value("auto")
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("When to print the icons"),
        )
        .arg(
            Arg::new("icon-theme")
                .long("icon-theme")
                .default_value("fancy")
                .value_parser(["fancy", "unicode"])
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("Whether to use fancy or unicode icons"),
        )
        .arg(
            Arg::new("indicators")
                .short('F')
                .long("classify")
                .action(ArgAction::SetTrue)
                .help("Append indicator (one of */=>@|) at the end of the file names"),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .action(ArgAction::SetTrue)
                .help("Display extended file metadata as a table"),
        )
        .arg(
            Arg::new("ignore-config")
                .long("ignore-config")
                .action(ArgAction::SetTrue)
                .help("Ignore the configuration file"),
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .help("Provide a custom lsd configuration file")
                .value_name("config-file")
                .takes_value(true)
        )
        .arg(
            Arg::new("oneline")
                .short('1')
                .long("oneline")
                .action(ArgAction::SetTrue)
                .help("Display one entry per line"),
        )
        .arg(
            Arg::new("recursive")
                .short('R')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .conflicts_with("tree")
                .help("Recurse into directories"),
        )
        .arg(
            Arg::new("human_readable")
                .short('h')
                .long("human-readable")
                .action(ArgAction::SetTrue)
                .help("For ls compatibility purposes ONLY, currently set by default"),
        )
        .arg(
            Arg::new("tree")
                .long("tree")
                .action(ArgAction::SetTrue)
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
            Arg::new("permission")
                .long("permission")
                .default_value("rwx")
                .value_parser(["rwx", "octal"])
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("How to display permissions"),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .value_parser(["default", "short", "bytes"])
                .default_value("default")
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("How to display size"),
        )
        .arg(
            Arg::new("total-size")
                .long("total-size")
                .action(ArgAction::SetTrue)
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
            Arg::new("timesort")
                .short('t')
                .long("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .action(ArgAction::SetTrue)
                .help("Sort by time modified"),
        )
        .arg(
            Arg::new("sizesort")
                .short('S')
                .long("sizesort")
                .overrides_with("timesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .action(ArgAction::SetTrue)
                .help("Sort by size"),
        )
        .arg(
            Arg::new("extensionsort")
                .short('X')
                .long("extensionsort")
                .overrides_with("sizesort")
                .overrides_with("timesort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .action(ArgAction::SetTrue)
                .help("Sort by file extension"),
        )
        .arg(
            Arg::new("versionsort")
                .short('v')
                .long("versionsort")
                .action(ArgAction::SetTrue)
                .overrides_with("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .help("Natural sort of (version) numbers within text"),
        )
        .arg(
            Arg::new("sort")
                .long("sort")
                .action(ArgAction::Append)
                .value_parser(["size", "time", "version", "extension", "none"])
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
            Arg::new("no-sort")
            .short('U')
            .long("no-sort")
            .action(ArgAction::SetTrue)
            .overrides_with("timesort")
            .overrides_with("sizesort")
            .overrides_with("extensionsort")
            .overrides_with("sort")
            .overrides_with("versionsort")
            .help("Do not sort. List entries in directory order")
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .action(ArgAction::SetTrue)
                .help("Reverse the order of the sort"),
        )
        .arg(
            Arg::new("group-dirs")
                .long("group-dirs")
                .value_parser(["none", "first", "last"])
                .action(ArgAction::Append)
                .number_of_values(1)
                .help("Sort the directories then the files"),
        )
        .arg(
            Arg::new("group-directories-first")
                .long("group-directories-first")
                .action(ArgAction::SetTrue)
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
            Arg::new("classic")
                .long("classic")
                .action(ArgAction::SetTrue)
                .help("Enable classic mode (display output similar to ls)"),
        )
        .arg(
            Arg::new("no-symlink")
                .long("no-symlink")
                .action(ArgAction::SetTrue)
                .help("Do not display symlink target"),
        )
        .arg(
            Arg::new("ignore-glob")
                .short('I')
                .long("ignore-glob")
                .action(ArgAction::Append)
                .number_of_values(1)
                .value_name("pattern")
                .default_value("")
                .help("Do not display files/directories with names matching the glob pattern(s). More than one can be specified by repeating the argument"),
        )
        .arg(
            Arg::new("inode")
                .short('i')
                .long("inode")
                .action(ArgAction::SetTrue)
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
            Arg::new("context")
                .short('Z')
                .long("context")
                .required(false)
                .takes_value(false)
                .action(ArgAction::SetTrue)
                .help("Print security context (label) of each file"),
        )
        .arg(
            Arg::new("hyperlink")
                .long("hyperlink")
                .value_parser(["always", "auto", "never"])
                .default_value("never")
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("Attach hyperlink to filenames"),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .action(ArgAction::SetTrue)
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
