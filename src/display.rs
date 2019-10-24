use crate::color::{ColoredString, Colors};
use crate::flags::{Block, Display, Flags};
use crate::icon::Icons;
use crate::meta::{FileType, Meta};
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "├  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"
const BLANK: &str = "   ";

pub fn one_line(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_one_line(metas, &flags, colors, icons, 0)
}

pub fn grid(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    let term_width = match terminal_size() {
        Some((w, _)) => Some(w.0 as usize),
        None => None,
    };

    inner_display_grid(metas, &flags, colors, icons, 0, term_width)
}

pub fn tree(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_tree(metas, &flags, colors, icons, 0, "")
}

fn inner_display_one_line(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(&metas, flags, icons);
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DisplayDirectoryItself);

    // print the files first.
    for meta in metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if let (true, FileType::Directory { .. }) = (skip_dirs, meta.file_type) {
            continue;
        }

        if flags.prefix_indent {
            output.push_str("    ");
        }

        let blocks = get_output(&meta, &colors, &icons, &flags, &padding_rules);

        for block in blocks {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }
    }

    output += &grid.fit_into_columns(flags.blocks.len()).to_string();

    let should_display_folder_path = should_display_folder_path(depth, &metas);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(&meta);
            }

            output += &inner_display_one_line(
                meta.content.as_ref().unwrap(),
                &flags,
                colors,
                icons,
                depth + 1,
            );
        }
    }

    output
}

fn inner_display_grid(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(&metas, flags, icons);

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DisplayDirectoryItself);

    // print the files first.
    for meta in metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if let (true, FileType::Directory { .. }) = (skip_dirs, meta.file_type) {
            continue;
        }

        for block in get_output(&meta, &colors, &icons, &flags, &padding_rules) {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }
    }

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

    let should_display_folder_path = should_display_folder_path(depth, &metas);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(&meta);
            }

            output += &inner_display_grid(
                &meta.content.as_ref().unwrap(),
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

    let padding_rules = get_padding_rules(&metas, flags, icons);

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    for (idx, meta) in metas.into_iter().enumerate() {
        let is_last_folder_elem = idx + 1 != last_idx;

        if depth > 0 {
            output += prefix;

            if is_last_folder_elem {
                output += EDGE;
            } else {
                output += CORNER;
            }
            output += " ";
        }

        for block in get_output(&meta, &colors, &icons, &flags, &padding_rules) {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }

        if meta.content.is_some() {
            let mut new_prefix = String::from(prefix);

            if depth > 0 {
                if is_last_folder_elem {
                    new_prefix += LINE;
                } else {
                    new_prefix += BLANK;
                }
            }

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

fn should_display_folder_path(depth: usize, metas: &[Meta]) -> bool {
    if depth > 0 {
        true
    } else {
        let folder_number = metas
            .iter()
            .filter(|x| match x.file_type {
                FileType::Directory { .. } => true,
                _ => false,
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
    padding_rules: &HashMap<Block, usize>,
) -> Vec<ANSIString<'a>> {
    let mut strings: Vec<ANSIString> = Vec::new();
    for block in flags.blocks.iter() {
        match block {
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
            Block::SizeUnit => strings.push(meta.size.render_unit(colors, flags)),
            Block::Date => strings.push(meta.date.render(
                colors,
                padding_rules[&Block::Date],
                &flags,
            )),
            Block::Name => {
                let s: &[ColoredString] = &[
                    meta.name.render(colors, icons),
                    meta.indicator.render(&flags),
                    ANSIString::from(" ".to_string().repeat(
                        padding_rules[&Block::Name]
                            - meta.indicator.len(&flags)
                            - meta.name.name_string(icons).len(),
                    )),
                ];

                let res = ANSIStrings(s).to_string();
                strings.push(ColoredString::from(res));
            }
            Block::NameWithSymlink => {
                match meta.symlink.symlink_string() {
                    Some(s) => {
                        let s2: &[ColoredString] = &[
                            meta.name.render(colors, icons),
                            meta.indicator.render(&flags),
                            meta.symlink.render(colors),
                            ANSIString::from(" ".to_string().repeat(
                                padding_rules[&Block::NameWithSymlink]
                        - 3 //  3 = ( arrow + 2 spaces) for symlink;
                        - meta.name.name_string(icons).len()
                        - meta.indicator.len(&flags)
                        - s.len(),
                            )),
                        ];

                        let res = ANSIStrings(s2).to_string();
                        strings.push(ColoredString::from(res));
                    }
                    None => {
                        let s: &[ColoredString] = &[
                            meta.name.render(colors, icons),
                            meta.indicator.render(&flags),
                            meta.symlink.render(colors),
                            ANSIString::from(" ".to_string().repeat(
                                padding_rules[&Block::NameWithSymlink]
                                    - meta.indicator.len(&flags)
                                    - meta.name.name_string(icons).len(),
                            )),
                        ];

                        let res = ANSIStrings(s).to_string();
                        strings.push(ColoredString::from(res));
                    }
                }
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
            nb_invisible_char = nb_invisible_char + len
        }
    }

    UnicodeWidthStr::width(input) - nb_invisible_char
}

fn detect_user_length(metas: &[Meta]) -> usize {
    let mut max: usize = 0;

    for meta in metas {
        let user = meta.owner.user();
        if user.len() > max {
            max = user.len();
        }
    }

    max
}

fn detect_group_length(metas: &[Meta]) -> usize {
    let mut max: usize = 0;

    for meta in metas {
        let group = meta.owner.group();
        if group.len() > max {
            max = group.len();
        }
    }

    max
}

fn detect_date_length(metas: &[Meta], flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        if meta.date.date_string(&flags).len() > max_value_length {
            max_value_length = meta.date.date_string(&flags).len();
        }
    }

    max_value_length
}

fn detect_size_lengths(metas: &[Meta], flags: &Flags) -> (usize, usize) {
    let mut max_value_length: usize = 0;
    let mut max_unit_size: usize = 0;

    for meta in metas {
        let value_len = meta.size.value_string(flags).len();
        let unit_len = meta.size.unit_string(&flags).len();

        if value_len > max_value_length {
            max_value_length = value_len;
        }

        if unit_len > max_unit_size {
            max_unit_size = unit_len;
        }
    }

    (max_value_length, max_unit_size)
}

fn detect_name_length(metas: &[Meta], icons: &Icons, flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let len = meta.name.name_string(&icons).len() + meta.indicator.len(&flags);
        if len > max_value_length {
            max_value_length = len;
        }
    }

    max_value_length
}

fn detect_name_with_symlink_length(metas: &[Meta], icons: &Icons, flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let mut len = meta.name.name_string(&icons).len() + meta.indicator.len(&flags);
        if let Some(syml) = meta.symlink.symlink_string() {
            len += syml.len() + 3 // 3 = ( arrow + 2 spaces) for symlink;
        }
        if len > max_value_length {
            max_value_length = len;
        }
    }

    max_value_length
}

fn get_padding_rules(metas: &[Meta], flags: &Flags, icons: &Icons) -> HashMap<Block, usize> {
    let mut padding_rules: HashMap<Block, usize> = HashMap::new();

    for block in flags.blocks.iter() {
        match block {
            Block::SizeValue => None,
            Block::Permission => padding_rules.insert(Block::Permission, 10),
            Block::SizeUnit => None,
            Block::User => padding_rules.insert(Block::User, detect_user_length(&metas)),
            Block::Group => padding_rules.insert(Block::Group, detect_group_length(&metas)),
            Block::Date => padding_rules.insert(Block::Date, detect_date_length(&metas, &flags)),
            Block::Name => {
                padding_rules.insert(Block::Name, detect_name_length(&metas, &icons, &flags))
            }
            Block::NameWithSymlink => padding_rules.insert(
                Block::NameWithSymlink,
                detect_name_with_symlink_length(&metas, &icons, &flags),
            ),
            Block::Size => {
                let (size_val, size_unit) = detect_size_lengths(&metas, &flags);

                padding_rules.insert(Block::SizeValue, size_val);
                padding_rules.insert(Block::SizeUnit, size_unit);

                None
            }
        };
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
                &Icons::new(icon::Theme::NoIcon),
            );

            assert_eq!(get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        for (s, l) in &[
            // Add 3 characters for the icons.
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 25),
            ("ASCII1234-_", 14),
            ("File with space", 18),
            ("制作样本。", 13),
            ("日本語", 9),
            ("샘플은 무료로 드리겠습니다", 29),
            ("👩🐩", 7),
            ("🔬", 5),
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
                    &Icons::new(icon::Theme::Fancy),
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
                    &Icons::new(icon::Theme::NoIcon),
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
                    &Icons::new(icon::Theme::NoIcon),
                )
                .to_string();

            // check if the color is present.
            assert_eq!(false, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(false, output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output), *l);
        }
    }
}
