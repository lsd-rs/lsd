---
title: lsd
section: 1
header: User Manual
footer: lsd <version>
date: <date>
---

# NAME

lsd - LSDeluxe

# SYNOPSIS

`lsd [FLAGS] [OPTIONS] [--] [FILE]...`

# DESCRIPTION

lsd is a ls command with a lot of pretty colours and some other stuff to enrich and enhance the directory listing experience.

# OPTIONS

`-a`, `--all`
: Do not ignore entries starting with **.**

`-A`, `--almost-all`
: Do not list implied **.** and **..**

`--classic`
: Enable classic mode (no colours or icons)

`-L`, `--dereference`
: When showing file information for a symbolic link, show information for the file the link references rather than for the link itself

`-d`, `--directory-only`
: Display directories themselves, and not their contents (recursively when used with --tree)

`-X`, `--extensionsort`
: Sort by file extension

`--git`
: Display git status. Directory git status is a reduction of included file statuses (recursively).

`--help`
: Prints help information

`-h`, `--human-readable`
: For ls compatibility purposes ONLY, currently set by default

`--ignore-config`
: Ignore the configuration file

`--config-file <path>`
: Provide the config file from a custom location

`-F`, `--classify`
: Append indicator (one of \*/=>@|) at the end of the file names

`-i`, `--inode`
: Display the index number of each file

`-l`, `--long`
: Display extended file metadata as a table

`--no-symlink`
: Do not display symlink target

`-1`, `--oneline`
: Display one entry per line

`-R`, `--recursive`
: Recurse into directories

`-r`, `--reverse`
: Reverse the order of the sort

`-S`, `--sizesort`
: Sort by size

`-t`, `--timesort`
: Sort by time modified

`--total-size`
: Display the total size of directories

`--tree`
: Recurse into directories and present the result as a tree

`-V`, `--version`
: Prints version information

`-v`, `--versionsort`
: Natural sort of (version) numbers within text

`--blocks <blocks>...`
: Specify the blocks that will be displayed and in what order [possible values: permission, user, group, size, date, name, inode, git]

`--color <color>...`
: When to use terminal colours [default: auto]  [possible values: always, auto, never]

`--date <date>...`
: How to display date [possible values: date, locale, relative, +date-time-format] [default: date]

`--depth <num>...`
: Stop recursing into directories after reaching specified depth

`--group-dirs <group-dirs>...`
: Sort the directories then the files [default: none]  [possible values: none, first, last]

`--group-directories-first`
: Groups the directories at the top before the files. Same as `--group-dirs=first`

`--hyperlink <hyperlink>...`
: Attach hyperlink to filenames [default: never]  [possible values: always, auto, never]

`--icon <icon>...`
: When to print the icons [default: auto]  [possible values: always, auto, never]

`--icon-theme <icon-theme>...`
: Whether to use fancy or unicode icons [default: fancy]  [possible values: fancy, unicode]

`-I, --ignore-glob <pattern>...`
: Do not display files/directories with names matching the glob pattern(s). More than one can be specified by repeating the argument [default: ]

`--permission <permission>...`
: How to display permissions [default: rwx for linux, attributes for windows]  [possible values: rwx, octal, attributes, disable]

`--size <size>...`
: How to display size [default: default]  [possible values: default, short, bytes]

`--sort <WORD>...`
: Sort by WORD instead of name [possible values: size, time, version, extension, git]

`-U`, `--no-sort`
: Do not sort. List entries in directory order

`-Z` `--context`
: Display SELinux or SMACK security context

`--header`
: Display block headers

`-N --literal`
: Print entry names without quoting

`--truncate-owner-after`
: Truncate the user and group names if they exceed a certain number of characters

`--truncate-owner-marker`
: Truncation marker appended to a truncated user or group name

# ARGS

`<FILE>...`
: A file or directory to list [default: .]

# EXAMPLES

`lsd`
: Display listing for current directory

`lsd /etc`
: Display listing of /etc

`lsd -la`
: Display listing of current directory, including files starting with `.` and the current directory's entry.

# ENVIRONMENT

`LS_COLORS`
: Used to determine color for displaying filenames. See **dir_colors**.

`XDG_CONFIG_HOME`
: Used to locate optional config file. If `XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/lsd/config.yaml` else `$HOME/.config/lsd/config.yaml`.

`SHELL_COMPLETIONS_DIR` or `OUT_DIR`
: Used to specify the directory for generating a shell completions file. If neither are set, no completions file will be generated. The directory will be created if it does not exist.
