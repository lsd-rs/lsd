// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::*;
use std::fs;
use std::process::exit;

include!("src/app.rs");

fn main() {
    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| std::env::var_os("OUT_DIR"))
        .unwrap_or_else(|| exit(0));

    fs::create_dir_all(&outdir).unwrap();

    let mut app = Cli::command();
    let bin_name = "lsd";
    generate_to(Bash, &mut app, bin_name, &outdir).expect("Failed to generate Bash completions");
    generate_to(Fish, &mut app, bin_name, &outdir).expect("Failed to generate Fish completions");
    generate_to(Zsh, &mut app, bin_name, &outdir).expect("Failed to generate Zsh completions");
    generate_to(PowerShell, &mut app, bin_name, &outdir)
        .expect("Failed to generate PowerShell completions");

    // Disable git feature for these target where git2 is not well supported
    if !std::env::var("CARGO_FEATURE_GIT2")
        .map(|flag| flag == "1")
        .unwrap_or(false)
        || std::env::var("TARGET")
            .map(|target| target == "i686-pc-windows-gnu")
            .unwrap_or(false)
    {
        println!(r#"cargo:rustc-cfg=feature="no-git""#);
    }
}
