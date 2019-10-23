use crate::color::Colors;
use crate::flags::{Block, Display, Flags, Layout};
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

#[derive(Debug, Copy, Clone)]
struct PaddingRules {
    user: usize,
    group: usize,
    size_val: usize,
    size_unit: usize,
    date: usize,
    name: usize,
    name_with_symlink: usize,
}

pub fn one_line(metas: Vec<Meta>, flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_one_line(metas, &flags, colors, icons, 0)
}

pub fn grid(metas: Vec<Meta>, flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    let term_width = match terminal_size() {
        Some((w, _)) => Some(w.0 as usize),
        None => None,
    };

    inner_display_grid(metas, &flags, colors, icons, 0, term_width)
}

pub fn tree(metas: Vec<Meta>, flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_tree(metas, &flags, colors, icons, 0, "")
}

fn inner_display_one_line(
    metas: Vec<Meta>,
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(&metas, flags, icons);

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DisplayDirectoryItself);

    // print the files first.
    for meta in &metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if let (true, FileType::Directory { .. }) = (skip_dirs, meta.file_type) {
            continue;
        }

        if flags.prefix_indent {
            output.push_str("    ");
        }

        if let Layout::OneLine { long: true } = flags.layout {
            output += &get_long_output(&meta, &colors, &icons, &flags, &padding_rules);
        } else {
            output += &get_short_output(&meta, &colors, &icons, &flags);
        }

        output.push('\n');
    }

    let should_display_folder_path = should_display_folder_path(depth, &metas);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(&meta);
            }

            output +=
                &inner_display_one_line(meta.content.unwrap(), &flags, colors, icons, depth + 1);
        }
    }

    output
}

fn inner_display_grid(
    metas: Vec<Meta>,
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DisplayDirectoryItself);

    // print the files first.
    for meta in &metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if let (true, FileType::Directory { .. }) = (skip_dirs, meta.file_type) {
            continue;
        }

        let line_output = get_short_output(&meta, &colors, &icons, &flags);
        grid.add(Cell {
            width: get_visible_width(&line_output),
            contents: line_output,
        });
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
                meta.content.unwrap(),
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
    metas: Vec<Meta>,
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    prefix: &str,
) -> String {
    let mut output = String::new();
    let last_idx = metas.len();

    let padding_rules = get_padding_rules(&metas, flags, icons);

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

        if let Layout::Tree { long: true } = flags.layout {
            output += &get_long_output(&meta, &colors, &icons, &flags, &padding_rules);
        } else {
            output += &get_short_output(&meta, &colors, &icons, &flags);
        }
        output += "\n";

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
                meta.content.unwrap(),
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

fn get_short_output(meta: &Meta, colors: &Colors, icons: &Icons, flags: &Flags) -> String {
    let strings: &[ANSIString] = &[
        meta.name.render(colors, icons),
        meta.indicator.render(&flags),
    ];

    ANSIStrings(strings).to_string()
}

fn get_long_output(
    meta: &Meta,
    colors: &Colors,
    icons: &Icons,
    flags: &Flags,
    padding_rules: &HashMap<Block, usize>,
) -> String {
    let mut strings: Vec<ANSIString> = Vec::new();
    for block in flags.blocks.iter() {
        match block {
            Block::Permission => {
                strings.push(meta.file_type.render(colors));
                strings.push(meta.permissions.render(colors));
            }
            Block::User => {
                strings.push(meta.owner.render_user(colors, padding_rules[&Block::User]))
            }
            Block::Group => strings.push(
                meta.owner
                    .render_group(colors, padding_rules[&Block::Group]),
            ),
            Block::Size => strings.push(meta.size.render(
                colors,
                &flags,
                padding_rules[&Block::SizeValue],
                padding_rules[&Block::SizeUnit],
            )),
            Block::SizeValue => strings.push(meta.size.render_value(colors, flags)),
            Block::SizeUnit => strings.push(meta.size.render_unit(colors, flags)),
            Block::Date => strings.push(meta.date.render(
                colors,
                padding_rules[&Block::Date],
                &flags,
            )),
            Block::Name => {
                strings.push(meta.name.render(colors, icons));
                strings.push(meta.indicator.render(&flags));
                strings.push(ANSIString::from(" ".to_string().repeat(
                    padding_rules[&Block::Name]
                        - meta.indicator.len(&flags)
                        - meta.name.name_string(icons).len(),
                )))
            }
            Block::NameWithSymlink => {
                match meta.symlink.symlink_string() {
                    Some(s) => {
                        strings.push(meta.name.render(colors, icons));
                        strings.push(meta.indicator.render(&flags));
                        strings.push(meta.symlink.render(colors));
                        strings.push(ANSIString::from(" ".to_string().repeat(
                            padding_rules[&Block::NameWithSymlink]
                                //padding_rules.name_with_symlink
                                    - 3 //  3 = ( arrow + 2 spaces) for symlink;
                                    - meta.name.name_string(icons).len()
                                    - meta.indicator.len(&flags)
                                    - s.len(),
                        )))
                    }
                    None => {
                        strings.push(meta.name.render(colors, icons));
                        strings.push(meta.indicator.render(&flags));
                        strings.push(meta.symlink.render(colors));
                        strings.push(ANSIString::from(" ".to_string().repeat(
                            padding_rules[&Block::NameWithSymlink]
                                - meta.name.name_string(icons).len()
                                - meta.indicator.len(&flags),
                        )))
                    }
                }
            }
        };
        strings.push(ANSIString::from(" "));
    }

    strings.pop(); // remove the last space

    ANSIStrings(&strings).to_string()
}

fn get_visible_width(input: &str) -> usize {
    let mut nb_invisible_char = 0;

    // If the input has color, do not compute the length contributed by the color to the actual length
    if input.starts_with("\u{1b}[") {
        let m_pos = input.find('m');
        if let Some(len) = m_pos {
            nb_invisible_char = len + 3 // 1 (index -> length) + 2 ( compensate for color reset chars )
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
        let value_len = meta.size.size_string(flags).len();
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
                Block::Name,
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
