# Place this file in the **completions** folder of the fish configuration(usually $HOME/.config/fish) folder
# See fish's documentation for more complete details
# https://fishshell.com/docs/current/cmds/complete.html

complete -c lsd -s '1' -l 'oneline' -d "Display one entry per line"
complete -c lsd -s 'a' -l 'all' -d "Do not ignore entries starting with ."
complete -c lsd -s 'A' -l 'almost-all' -d "Do not list implied . and .."


complete -c lsd -l 'blocks' -d "Specify the blocks that will be displayed and in what order" -x -a "permission user group context size date name inode links"
complete -c lsd -l 'classic' -d "Enable classic mode (display output similar to ls)"
complete -c lsd -l 'color' -d "When to use terminal colours [default: auto]" -x -a "always auto nerver"
complete -c lsd -l 'config-file' -d "Provide a custom lsd configuration file"

complete -c lsd -s 'd' -l 'directory-only' -d "Display directories themselves, and not their contents (recursively when used with --tree)"

complete -c lsd -l 'date' -d "How to display date [default: date]" -x -a "date relative +date-time-format"
complete -c lsd -l 'depth' -d "Stop recursing into directories after reaching specified depth" -x -a "1 2 3 4 5 6 7 8 9"

complete -c lsd -s 'F' -l 'classify' -d "Append indicator (one of */=>@|) at the end of the file names"

complete -c lsd -l 'group-directories-first' -d "Groups the directories at the top before the files. Same as --group-dirs=first"
complete -c lsd -l 'group-dirs' -d "Sort the directories then the files" -x -a 'none first last'

complete -c lsd -s 'h' -l 'human-readable' -d "For ls compatibility purposes ONLY, currently set by default"

complete -c lsd -l 'header' -d "Display block headers"
complete -c lsd -l 'help' -d "Print help information"
complete -c lsd -l 'hyperlink' -d "Attach hyperlink to filenames [default: never]" -x -a "always auto never"

complete -c lsd -s 'i' -l 'inode' -d "Display the index number of each file"
complete -c lsd -s 'I' -l 'ignore-glob' -d "
  Do not display files/directories with names matching the glob 
  pattern(s). More than one can be specified by repeating the argument [default: ]
"

complete -c lsd -l 'icon' -d "When to print the icons [default: auto]" -x -a "always auto never"
complete -c lsd -l 'icon-theme' -d "Whether to use fancy or unicode icons [default: fancy]" -x -a "fancy unicode"
complete -c lsd -l 'ignore-config' -d "Ignore the configuration file"

complete -c lsd -s 'l' -l 'long' -d "Display extended file metadata as a table"
complete -c lsd -s 'L' -l 'dereference' -d "
  When showing file information for a symbolic link, show
  information for the file the link references rather than for
  the link itself
"

complete -c lsd -l 'no-symlink' -d "Do not display symlink target"
complete -c lsd -l 'permission' -d "How to display permissions [default: rwx]" -x -a "rwx octal"

complete -c lsd -s 'r' -l 'reverse' -d "Reverse the order of the sort"
complete -c lsd -s 'R' -l 'recursive' -d "Recurse into directories"
complete -c lsd -s 'S' -l 'sizesort' -d "Sort by size"

complete -c lsd -l 'size' -d "How to display size [default: default]" -x -a "default short bytes"
complete -c lsd -l 'sort' -d "sort by WORD instead of name" -x -a "size time version extension none"

complete -c lsd -s 't' -l 'timesort' -d "Sort by time modified"
complete -c lsd -l 'total-size' -d "Display the total size of directories"
complete -c lsd -l 'tree' -d "Recurse into directories and present the result as a tree"


complete -c lsd -s 'U' -l 'no-sort' -d "Do not sort. List entries in directory order"
complete -c lsd -s 'v' -l 'versionsort' -d "Natural sort of (version) numbers within text"
complete -c lsd -s 'V' -l 'version' -d "Print version information"
complete -c lsd -s 'X' -l 'extensionsort' -d "Sort by file extension"
complete -c lsd -s 'Z' -l 'context' -d "Print security context (label) of each file"
