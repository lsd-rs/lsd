#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::match_same_arms,
        clippy::cast_possible_wrap
    )
)]

#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate chrono_humanize;
extern crate libc;
#[cfg(test)]
extern crate tempdir;
extern crate term_grid;
extern crate terminal_size;
extern crate time;
extern crate unicode_width;
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
    let matches = app::build().get_matches();

    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();

    let flags = Flags::from_matches(&matches).unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);

    core.run(inputs);
}
