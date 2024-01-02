#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::match_same_arms,
    clippy::cast_possible_wrap
)]

extern crate chrono;
extern crate chrono_humanize;
extern crate clap;
extern crate dirs;
extern crate libc;
extern crate lscolors;
#[cfg(test)]
extern crate tempfile;
extern crate term_grid;
extern crate terminal_size;
extern crate unicode_width;
extern crate url;
extern crate wild;
extern crate yaml_rust;

#[cfg(unix)]
extern crate users;

#[cfg(windows)]
extern crate windows;

mod app;
mod color;
mod config_file;
mod core;
mod display;
mod flags;
mod git;
mod git_theme;
mod icon;
mod meta;
mod sort;
mod theme;

use clap::Parser;

use crate::app::Cli;
use crate::config_file::Config;
use crate::core::Core;
use crate::flags::Flags;

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone)]
pub enum ExitCode {
    OK,
    MinorIssue,
    MajorIssue,
}
impl ExitCode {
    pub fn set_if_greater(&mut self, code: ExitCode) {
        let self_i32 = *self as i32;
        let code_i32 = code as i32;
        if self_i32 < code_i32 {
            *self = code;
        }
    }
}
/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stderr close before our program.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        {
            use std::io::Write;

            let stderr = std::io::stderr();

            {
                let mut handle = stderr.lock();
                // We can write on stderr, so we simply ignore the error and don't print
                // and stop with success.
                let res = handle.write_all(std::format!("lsd: {}\n\n",
                                                        std::format!($($arg)*)).as_bytes());
                if res.is_err() {
                    std::process::exit(0);
                }
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
    let cli = Cli::parse_from(wild::args_os());

    let config = if cli.ignore_config {
        Config::with_none()
    } else if let Some(path) = &cli.config_file {
        Config::from_file(path).expect("Provided file path is invalid")
    } else {
        Config::default()
    };
    let flags = Flags::configure_from(&cli, &config).unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);

    let exit_code = core.run(cli.inputs);
    std::process::exit(exit_code as i32);
}
