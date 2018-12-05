use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use Options;

const EDGE: &str = "├──";
const LINE: &str = "│  ";
const CORNER: &str = "└──";

pub struct Display {
    options: Options,
}

impl Display {
    pub fn new(options: Options) -> Display {
        Display { options }
    }

    pub fn print_outputs(&self, outputs: Vec<String>) {
        if self.options.display_long || self.options.display_online {
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

        if let Some(gridded_output) = grid.fit_into_width(term_width) {
            println!("{}", gridded_output);
        } else {
            //does not fit into grid, usually because (some) filename(s)
            //are longer or almost as long as term_width
            //print line by line instead!
            let lined_output = grid.fit_into_columns(1);
            println!("{}", lined_output);
        }
    }

    pub fn print_tree_row(&self, output: String, depth: usize, last: bool) {
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

        println!("{}", res);
    }

    fn print_one_per_line(&self, outputs: &[String]) {
        let mut res = String::new();
        for output in outputs {
            res += output;
            res += "\n";
        }

        println!("{}", res);
    }

    fn get_visible_width(&self, input: &str) -> usize {
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
