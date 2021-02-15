use crate::color::{ColoredString, Colors};
use crate::flags::{Block, Display, Flags, Layout};
use crate::icon::Icons;
use crate::meta::name::DisplayOption;
use crate::meta::{FileType, Meta};
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "│  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"
const BLANK: &str = "   ";

pub fn grid(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    let term_width = match terminal_size() {
        Some((w, _)) => Some(w.0 as usize),
        None => None,
    };

    inner_display_grid(
        &DisplayOption::None,
        metas,
        &flags,
        colors,
        icons,
        0,
        term_width,
    )
}

pub fn tree(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_tree(metas, &flags, colors, icons, 0, "")
}

fn inner_display_grid(
    display_option: &DisplayOption,
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(&metas, flags);
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
            && (matches!(meta.file_type, FileType::Directory{..})
                || (matches!(meta.file_type, FileType::SymLink { is_dir: true })
                    && flags.layout != Layout::OneLine))
        {
            continue;
        }

        let blocks = get_output(
            &meta,
            &colors,
            &icons,
            &flags,
            &display_option,
            &padding_rules,
            "",
        );

        for block in blocks {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }
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

    let should_display_folder_path = should_display_folder_path(depth, &metas, &flags);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(&meta);
            }

            let display_option = DisplayOption::Relative {
                base_path: &meta.path,
            };

            output += &inner_display_grid(
                &display_option,
                meta.content.as_ref().unwrap(),
                &flags,
                colors,
                icons,
                depth + 1,
                term_width,
            );
        }
    }

    output
}

fn inner_display_tree(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    prefix: &str,
) -> String {
    let mut output = String::new();
    let last_idx = metas.len();

    let padding_rules = get_padding_rules(&metas, flags);

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for (idx, meta) in metas.iter().enumerate() {
        let current_prefix = if depth > 0 {
            if idx + 1 != last_idx {
                // is last folder elem
                format!("{}{} ", prefix, EDGE)
            } else {
                format!("{}{} ", prefix, CORNER)
            }
        } else {
            prefix.to_string()
        };

        for block in get_output(
            &meta,
            &colors,
            &icons,
            &flags,
            &DisplayOption::FileName,
            &padding_rules,
            &current_prefix,
        ) {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }
    }

    let content = grid.fit_into_columns(flags.blocks.0.len()).to_string();
    let mut lines = content.lines();

    for (idx, meta) in metas.iter().enumerate() {
        output += &String::from(lines.next().unwrap());
        output += "\n";

        if meta.content.is_some() {
            let new_prefix = if depth > 0 {
                if idx + 1 != last_idx {
                    // is last folder elem
                    format!("{}{}", prefix, LINE)
                } else {
                    format!("{}{}", prefix, BLANK)
                }
            } else {
                prefix.to_string()
            };

            output += &inner_display_tree(
                &meta.content.as_ref().unwrap(),
                &flags,
                colors,
                icons,
                depth + 1,
                &new_prefix,
            );
        }
    }

    output
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
    let mut output = String::new();
    output.push('\n');
    output += &meta.path.to_string_lossy();
    output += ":\n";

    output
}

fn get_output<'a>(
    meta: &'a Meta,
    colors: &'a Colors,
    icons: &'a Icons,
    flags: &'a Flags,
    display_option: &DisplayOption,
    padding_rules: &HashMap<Block, usize>,
    tree_prefix: &'a str,
) -> Vec<ANSIString<'a>> {
    let mut strings: Vec<ANSIString> = Vec::new();
    for block in flags.blocks.0.iter() {
        match block {
            Block::INode => strings.push(meta.inode.render(colors)),
            Block::Links => strings.push(meta.links.render(colors)),
            Block::Permission => {
                let s: &[ColoredString] = &[
                    meta.file_type.render(colors),
                    meta.permissions.render(colors),
                ];
                let res = ANSIStrings(s).to_string();
                strings.push(ColoredString::from(res));
            }
            Block::User => strings.push(meta.owner.render_user(colors)),
            Block::Group => strings.push(meta.owner.render_group(colors)),
            Block::Size => strings.push(meta.size.render(
                colors,
                &flags,
                padding_rules[&Block::SizeValue],
            )),
            Block::SizeValue => strings.push(meta.size.render_value(colors, flags)),
            Block::Date => strings.push(meta.date.render(colors, &flags)),
            Block::Name => {
                let s: String =
                    if flags.no_symlink.0 || flags.dereference.0 || flags.layout == Layout::Grid {
                        ANSIStrings(&[
                            ANSIString::from(tree_prefix),
                            meta.name.render(colors, icons, &display_option),
                            meta.indicator.render(&flags),
                        ])
                        .to_string()
                    } else {
                        ANSIStrings(&[
                            ANSIString::from(tree_prefix),
                            meta.name.render(colors, icons, &display_option),
                            meta.indicator.render(&flags),
                            meta.symlink.render(colors, &flags),
                        ])
                        .to_string()
                    };
                strings.push(ColoredString::from(s));
            }
        };
    }

    strings
}

fn get_visible_width(input: &str) -> usize {
    let mut nb_invisible_char = 0;

    // If the input has color, do not compute the length contributed by the color to the actual length
    for (idx, _) in input.match_indices("\u{1b}[") {
        let (_, s) = input.split_at(idx);

        let m_pos = s.find('m');
        if let Some(len) = m_pos {
            nb_invisible_char += len
        }
    }

    UnicodeWidthStr::width(input) - nb_invisible_char
}

fn detect_size_lengths(metas: &[Meta], flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let value_len = meta.size.value_string(flags).len();

        if value_len > max_value_length {
            max_value_length = value_len;
        }
    }

    max_value_length
}

fn get_padding_rules(metas: &[Meta], flags: &Flags) -> HashMap<Block, usize> {
    let mut padding_rules: HashMap<Block, usize> = HashMap::new();

    if flags.blocks.0.contains(&Block::Size) {
        let size_val = detect_size_lengths(&metas, &flags);

        padding_rules.insert(Block::SizeValue, size_val);
    }

    padding_rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::color::Colors;
    use crate::icon;
    use crate::icon::Icons;
    use crate::meta::{FileType, Name};
    use std::path::Path;

    #[test]
    fn test_display_get_visible_width_without_icons() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name.render(
                &Colors::new(color::Theme::NoColor),
                &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                &DisplayOption::FileName,
            );

            assert_eq!(get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        for (s, l) in &[
            // Add 3 characters for the icons.
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 24),
            ("ASCII1234-_", 13),
            ("File with space", 17),
            ("制作样本。", 12),
            ("日本語", 8),
            ("샘플은 무료로 드리겠습니다", 28),
            ("👩🐩", 6),
            ("🔬", 4),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoColor),
                    &Icons::new(icon::Theme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                )
                .to_string();

            assert_eq!(get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_colors() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoLscolors),
                    &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                    &DisplayOption::FileName,
                )
                .to_string();

            // check if the color is present.
            assert_eq!(true, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(true, output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_without_colors() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoColor),
                    &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                    &DisplayOption::FileName,
                )
                .to_string();

            // check if the color is present.
            assert_eq!(false, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(false, output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_tree_with_all() {
        let argv = vec!["lsd", "--tree", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        dir.child("one.d/.hidden").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        let output = inner_display_tree(
            &metas,
            &flags,
            &Colors::new(color::Theme::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
            0,
            "",
        );

        assert_eq!("one.d\n├── .hidden\n└── two\n", output);
    }

    #[test]
    fn test_display_tree_edge_before_name() {
        let argv = vec!["lsd", "--tree", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        let output = inner_display_tree(
            &metas,
            &flags,
            &Colors::new(color::Theme::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
            0,
            "",
        );

        assert!(output.ends_with("└── two\n"));
    }
}
