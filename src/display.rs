use color::ColoredString;
use flags::Flags;
use std::io::{self, Write};
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "├  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"

pub struct Display {
    flags: Flags,
}

impl Display {
    pub fn new(flags: Flags) -> Self {
        Self { flags }
    }

    pub fn print_outputs(&self, outputs: Vec<String>) {
        if self.flags.display_long || self.flags.display_online {
            self.print_one_per_line(&outputs);
        } else {
            self.print_grid(outputs);
        }
    }

    fn print_grid(&self, outputs: Vec<String>) {
        let term_width = match terminal_size() {
            Some((w, _)) => w.0 as usize,
            None => panic!("failed to retrieve terminal size"),
        };

        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom,
        });

        for output in outputs {
            grid.add(Cell {
                width: self.get_visible_width(&output),
                contents: output,
            });
        }

        if let Some(gridded_output) = grid.fit_into_width(term_width) {
            self.print_output(&gridded_output.to_string());
        } else {
            //does not fit into grid, usually because (some) filename(s)
            //are longer or almost as long as term_width
            //print line by line instead!
            let lined_output = grid.fit_into_columns(1);
            self.print_output(&lined_output.to_string());
        }
    }

    pub fn print_output(&self, output: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        if let Err(err) = handle.write_all(output.as_bytes()) {
            if err.kind() != io::ErrorKind::Interrupted {
                io::stderr().write_all(err.to_string().as_bytes()).unwrap();
                std::process::exit(1);
            }
        };

        // Do not check th
        if let Err(err) = handle.flush() {
            match err.kind() {
                io::ErrorKind::Interrupted | io::ErrorKind::BrokenPipe => std::process::exit(0),
                _ => {
                    io::stderr().write_all(err.to_string().as_bytes()).unwrap();
                    std::process::exit(1);
                }
            };
        }
    }

    pub fn print_tree_row(&self, output: &ColoredString, depth: usize, last: bool) -> String {
        let mut res = String::new();

        for _ in 0..depth {
            res += LINE;
        }

        if last {
            res += EDGE;
        } else {
            res += CORNER;
        }

        res += " ";
        res += &output;
        res += "\n";

        res
    }

    fn print_one_per_line(&self, outputs: &[String]) {
        let mut res = String::new();
        for output in outputs {
            res += output;
            res += "\n";
        }

        self.print_output(&res);
    }

    fn get_visible_width(&self, input: &str) -> usize {
        let mut nb_invisible_char = 0;

        for (idx, _) in input.match_indices("\u{1b}[38;5;" /* "\e[38;5;" */) {
            let color_code = input.chars().skip(idx + 7);
            let mut code_size = 0;
            color_code
                .skip_while(|x| {
                    code_size += 1;
                    char::is_numeric(*x)
                })
                .count();
            nb_invisible_char += 6 + code_size; /* "\e[38;5;" + color number + "m" */
        }

        if nb_invisible_char > 0 {
            // If no color have been set, the is no reset character.
            nb_invisible_char += 3; /* "[0m" */
        }

        UnicodeWidthStr::width(input) - nb_invisible_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color;
    use color::Colors;
    use icon;
    use icon::Icons;
    use meta::{FileType, Name};
    use std::path::Path;

    #[test]
    fn test_display_get_visible_width_without_icons() {
        let display = Display::new(Flags::default());
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
            let name = Name::new(&path, FileType::File);
            let output = name.render(
                &Colors::new(color::Theme::NoColor),
                &Icons::new(icon::Theme::NoIcon),
            );
            assert_eq!(display.get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        let display = Display::new(Flags::default());
        for (s, l) in &[
            // Add 3 characters for the icons.
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 25),
            ("ASCII1234-_", 14),
            ("制作样本。", 13),
            ("日本語", 9),
            ("샘플은 무료로 드리겠습니다", 29),
            ("👩🐩", 7),
            ("🔬", 5),
        ] {
            let path = Path::new(s);
            let name = Name::new(&path, FileType::File);
            let output = name
                .render(
                    &Colors::new(color::Theme::NoColor),
                    &Icons::new(icon::Theme::Fancy),
                )
                .to_string();
            assert_eq!(display.get_visible_width(&output), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_colors() {
        let display = Display::new(Flags::default());
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
            let name = Name::new(&path, FileType::File);
            let output = name
                .render(
                    &Colors::new(color::Theme::Default),
                    &Icons::new(icon::Theme::NoIcon),
                )
                .to_string();

            // check if the color is present.
            assert_eq!(true, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(true, output.ends_with("[0m"));

            assert_eq!(display.get_visible_width(&output), *l);
        }
    }
}
