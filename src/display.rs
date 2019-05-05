use crate::color::Colors;
use crate::flags::{Flags, Layout};
use crate::icon::Icons;
use crate::meta::{FileType, Meta};
use ansi_term::{ANSIString, ANSIStrings};
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
    size: (usize, usize),
    date: usize,
}

pub fn one_line(metas: Vec<Meta>, flags: Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_one_line(metas, flags, colors, icons, 0)
}

pub fn grid(metas: Vec<Meta>, flags: Flags, colors: &Colors, icons: &Icons) -> String {
    let term_width = match terminal_size() {
        Some((w, _)) => Some(w.0 as usize),
        None => None,
    };

    inner_display_grid(metas, flags, colors, icons, 0, term_width)
}

pub fn tree(metas: Vec<Meta>, flags: Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_tree(metas, flags, colors, icons, 0, "")
}

fn inner_display_one_line(
    metas: Vec<Meta>,
    flags: Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
) -> String {
    let mut output = String::new();

    let mut padding_rules = None;
    if let Layout::OneLine { long: true } = flags.layout {
        // Defining the padding rules is costly and so shouldn't be done several
        // times. That's why it's done outside the loop.
        padding_rules = Some(PaddingRules {
            user: detect_user_length(&metas),
            group: detect_group_length(&metas),
            size: detect_size_lengths(&metas, flags),
            date: detect_date_length(&metas, flags),
        })
    }

    // print the files first.
    for meta in &metas {
        // The first iteration (depth == 0) correspond to the inputs given by
        // the user. If the user enter a folder name it should not print the
        // folder meta but its content.
        if let (0, FileType::Directory { .. }) = (depth, meta.file_type) {
            continue;
        }

        if let Layout::OneLine { long: true } = flags.layout {
            output += &get_long_output(&meta, &colors, &icons, flags, padding_rules.unwrap());
        } else {
            output += &get_short_output(&meta, &colors, &icons, flags);
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
                &inner_display_one_line(meta.content.unwrap(), flags, colors, icons, depth + 1);
        }
    }

    output
}

fn inner_display_grid(
    metas: Vec<Meta>,
    flags: Flags,
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

    // print the files first.
    for meta in &metas {
        // The first iteration (depth == 0) correspond to the inputs given by
        // the user. If the user enter a folder name it should not print the
        // folder meta but its content.
        if let (0, FileType::Directory { .. }) = (depth, meta.file_type) {
            continue;
        }

        let line_output = get_short_output(&meta, &colors, &icons, flags);
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
                flags,
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
    flags: Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    prefix: &str,
) -> String {
    let mut output = String::new();
    let last_idx = metas.len();

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

        output += &get_short_output(&meta, &colors, &icons, flags);
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
                flags,
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

fn get_short_output(meta: &Meta, colors: &Colors, icons: &Icons, flags: Flags) -> String {
    let strings: &[ANSIString] = &[
        meta.name.render(colors, icons),
        meta.indicator.render(flags),
    ];

    ANSIStrings(strings).to_string()
}

fn get_long_output(
    meta: &Meta,
    colors: &Colors,
    icons: &Icons,
    flags: Flags,
    padding_rules: PaddingRules,
) -> String {
    let strings: &[ANSIString] = &[
        meta.file_type.render(colors),
        meta.permissions.render(colors),
        ANSIString::from(" "),
        meta.owner.render_user(colors, padding_rules.user),
        ANSIString::from(" "),
        meta.owner.render_group(colors, padding_rules.group),
        ANSIString::from(" "),
        meta.size
            .render(colors, padding_rules.size.0, padding_rules.size.1, flags),
        ANSIString::from(" "),
        meta.date.render(colors, padding_rules.date, flags),
        ANSIString::from(" "),
        meta.name.render(colors, icons),
        meta.indicator.render(flags),
        meta.symlink.render(colors),
    ];

    ANSIStrings(strings).to_string()
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

fn detect_date_length(metas: &[Meta], flags: Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        if meta.date.date_string(flags).len() > max_value_length {
            max_value_length = meta.date.date_string(flags).len();
        }
    }

    max_value_length
}

fn detect_size_lengths(metas: &[Meta], flags: Flags) -> (usize, usize) {
    let mut max_value_length: usize = 0;
    let mut max_unit_size: usize = 0;

    for meta in metas {
        if meta.size.render_value().len() > max_value_length {
            max_value_length = meta.size.render_value().len();
        }

        if meta.size.render_unit(flags).len() > max_unit_size {
            max_unit_size = meta.size.render_unit(flags).len();
        }
    }

    (max_value_length, max_unit_size)
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
