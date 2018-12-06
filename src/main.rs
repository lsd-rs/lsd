#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate libc;
extern crate term_grid;
extern crate terminal_size;
extern crate time;
extern crate users;

mod app;
mod batch;
mod color;
mod core;
mod display;
mod icon;
mod meta;

use core::Core;
use std::path::PathBuf;

#[derive(Clone, Debug, Copy)]
pub struct Options {
    pub display_all: bool,
    pub display_long: bool,
    pub display_online: bool,
    pub display_tree: bool,
    pub recursive: bool,
}

fn main() {
    let matches = app::build_app().get_matches();

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

    let core = Core::new(options);

    core.run(inputs);
}
