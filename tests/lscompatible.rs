/// This is tests make sure lsd is compatible with the GNU ls
/// The tests here MUST all pass in every changes
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
#[cfg(unix)]
use std::os::unix::fs;
use std::process::{Command, Stdio};

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}

#[test]
fn test_pipe_should_use_line() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    let lsd = cmd()
        .arg("--ignore-config")
        .arg(dir.path())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start lsd process");
    let lsd_out = lsd.stdout.expect("Failed to open ls stdout");

    let cat_lsd = Command::new("cat")
        .stdin(Stdio::from(lsd_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cat process");
    let output_lsd = cat_lsd
        .wait_with_output()
        .expect("Failed to wait on cat lsd");

    let ls = Command::new("ls")
        .arg(dir.path())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ls process");
    let ls_out = ls.stdout.expect("Failed to open ls stdout");

    let cat_ls = Command::new("cat")
        .stdin(Stdio::from(ls_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cat process");
    let output_ls = cat_ls.wait_with_output().expect("Failed to wait on cat ls");

    assert_eq!(output_ls.stdout, output_lsd.stdout);
}

#[cfg(unix)]
#[test]
fn test_show_folder_content_of_symlink_oneline() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("--ignore-config")
        .arg("--oneline")
        .arg(link)
        .assert()
        .stdout(predicate::str::starts_with("link").not())
        .stdout(predicate::str::starts_with("inside"));
}

#[cfg(unix)]
#[test]
fn test_show_folder_content_of_symlink() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("--ignore-config")
        .arg(link)
        .assert()
        .stdout(predicate::str::starts_with("link").not())
        .stdout(predicate::str::starts_with("inside"));
}
