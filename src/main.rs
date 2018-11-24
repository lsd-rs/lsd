extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate failure;
extern crate size;
extern crate time;
extern crate users;

mod color;
mod core;
mod formatter;
mod logo;
mod meta;

use clap::{App, Arg};
use core::Core;

pub struct Options {
    pub display_all: bool,
}

fn main() {
    let matches = App::new("lsd")
        .about("A ls comment with a lot of pretty colors and some other stuff.")
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(Arg::with_name("all").short("a"))
        .get_matches();

    let options = Options {
        display_all: matches.is_present("all"),
    };

    let inputs: Vec<&str> = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .collect();

    let core = Core::new(&options);

    core.print(inputs);
}
