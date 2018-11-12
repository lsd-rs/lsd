extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate time;

mod formatter;
mod path_lister;
mod presenter;

use clap::{App, Arg};
use path_lister::PathLister;
use presenter::Presenter;

pub struct Options {
    pub display_long: bool,
    pub display_all: bool,
}

fn main() {
    let matches = App::new("lsd")
        .about("A ls comment with a lot of pretty colors and some other stuff.")
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(Arg::with_name("long").short("l"))
        .arg(Arg::with_name("all").short("a"))
        .get_matches();

    let options = Options {
        display_long: matches.is_present("long"),
        display_all: matches.is_present("all"),
    };

    let inputs: Vec<&str> = matches.values_of("FILE").unwrap().collect();

    let path_lister = PathLister::new(&options);

    let presenter = Presenter::new(&options);

    presenter.print(path_lister.list_paths_to_print(inputs));
}
