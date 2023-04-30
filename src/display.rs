use crate::color::{Colors, Elem};
use crate::flags::blocks::Block;
use crate::flags::{Display, Flags, HyperlinkOption, Layout};
use crate::git_theme::GitTheme;
use crate::icon::Icons;
use crate::meta::name::DisplayOption;
use crate::meta::{FileType, Meta};
use std::collections::HashMap;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "│  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"
const BLANK: &str = "   ";

pub fn grid(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    git_theme: &GitTheme,
) -> String {
    let term_width = terminal_size().map(|(w, _)| w.0 as usize);

    inner_display_grid(
        &DisplayOption::None,
        metas,
        flags,
        colors,
        icons,
        git_theme,
        0,
        term_width,
    )
}

pub fn tree(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    git_theme: &GitTheme,
) -> String {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    let padding_rules = get_padding_rules(metas, flags);
    let mut index = 0;
    for (i, block) in flags.blocks.0.iter().enumerate() {
        if block == &Block::Name {
            index = i;
            break;
        }
    }

    for cell in inner_display_tree(
        metas,
        flags,
        colors,
        icons,
        git_theme,
        (0, ""),
        &padding_rules,
        index,
    ) {
        grid.add(cell);
    }

    grid.fit_into_columns(flags.blocks.0.len()).to_string()
}

#[allow(clippy::too_many_arguments)] // should wrap flags, colors, icons, git_theme into one struct
fn inner_display_grid(
    display_option: &DisplayOption,
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    git_theme: &GitTheme,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();
    let mut cells = Vec::new();

    let padding_rules = get_padding_rules(metas, flags);
    let mut grid = match flags.layout {
        Layout::OneLine => Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        }),
        _ => Grid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom,
        }),
    };

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DirectoryOnly);

    // print the files first.
    for meta in metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if skip_dirs
            && (matches!(meta.file_type, FileType::Directory { .. })
                || (matches!(meta.file_type, FileType::SymLink { is_dir: true })
                    && flags.layout != Layout::OneLine))
        {
            continue;
        }

        let blocks = get_output(
            meta,
            colors,
            icons,
            git_theme,
            flags,
            display_option,
            &padding_rules,
            (0, ""),
        );

        for block in blocks {
            cells.push(Cell {
                width: get_visible_width(&block, flags.hyperlink == HyperlinkOption::Always),
                contents: block,
            });
        }
    }

    // Print block headers
    if flags.header.0 && flags.layout == Layout::OneLine && !cells.is_empty() {
        add_header(flags, &cells, &mut grid);
    }

    for cell in cells {
        grid.add(cell);
    }

    if flags.layout == Layout::Grid {
        if let Some(tw) = term_width {
            if let Some(gridded_output) = grid.fit_into_width(tw) {
                output += &gridded_output.to_string();
            } else {
                //does not fit into grid, usually because (some) filename(s)
                //are longer or almost as long as term_width
                //print line by line instead!
                output += &grid.fit_into_columns(1).to_string();
            }
        } else {
            output += &grid.fit_into_columns(1).to_string();
        }
    } else {
        output += &grid.fit_into_columns(flags.blocks.0.len()).to_string();
    }

    let should_display_folder_path = should_display_folder_path(depth, metas, flags);

    // print the folder content
    for meta in metas {
        if let Some(content) = &meta.content {
            if should_display_folder_path {
                output += &display_folder_path(meta);
            }

            let display_option = DisplayOption::Relative {
                base_path: &meta.path,
            };

            output += &inner_display_grid(
                &display_option,
                content,
                flags,
                colors,
                icons,
                git_theme,
                depth + 1,
                term_width,
            );
        }
    }

    output
}

fn add_header(flags: &Flags, cells: &[Cell], grid: &mut Grid) {
    let num_columns: usize = flags.blocks.0.len();

    let mut widths = flags
        .blocks
        .0
        .iter()
        .map(|b| get_visible_width(b.get_header(), flags.hyperlink == HyperlinkOption::Always))
        .collect::<Vec<usize>>();

    // find max widths of each column
    for (index, cell) in cells.iter().enumerate() {
        let index = index % num_columns;
        widths[index] = std::cmp::max(widths[index], cell.width);
    }

    for (idx, block) in flags.blocks.0.iter().enumerate() {
        // center and underline header
        let underlined_header = crossterm::style::Stylize::attribute(
            format!("{: ^1$}", block.get_header(), widths[idx]),
            crossterm::style::Attribute::Underlined,
        )
        .to_string();

        grid.add(Cell {
            width: widths[idx],
            contents: underlined_header,
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn inner_display_tree(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    git_theme: &GitTheme,
    tree_depth_prefix: (usize, &str),
    padding_rules: &HashMap<Block, usize>,
    tree_index: usize,
) -> Vec<Cell> {
    let mut cells = Vec::new();
    let last_idx = metas.len();

    for (idx, meta) in metas.iter().enumerate() {
        let current_prefix = if tree_depth_prefix.0 > 0 {
            if idx + 1 != last_idx {
                // is last folder elem
                format!("{}{} ", tree_depth_prefix.1, EDGE)
            } else {
                format!("{}{} ", tree_depth_prefix.1, CORNER)
            }
        } else {
            tree_depth_prefix.1.to_string()
        };

        for block in get_output(
            meta,
            colors,
            icons,
            git_theme,
            flags,
            &DisplayOption::FileName,
            padding_rules,
            (tree_index, &current_prefix),
        ) {
            cells.push(Cell {
                width: get_visible_width(&block, flags.hyperlink == HyperlinkOption::Always),
                contents: block,
            });
        }

        if let Some(content) = &meta.content {
            let new_prefix = if tree_depth_prefix.0 > 0 {
                if idx + 1 != last_idx {
                    // is last folder elem
                    format!("{}{} ", tree_depth_prefix.1, LINE)
                } else {
                    format!("{}{} ", tree_depth_prefix.1, BLANK)
                }
            } else {
                tree_depth_prefix.1.to_string()
            };

            cells.extend(inner_display_tree(
                content,
                flags,
                colors,
                icons,
                git_theme,
                (tree_depth_prefix.0 + 1, &new_prefix),
                padding_rules,
                tree_index,
            ));
        }
    }

    cells
}

fn should_display_folder_path(depth: usize, metas: &[Meta], flags: &Flags) -> bool {
    if depth > 0 {
        true
    } else {
        let folder_number = metas
            .iter()
            .filter(|x| {
                matches!(x.file_type, FileType::Directory { .. })
                    || (matches!(x.file_type, FileType::SymLink { is_dir: true })
                        && flags.layout != Layout::OneLine)
            })
            .count();

        folder_number > 1 || folder_number < metas.len()
    }
}

fn display_folder_path(meta: &Meta) -> String {
    format!("\n{}:\n", meta.path.to_string_lossy())
}

#[allow(clippy::too_many_arguments)]
fn get_output(
    meta: &Meta,
    colors: &Colors,
    icons: &Icons,
    git_theme: &GitTheme,
    flags: &Flags,
    display_option: &DisplayOption,
    padding_rules: &HashMap<Block, usize>,
    tree: (usize, &str),
) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    let colorize_missing = |string: &str| colors.colorize(string, &Elem::NoAccess);

    for (i, block) in flags.blocks.0.iter().enumerate() {
        let mut block_vec = if Layout::Tree == flags.layout && tree.0 == i {
            vec![colors.colorize(tree.1, &Elem::TreeEdge)]
        } else {
            Vec::new()
        };

        match block {
            Block::INode => block_vec.push(match &meta.inode {
                Some(inode) => inode.render(colors),
                None => colorize_missing("?"),
            }),
            Block::Links => block_vec.push(match &meta.links {
                Some(links) => links.render(colors),
                None => colorize_missing("?"),
            }),
            Block::Permission => {
                block_vec.extend([
                    meta.file_type.render(colors),
                    match meta.permissions {
                        Some(permissions) => permissions.render(colors, flags),
                        None => colorize_missing("?????????"),
                    },
                    match &meta.access_control {
                        Some(access_control) => access_control.render_method(colors),
                        None => colorize_missing(""),
                    },
                ]);
            }
            Block::User => block_vec.push(match &meta.owner {
                Some(owner) => owner.render_user(colors),
                None => colorize_missing("?"),
            }),
            Block::Group => block_vec.push(match &meta.owner {
                Some(owner) => owner.render_group(colors),
                None => colorize_missing("?"),
            }),
            Block::Context => block_vec.push(match &meta.access_control {
                Some(access_control) => access_control.render_context(colors),
                None => colorize_missing("?"),
            }),
            Block::Size => {
                let pad = if Layout::Tree == flags.layout && 0 == tree.0 && 0 == i {
                    None
                } else {
                    Some(padding_rules[&Block::SizeValue])
                };
                block_vec.push(match &meta.size {
                    Some(size) => size.render(colors, flags, pad),
                    None => colorize_missing("?"),
                })
            }
            Block::SizeValue => block_vec.push(match &meta.size {
                Some(size) => size.render_value(colors, flags),
                None => colorize_missing("?"),
            }),
            Block::Date => block_vec.push(match &meta.date {
                Some(date) => date.render(colors, flags),
                None => colorize_missing("?"),
            }),
            Block::Name => {
                block_vec.extend([
                    meta.name.render(
                        colors,
                        icons,
                        display_option,
                        flags.hyperlink,
                        flags.should_quote,
                    ),
                    meta.indicator.render(flags),
                ]);
                if !(flags.no_symlink.0 || flags.dereference.0 || flags.layout == Layout::Grid) {
                    block_vec.push(meta.symlink.render(colors, flags))
                }
            }
            Block::GitStatus => {
                if let Some(_s) = &meta.git_status {
                    block_vec.push(_s.render(colors, git_theme));
                }
            }
        };
        strings.push(
            block_vec
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(""),
        );
    }
    strings
}

fn get_visible_width(input: &str, hyperlink: bool) -> usize {
    let mut nb_invisible_char = 0;

    // If the input has color, do not compute the length contributed by the color to the actual length
    for (idx, _) in input.match_indices("\u{1b}[") {
        let (_, s) = input.split_at(idx);

        let m_pos = s.find('m');
        if let Some(len) = m_pos {
            nb_invisible_char += len
        }
    }

    if hyperlink {
        for (idx, _) in input.match_indices("\x1B]8;;") {
            let (_, s) = input.split_at(idx);

            let m_pos = s.find("\x1B\x5C");
            if let Some(len) = m_pos {
                nb_invisible_char += len
            }
        }
    }

    UnicodeWidthStr::width(input) - nb_invisible_char
}

fn detect_size_lengths(metas: &[Meta], flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let value_len = match &meta.size {
            Some(size) => size.value_string(flags).len(),
            None => 0,
        };

        if value_len > max_value_length {
            max_value_length = value_len;
        }

        if Layout::Tree == flags.layout {
            if let Some(subs) = &meta.content {
                let sub_length = detect_size_lengths(subs, flags);
                if sub_length > max_value_length {
                    max_value_length = sub_length;
                }
            }
        }
    }

    max_value_length
}

fn get_padding_rules(metas: &[Meta], flags: &Flags) -> HashMap<Block, usize> {
    let mut padding_rules: HashMap<Block, usize> = HashMap::new();

    if flags.blocks.0.contains(&Block::Size) {
        let size_val = detect_size_lengths(metas, flags);

        padding_rules.insert(Block::SizeValue, size_val);
    }

    padding_rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Cli;
    use crate::color;
    use crate::color::Colors;
    use crate::flags::{HyperlinkOption, IconOption, IconTheme as FlagTheme};
    use crate::icon::Icons;
    use crate::meta::{FileType, Name};
    use crate::Config;
    use crate::{flags, sort};
    use assert_fs::prelude::*;
    use clap::Parser;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_display_get_visible_width_without_icons() {
        for (s, l) in [
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 28),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                    true,
                )
                .to_string();

            assert_eq!(get_visible_width(&output, false), l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        for (s, l) in [
            // Add 3 characters for the icons.
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 24),
            ("ASCII1234-_", 13),
            ("File with space", 19),
            ("制作样本。", 12),
            ("日本語", 8),
            ("샘플은 무료로 드리겠습니다", 30),
            ("👩🐩", 6),
            ("🔬", 4),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                    true,
                )
                .to_string();

            assert_eq!(get_visible_width(&output, false), l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_colors() {
        for (s, l) in [
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 17),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 28),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoLscolors),
                    &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                    true,
                )
                .to_string();

            // check if the color is present.
            assert!(
                output.starts_with("\u{1b}[38;5;"),
                "{output:?} should start with color"
            );
            assert!(output.ends_with("[39m"), "reset foreground color");

            assert_eq!(get_visible_width(&output, false), l, "visible match");
        }
    }

    #[test]
    fn test_display_get_visible_width_without_colors() {
        for (s, l) in [
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 17),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 28),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                    true,
                )
                .to_string();

            // check if the color is present.
            assert!(!output.starts_with("\u{1b}[38;5;"));
            assert!(!output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output, false), l);
        }
    }

    #[test]
    fn test_display_get_visible_width_hypelink_simple() {
        for (s, l) in [
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            // rending name require actual file, so we are mocking that
            let output = format!("\x1B]8;;{}\x1B\x5C{}\x1B]8;;\x1B\x5C", "url://fake-url", s);
            assert_eq!(get_visible_width(&output, true), l);
        }
    }

    fn sort(metas: &mut Vec<Meta>, sorters: &Vec<(flags::SortOrder, sort::SortFn)>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(sorters, a, b));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                sort(content, sorters);
            }
        }
    }

    #[test]
    fn test_display_tree_with_all() {
        let argv = ["lsd", "--tree", "--all"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        dir.child("one.d/.hidden").touch().unwrap();
        let mut metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags, None)
            .unwrap()
            .0
            .unwrap();
        sort(&mut metas, &sort::assemble_sorters(&flags));
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        assert_eq!("one.d\n├── .hidden\n└── two\n", output);
    }

    /// Different level of folder may form a different width
    /// we must make sure it is aligned in all level
    ///
    /// dir has a bytes size
    /// empty file has an empty size
    /// `---blocks size,name` can help us for this case
    #[test]
    fn test_tree_align_subfolder() {
        let argv = ["lsd", "--tree", "--blocks", "size,name"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("dir").create_dir_all().unwrap();
        dir.child("dir/file").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags, None)
            .unwrap()
            .0
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        let length_before_b = |i| -> usize {
            output
                .lines()
                .nth(i)
                .unwrap()
                .split(|c| c == 'K' || c == 'B')
                .next()
                .unwrap()
                .len()
        };
        assert_eq!(length_before_b(0), length_before_b(1));
        assert_eq!(
            output.lines().next().unwrap().find('d'),
            output.lines().nth(1).unwrap().find('└')
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_tree_size_first_without_name() {
        let argv = ["lsd", "--tree", "--blocks", "size,permission"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("dir").create_dir_all().unwrap();
        dir.child("dir/file").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags, None)
            .unwrap()
            .0
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        assert_eq!(output.lines().nth(1).unwrap().chars().next().unwrap(), '└');
        assert_eq!(
            output
                .lines()
                .next()
                .unwrap()
                .chars()
                .position(|x| x == 'd'),
            output
                .lines()
                .nth(1)
                .unwrap()
                .chars()
                .position(|x| x == '.'),
        );
    }

    #[test]
    fn test_tree_edge_before_name() {
        let argv = ["lsd", "--tree", "--long"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags, None)
            .unwrap()
            .0
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        assert!(output.ends_with("└── two\n"));
    }

    #[test]
    fn test_grid_all_block_headers() {
        let argv = [
            "lsd",
            "--header",
            "--blocks",
            "permission,user,group,size,date,name,inode,links",
        ];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("testdir").create_dir_all().unwrap();
        dir.child("test").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(1, &flags, None)
            .unwrap()
            .0
            .unwrap();
        let output = grid(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        dir.close().unwrap();

        assert!(output.contains("Permissions"));
        assert!(output.contains("User"));
        assert!(output.contains("Group"));
        assert!(output.contains("Size"));
        assert!(output.contains("Date Modified"));
        assert!(output.contains("Name"));
        assert!(output.contains("INode"));
        assert!(output.contains("Links"));
    }

    #[test]
    fn test_grid_no_header_with_empty_meta() {
        let argv = ["lsd", "--header", "-l"];
        let cli = Cli::try_parse_from(argv).unwrap();
        let flags = Flags::configure_from(&cli, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("testdir").create_dir_all().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(1, &flags, None)
            .unwrap()
            .0
            .unwrap();
        let output = grid(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string()),
            &GitTheme::new(),
        );

        dir.close().unwrap();

        assert!(!output.contains("Permissions"));
        assert!(!output.contains("User"));
        assert!(!output.contains("Group"));
        assert!(!output.contains("Size"));
        assert!(!output.contains("Date Modified"));
        assert!(!output.contains("Name"));
    }

    #[test]
    fn test_folder_path() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let file_path = tmp_dir.path().join("file");
        std::fs::File::create(&file_path).expect("failed to create the file");
        let file = Meta::from_path(&file_path, false).unwrap();

        let dir_path = tmp_dir.path().join("dir");
        std::fs::create_dir(&dir_path).expect("failed to create the dir");
        let dir = Meta::from_path(&dir_path, false).unwrap();

        assert_eq!(
            display_folder_path(&dir),
            format!(
                "\n{}{}dir:\n",
                tmp_dir.path().to_string_lossy(),
                std::path::MAIN_SEPARATOR
            )
        );

        const YES: bool = true;
        const NO: bool = false;

        assert_eq!(
            should_display_folder_path(0, &[file.clone()], &Flags::default()),
            YES // doesn't matter since there is no folder
        );
        assert_eq!(
            should_display_folder_path(0, &[dir.clone()], &Flags::default()),
            NO
        );
        assert_eq!(
            should_display_folder_path(0, &[file.clone(), dir.clone()], &Flags::default()),
            YES
        );
        assert_eq!(
            should_display_folder_path(0, &[dir.clone(), dir.clone()], &Flags::default()),
            YES
        );
        assert_eq!(
            should_display_folder_path(0, &[file.clone(), file.clone()], &Flags::default()),
            YES // doesn't matter since there is no folder
        );

        drop(dir); // to avoid clippy complains about previous .clone()
        drop(file);
    }

    #[cfg(unix)]
    #[test]
    fn test_folder_path_with_links() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let file_path = tmp_dir.path().join("file");
        std::fs::File::create(&file_path).expect("failed to create the file");
        let file = Meta::from_path(&file_path, false).unwrap();

        let dir_path = tmp_dir.path().join("dir");
        std::fs::create_dir(&dir_path).expect("failed to create the dir");
        let dir = Meta::from_path(&dir_path, false).unwrap();

        let link_path = tmp_dir.path().join("link");
        std::os::unix::fs::symlink("dir", &link_path).unwrap();
        let link = Meta::from_path(&link_path, false).unwrap();

        let grid_flags = Flags {
            layout: Layout::Grid,
            ..Flags::default()
        };

        let oneline_flags = Flags {
            layout: Layout::OneLine,
            ..Flags::default()
        };

        const YES: bool = true;
        const NO: bool = false;

        assert_eq!(
            should_display_folder_path(0, &[link.clone()], &grid_flags),
            NO
        );
        assert_eq!(
            should_display_folder_path(0, &[link.clone()], &oneline_flags),
            YES // doesn't matter since this link will be expanded as a directory
        );

        assert_eq!(
            should_display_folder_path(0, &[file.clone(), link.clone()], &grid_flags),
            YES
        );
        assert_eq!(
            should_display_folder_path(0, &[file.clone(), link.clone()], &oneline_flags),
            YES // doesn't matter since this link will be expanded as a directory
        );

        assert_eq!(
            should_display_folder_path(0, &[dir.clone(), link.clone()], &grid_flags),
            YES
        );
        assert_eq!(
            should_display_folder_path(0, &[dir.clone(), link.clone()], &oneline_flags),
            YES
        );

        drop(dir); // to avoid clippy complains about previous .clone()
        drop(file);
        drop(link);
    }
}
