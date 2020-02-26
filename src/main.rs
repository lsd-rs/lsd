#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::match_same_arms,
    clippy::cast_possible_wrap
)]

#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate chrono_humanize;
extern crate libc;
extern crate lscolors;
#[cfg(test)]
extern crate tempfile;
extern crate term_grid;
extern crate terminal_size;
extern crate time;
extern crate unicode_width;
extern crate wild;

#[cfg(unix)]
extern crate users;

#[cfg(windows)]
extern crate winapi;

mod app;
mod color;
mod core;
mod display;
mod flags;
mod icon;
mod meta;
mod sort;

use crate::core::Core;
use crate::flags::Flags;
use std::path::PathBuf;

/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stderr close before our program.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        use std::io::Write;

        let stderr = std::io::stderr();

        {
            let mut handle = stderr.lock();
            // We can write on stderr, so we simply ignore the error and don't print
            // and stop with success.
            let res = handle.write_all(std::format!($($arg)*).as_bytes());
            if res.is_err() {
                std::process::exit(0);
            }
        }
    };
}

/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stdout close before our program.
#[macro_export]
macro_rules! print_output {
    ($($arg:tt)*) => {
        use std::io::Write;

        let stderr = std::io::stdout();


        {
            let mut handle = stderr.lock();
            // We can write on stdout, so we simply ignore the error and don't print
            // and stop with success.
            let res = handle.write_all(std::format!($($arg)*).as_bytes());
            if res.is_err() {
                std::process::exit(0);
            }
        }
    };
}

fn main() {
    let matches = app::build().get_matches_from(wild::args_os());

    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();

    let flags = Flags::from_matches(&matches).unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);

    core.run(inputs);
}
