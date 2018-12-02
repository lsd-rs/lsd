use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use Options;

pub struct Display<'a> {
    options: &'a Options,
}

impl<'a> Display<'a> {
    pub fn new(options: &'a Options) -> Display<'a> {
        Display { options }
    }

    pub fn print_outputs(&self, outputs: Vec<String>) {
        if self.options.display_long {
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
            direction: Direction::LeftToRight,
        });

        for output in outputs {
            grid.add(Cell {
                width: self.get_visible_width(&output),
                contents: output,
            });
        }

        println!(
            "{}",
            grid.fit_into_width(term_width)
                .expect("failed to print the grid")
        );
    }

    fn print_one_per_line(&self, outputs: &Vec<String>) {
        for output in outputs {
            println!("{}", output);
        }
    }

    fn get_visible_width(&self, input: &String) -> usize {
        let mut nb_invisible_char = 0;

        for (idx, _) in input.match_indices("[38;5;") {
            let color_code = input.chars().skip(idx + 6);
            let mut code_size = 0;
            color_code
                .skip_while(|x| {
                    code_size += 1;
                    char::is_numeric(*x)
                }).count();
            nb_invisible_char += 6 + code_size; /* "[38;5;" + color number + "m" */
        }

        nb_invisible_char += 3; /* "[0m" */

        input.chars().count() - nb_invisible_char
    }
}
