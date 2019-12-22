extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_runs_okay() {
    cmd().assert().success();
}

#[test]
fn test_list_empty_directory() {
    cmd()
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(""));
}

#[test]
fn test_list_almost_all_empty_directory() {
    let matched = "";
    cmd()
        .arg("--almost-all")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(matched));

    cmd()
        .arg("-A")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(matched));
}

#[test]
fn test_list_all_empty_directory() {
    let matched = "\\.\n\\.\\.\n$";
    cmd()
        .arg("--all")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());

    cmd()
        .arg("-a")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();
    cmd()
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("one\ntwo\n$").unwrap());
}

#[test]
fn test_list_almost_all_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();
    cmd()
        .arg("--almost-all")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("one\ntwo\n$").unwrap());
}

#[test]
fn test_list_all_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();
    cmd()
        .arg("--all")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("\\.\n\\.\\.\none\ntwo\n$").unwrap());
}

#[test]
fn test_list_inode_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    #[cfg(unix)]
    let matched = "\\d+ one\n\\d+ two\n$";
    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";

    cmd()
        .arg("--inode")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
    cmd()
        .arg("-i")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_block_inode_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    #[cfg(unix)]
    let matched = "\\d+ one\n\\d+ two\n$";
    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";

    cmd()
        .arg("--blocks")
        .arg("inode,name")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_inode_with_long_ok() {
    let dir = tempdir();
    cmd().arg("-i").arg("-l").arg(dir.path()).assert().success();
}

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}
