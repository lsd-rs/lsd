extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs;

#[test]
fn test_runs_okay() {
    cmd().assert().success();
}

#[test]
fn test_list_empty_directory() {
    cmd()
        .arg("--ignore-config")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(""));
}

#[test]
fn test_list_almost_all_empty_directory() {
    let matched = "";
    cmd()
        .arg("--almost-all")
        .arg("--ignore-config")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(matched));

    cmd()
        .arg("-A")
        .arg("--ignore-config")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(matched));
}

#[test]
fn test_list_all_empty_directory() {
    let matched = "\\.\n\\.\\.\n$";
    cmd()
        .arg("--all")
        .arg("--ignore-config")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());

    cmd()
        .arg("-a")
        .arg("--ignore-config")
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
        .arg("--ignore-config")
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
        .arg("--ignore-config")
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
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("\\.\n\\.\\.\none\ntwo\n$").unwrap());
}

#[test]
fn test_list_inode_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";
    #[cfg(unix)]
    let matched = "\\d+ one\n\\d+ two\n$";

    cmd()
        .arg("--inode")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
    cmd()
        .arg("-i")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_block_inode_populated_directory_without_long() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    let matched = "one\ntwo\n$";

    cmd()
        .arg("--blocks")
        .arg("inode,name")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_block_inode_populated_directory_with_long() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";
    #[cfg(unix)]
    let matched = "\\d+ one\n\\d+ two\n$";

    cmd()
        .arg("--long")
        .arg("--blocks")
        .arg("inode,name")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(matched).unwrap());
}

#[test]
fn test_list_inode_with_long_ok() {
    let dir = tempdir();
    cmd()
        .arg("-i")
        .arg("-l")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .success();
}

#[cfg(unix)]
#[test]
fn test_list_broken_link_ok() {
    let dir = tempdir();
    let broken_link = dir.path().join("broken-softlink");
    let matched = "No such file or directory";
    fs::symlink("not-existed-file", &broken_link).unwrap();

    cmd()
        .arg(&broken_link)
        .arg("--ignore-config")
        .assert()
        .stderr(predicate::str::contains(matched).not());

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(broken_link)
        .assert()
        .stderr(predicate::str::contains(matched).not());
}
#[cfg(unix)]
#[test]
fn test_nosymlink_on_non_long() {
    let dir = tempdir();
    dir.child("target").touch().unwrap();
    let link = dir.path().join("link");
    let link_icon = "⇒";
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::contains(link_icon));

    cmd()
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::contains(link_icon).not());
}

#[cfg(unix)]
#[test]
fn test_dereference_link_right_type_and_no_link() {
    let dir = tempdir();
    dir.child("target").touch().unwrap();
    let link = dir.path().join("link");
    let file_type = ".rw";
    let link_icon = "⇒";
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--dereference")
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::starts_with(file_type))
        .stdout(predicate::str::contains(link_icon).not());

    cmd()
        .arg("-l")
        .arg("-L")
        .arg("--ignore-config")
        .arg(link)
        .assert()
        .stdout(predicate::str::starts_with(file_type))
        .stdout(predicate::str::contains(link_icon).not());
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

#[cfg(unix)]
#[test]
fn test_no_show_folder_content_of_symlink_for_long() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(link)
        .assert()
        .stdout(predicate::str::starts_with("lrw"))
        .stdout(predicate::str::contains("⇒"));

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(dir.path().join("link/"))
        .assert()
        .stdout(predicate::str::starts_with(".rw"))
        .stdout(predicate::str::contains("⇒").not());
}

#[cfg(unix)]
#[test]
fn test_show_folder_of_symlink_for_long_multi() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(dir.path().join("link/"))
        .arg(dir.path().join("link"))
        .assert()
        .stdout(predicate::str::starts_with("lrw"))
        .stdout(predicate::str::contains("link:").not()) // do not show dir content when no /
        .stdout(predicate::str::contains("link/:"));
}

#[test]
fn test_version_sort() {
    let dir = tempdir();
    dir.child("0.3.7").touch().unwrap();
    dir.child("0.11.5").touch().unwrap();
    dir.child("11a").touch().unwrap();
    dir.child("0.2").touch().unwrap();
    dir.child("0.11").touch().unwrap();
    dir.child("1").touch().unwrap();
    dir.child("11").touch().unwrap();
    dir.child("2").touch().unwrap();
    dir.child("22").touch().unwrap();
    cmd()
        .arg("-v")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(
            predicate::str::is_match("0.2\n0.3.7\n0.11\n0.11.5\n1\n2\n11\n11a\n22\n$").unwrap(),
        );
}

#[test]
fn test_version_sort_overwrite_by_timesort() {
    let dir = tempdir();
    dir.child("2").touch().unwrap();
    dir.child("11").touch().unwrap();
    cmd()
        .arg("-v")
        .arg("-t")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("11\n2\n$").unwrap());
}

#[test]
fn test_version_sort_overwrite_by_sizesort() {
    use std::fs::File;
    use std::io::Write;
    let dir = tempdir();
    dir.child("2").touch().unwrap();
    let larger = dir.path().join("11");
    let mut larger_file = File::create(larger).unwrap();
    writeln!(larger_file, "this is larger").unwrap();
    cmd()
        .arg("-v")
        .arg("-S")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match("11\n2\n$").unwrap());
}

#[cfg(test)]
#[cfg(target_os = "linux")]
fn bad_utf8(tmp: &std::path::Path, pre: &str, suf: &str) -> String {
    let mut fname = format!("{}/{}", tmp.display(), pre).into_bytes();
    fname.reserve(2 + suf.len());
    fname.push(0xa7);
    fname.push(0xfd);
    fname.extend(suf.as_bytes());
    unsafe { String::from_utf8_unchecked(fname) }
}

#[test]
#[cfg(target_os = "linux")]
fn test_bad_utf_8_extension() {
    use std::fs::File;
    let tmp = tempdir();
    let fname = bad_utf8(tmp.path(), "bad.extension", "");
    File::create(fname).expect("failed to create file");

    cmd()
        .arg(tmp.path())
        .assert()
        .stdout(predicate::str::is_match("bad.extension\u{fffd}\u{fffd}\n$").unwrap());
}

#[test]
#[cfg(target_os = "linux")]
fn test_bad_utf_8_name() {
    use std::fs::File;
    let tmp = tempdir();
    let fname = bad_utf8(tmp.path(), "bad-name", ".ext");
    File::create(fname).expect("failed to create file");

    cmd()
        .arg(tmp.path())
        .assert()
        .stdout(predicate::str::is_match("bad-name\u{fffd}\u{fffd}.ext\n$").unwrap());
}

#[cfg(test)]
fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[cfg(test)]
fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}
