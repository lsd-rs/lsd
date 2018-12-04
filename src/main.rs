extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate libc;
extern crate term_grid;
extern crate terminal_size;
extern crate time;
extern crate users;

mod batch;
mod color;
mod core;
mod display;
mod icon;
mod meta;

use clap::{App, Arg};
use core::Core;
use std::path::PathBuf;

pub struct Options {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub recursive: bool,
}

fn main() {
    let matches = App::new("lsd")
        .about("An ls comment with a lot of pretty colors and some other stuff.")
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(Arg::with_name("all").short("a").long("all"))
        .arg(Arg::with_name("long").short("l").long("long"))
        .arg(Arg::with_name("oneline").short("1").long("oneline"))
        .arg(Arg::with_name("recursive").short("R").long("recursive"))
        .arg(Arg::with_name("tree").long("tree"))
        .get_matches();

    let options = Options {
        display_all: matches.is_present("all"),
        display_long: matches.is_present("long"),
        display_online: matches.is_present("oneline"),
        display_tree: matches.is_present("tree"),
        recursive: matches.is_present("recursive"),
    };

    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();

    let core = Core::new(&options);

    core.run(inputs);
}
