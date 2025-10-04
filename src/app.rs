use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(about, version, args_override_self = true, disable_help_flag = true)]
pub struct Cli {
    #[arg(value_name = "FILE", default_value = ".", value_hint = ValueHint::AnyPath)]
    pub inputs: Vec<PathBuf>,

    /// Do not ignore entries starting with . .
    #[arg(short, long, overrides_with = "almost_all")]
    pub all: bool,

    /// Do not list implied . and ..
    #[arg(short = 'A', long)]
    pub almost_all: bool,

    /// When to use terminal colours [default: auto]
    #[arg(long, value_name = "MODE", value_parser = ["always", "auto", "never"])]
    pub color: Option<String>,

    /// When to print the icons [default: auto]
    #[arg(long, value_name = "MODE", value_parser = ["always", "auto", "never"])]
    pub icon: Option<String>,

    /// Whether to use fancy or unicode icons [default: fancy]
    #[arg(long, value_name = "THEME", value_parser = ["fancy", "unicode"])]
    pub icon_theme: Option<String>,

    /// Append indicator (one of */=>@|) at the end of the file names
    #[arg(short = 'F', long = "classify")]
    pub indicators: bool,

    /// Display extended file metadata as a table
    #[arg(short, long)]
    pub long: bool,

    /// Ignore the configuration file
    #[arg(long)]
    pub ignore_config: bool,

    /// Provide a custom lsd configuration file
    #[arg(long, value_name = "PATH")]
    pub config_file: Option<PathBuf>,

    /// Display one entry per line
    #[arg(short = '1', long)]
    pub oneline: bool,

    /// Recurse into directories
    #[arg(short = 'R', long, conflicts_with = "tree")]
    pub recursive: bool,

    /// For ls compatibility purposes ONLY, currently set by default
    #[arg(short, long)]
    human_readable: bool,

    /// Recurse into directories and present the result as a tree
    #[arg(long)]
    pub tree: bool,

    /// Stop recursing into directories after reaching specified depth
    #[arg(long, value_name = "NUM")]
    pub depth: Option<usize>,

    /// Display directories themselves, and not their contents (recursively when used with --tree)
    #[arg(short, long, conflicts_with = "recursive")]
    pub directory_only: bool,

    /// How to display permissions [default: rwx for linux, attributes for windows]
    #[arg(long, value_name = "MODE", value_parser = ["rwx", "octal", "attributes", "disable"])]
    pub permission: Option<String>,

    /// How to display size [default: default]
    #[arg(long, value_name = "MODE", value_parser = ["default", "short", "bytes"])]
    pub size: Option<String>,

    /// Display the total size of directories
    #[arg(long)]
    pub total_size: bool,

    /// How to display date [default: date] [possible values: date, locale, relative, +date-time-format]
    #[arg(long, value_parser = validate_date_argument)]
    pub date: Option<String>,

    /// Sort by time modified
    #[arg(short = 't', long)]
    pub timesort: bool,

    /// Sort by size
    #[arg(short = 'S', long)]
    pub sizesort: bool,

    /// Sort by file extension
    #[arg(short = 'X', long)]
    pub extensionsort: bool,

    /// Sort by git status
    #[arg(short = 'G', long)]
    pub gitsort: bool,

    /// Natural sort of (version) numbers within text
    #[arg(short = 'v', long)]
    pub versionsort: bool,

    /// Sort by TYPE instead of name
    #[arg(
        long,
        value_name = "TYPE",
        value_parser = ["size", "time", "version", "extension", "git", "none"],
        overrides_with_all = ["timesort", "sizesort", "extensionsort", "versionsort", "gitsort", "no_sort"]
    )]
    pub sort: Option<String>,

    /// Do not sort. List entries in directory order
    #[arg(short = 'U', long, overrides_with_all = ["timesort", "sizesort", "extensionsort", "versionsort", "gitsort", "sort"])]
    pub no_sort: bool,

    /// Reverse the order of the sort
    #[arg(short, long)]
    pub reverse: bool,

    /// Sort the directories then the files
    #[arg(long, value_name = "MODE", value_parser = ["none", "first", "last"])]
    pub group_dirs: Option<String>,

    /// Groups the directories at the top before the files. Same as --group-dirs=first
    #[arg(long)]
    pub group_directories_first: bool,

    /// Specify the blocks that will be displayed and in what order
    #[arg(
    long,
    value_delimiter = ',',
    value_parser = ["permission", "user", "group", "context", "size", "date", "name", "inode", "links", "git"],
    )]
    pub blocks: Vec<String>,

    /// Enable classic mode (display output similar to ls)
    #[arg(long)]
    pub classic: bool,

    /// Do not display symlink target
    #[arg(long)]
    pub no_symlink: bool,

    /// Do not display files/directories with names matching the glob pattern(s).
    /// More than one can be specified by repeating the argument
    #[arg(short = 'I', long, value_name = "PATTERN")]
    pub ignore_glob: Vec<String>,

    /// Display the index number of each file
    #[arg(short, long)]
    pub inode: bool,

    /// Show git status on file and directory"
    /// Only when used with --long option
    #[arg(short, long)]
    pub git: bool,

    /// When showing file information for a symbolic link,
    /// show information for the file the link references rather than for the link itself
    #[arg(short = 'L', long)]
    pub dereference: bool,

    /// Print security context (label) of each file
    #[arg(short = 'Z', long)]
    pub context: bool,

    /// Attach hyperlink to filenames [default: never]
    #[arg(long, value_name = "MODE", value_parser = ["always", "auto", "never"])]
    pub hyperlink: Option<String>,

    /// Display block headers
    #[arg(long)]
    pub header: bool,

    /// Truncate the user and group names if they exceed a certain number of characters
    #[arg(long, value_name = "NUM")]
    pub truncate_owner_after: Option<usize>,

    /// Truncation marker appended to a truncated user or group name
    #[arg(long, value_name = "STR")]
    pub truncate_owner_marker: Option<String>,

    /// Includes files with the windows system protection flag set.
    /// This is the same as --all on other platforms
    #[arg(long, hide = !cfg!(windows))]
    pub system_protected: bool,

    /// Print entry names without quoting
    #[arg(short = 'N', long)]
    pub literal: bool,

    /// Print help information
    #[arg(long, action = ArgAction::Help)]
    help: (),
}

fn validate_date_argument(arg: &str) -> Result<String, String> {
    if arg.starts_with('+') {
        validate_time_format(arg)
    } else if arg == "date" || arg == "relative" || arg == "locale" {
        Result::Ok(arg.to_owned())
    } else {
        Result::Err("possible values: date, locale, relative, +date-time-format".to_owned())
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
                        Some(c) => return Err(format!("invalid format specifier: %.{n}{c}")),
                        None => return Err("missing format specifier".to_owned()),
                    },
                    Some(c) => return Err(format!("invalid format specifier: %.{c}")),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ (':' | '#')) => match chars.next() {
                    Some('z') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{n}{c}")),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ ('-' | '_' | '0')) => match chars.next() {
                    Some(
                        'C' | 'd' | 'e' | 'f' | 'G' | 'g' | 'H' | 'I' | 'j' | 'k' | 'l' | 'M' | 'm'
                        | 'S' | 's' | 'U' | 'u' | 'V' | 'W' | 'w' | 'Y' | 'y',
                    ) => (),
                    Some(c) => return Err(format!("invalid format specifier: %{n}{c}")),
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
                    Some(c) => return Err(format!("invalid format specifier: %{n}{c}")),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(c) => return Err(format!("invalid format specifier: %{c}")),
                None => return Err("missing format specifier".to_owned()),
            },
            None => break,
            _ => continue,
        }
    }
    Ok(formatter.to_owned())
}

// Wrapper for value_parser to simply remove non supported option (mainly git flag)
// required since value_parser requires impl Into<ValueParser> that Vec do not support
// should be located here, since this file is included by build.rs
struct LabelFilter<Filter: Fn(&'static str) -> bool, const C: usize>([&'static str; C], Filter);

impl<Filter: Fn(&'static str) -> bool, const C: usize> From<LabelFilter<Filter, C>>
    for clap::builder::ValueParser
{
    fn from(label_filter: LabelFilter<Filter, C>) -> Self {
        let filter = label_filter.1;
        let values = label_filter.0.into_iter().filter(|x| filter(x));
        let inner = clap::builder::PossibleValuesParser::from(values);
        Self::from(inner)
    }
}
