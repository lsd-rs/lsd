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
mod flags;
mod icon;
mod meta;

use core::Core;
use flags::Flags;
use std::path::PathBuf;

fn main() {
    let matches = app::build_app().get_matches();

    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();

    let core = Core::new(Flags::from(matches));

    core.run(inputs);
}
