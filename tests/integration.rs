extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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
fn test_almost_sort_with_folder() {
    let tmp = tempdir();
    tmp.child("z").create_dir_all().unwrap();
    tmp.child("z/a").touch().unwrap();

    cmd()
        .current_dir(tmp.path())
        .arg("-a")
        .arg("--ignore-config")
        .arg("z")
        .assert()
        .stdout(predicate::str::is_match("\\.\n\\.\\.\na\n$").unwrap());
}

#[test]
fn test_list_inode_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";
    #[cfg(unix)]
    let matched = "\\d+ +one\n\\d+ +two\n$";

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

    #[cfg(windows)]
    let matched = "- one\n\\- two\n$";
    #[cfg(unix)]
    let matched = "\\d+ +one\n\\d+ +two\n$";

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
    let matched = "\\d+ +one\n\\d+ +two\n$";

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

// ls link
// should show dir content
#[cfg(unix)]
#[test]
fn test_nosymlink_on_non_long() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    let link_icon = "⇒";
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::contains(link_icon).not());
}

// ls -l link
// should show the link itself
#[cfg(unix)]
#[test]
fn test_symlink_on_long() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    let link_icon = "⇒";
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::contains(link_icon));
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
fn test_dereference_link_broken_link() {
    let dir = tempdir();
    let link = dir.path().join("link");
    fs::symlink("target", &link).unwrap();

    cmd()
        .arg("-l")
        .arg("--dereference")
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stderr(predicate::str::contains("No such file or directory"));

    cmd()
        .arg("-l")
        .arg("-L")
        .arg("--ignore-config")
        .arg(link)
        .assert()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_dereference_link_broken_link_output() {
    let dir = tempdir();

    let link = dir.path().join("link");
    let target = dir.path().join("target");

    #[cfg(unix)]
    fs::symlink(target, &link).unwrap();

    // this needs to be tested on Windows
    // likely to fail because of permission issue
    // see https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(target, &link).expect("failed to create broken symlink");

    cmd()
        .arg("-l")
        .arg("--dereference")
        .arg("--ignore-config")
        .arg(&link)
        .assert()
        .stdout(predicate::str::starts_with("l????????? ? ? ? ?"));

    cmd()
        .arg("-l")
        .arg("-L")
        .arg("--ignore-config")
        .arg(link)
        .assert()
        .stdout(predicate::str::starts_with("l????????? ? ? ? ?"));
}

/// should work both tty available and not
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

/// ls -l link
/// should show the link itself
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
}

/// ls -l link/
/// should show the dir content
#[cfg(unix)]
#[test]
fn test_show_folder_content_of_symlink_for_long_tail_slash() {
    let dir = tempdir();
    dir.child("target").child("inside").touch().unwrap();
    let link = dir.path().join("link");
    fs::symlink("target", link).unwrap();

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
    fs::symlink("target", link).unwrap();

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
        .arg("--ignore-config")
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
        .arg("--ignore-config")
        .assert()
        .stdout(predicate::str::is_match("bad-name\u{fffd}\u{fffd}.ext\n$").unwrap());
}

#[test]
fn test_tree() {
    let tmp = tempdir();
    tmp.child("one").touch().unwrap();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/two").touch().unwrap();

    cmd()
        .arg(tmp.path())
        .arg("--tree")
        .arg("--ignore-config")
        .assert()
        .stdout(predicate::str::is_match("├── one\n└── one.d\n    └── two\n$").unwrap());
}

#[test]
fn test_tree_all_not_show_self() {
    let tmp = tempdir();
    tmp.child("one").touch().unwrap();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/two").touch().unwrap();
    tmp.child("one.d/.hidden").touch().unwrap();

    cmd()
        .arg(tmp.path())
        .arg("--tree")
        .arg("--all")
        .arg("--ignore-config")
        .assert()
        .stdout(
            predicate::str::is_match("├── one\n└── one.d\n    ├── .hidden\n    └── two\n$")
                .unwrap(),
        );
}

#[test]
fn test_tree_show_edge_before_name() {
    let tmp = tempdir();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/two").touch().unwrap();

    cmd()
        .arg(tmp.path())
        .arg("--tree")
        .arg("--long")
        .arg("--ignore-config")
        .assert()
        .stdout(predicate::str::is_match("└── two\n$").unwrap());
}

#[test]
fn test_tree_d() {
    let tmp = tempdir();
    tmp.child("one").touch().unwrap();
    tmp.child("two").touch().unwrap();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/one").touch().unwrap();
    tmp.child("one.d/one.d").create_dir_all().unwrap();
    tmp.child("two.d").create_dir_all().unwrap();

    cmd()
        .arg(tmp.path())
        .arg("--tree")
        .arg("-d")
        .arg("--ignore-config")
        .assert()
        .stdout(predicate::str::is_match("├── one.d\n│   └── one.d\n└── two.d\n$").unwrap());
}

#[cfg(unix)]
#[test]
fn test_tree_no_dereference() {
    let tmp = tempdir();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/samplefile").touch().unwrap();
    let link = tmp.path().join("link");
    fs::symlink("one.d", link).unwrap();

    cmd()
        .arg("--tree")
        .arg("--ignore-config")
        .arg(tmp.path())
        .assert()
        .stdout(
            predicate::str::is_match("├── link ⇒ one.d\n└── one.d\n    └── samplefile\n$").unwrap(),
        );
}

#[cfg(unix)]
#[test]
fn test_tree_dereference() {
    let tmp = tempdir();
    tmp.child("one.d").create_dir_all().unwrap();
    tmp.child("one.d/samplefile").touch().unwrap();
    let link = tmp.path().join("link");
    fs::symlink("one.d", link).unwrap();

    cmd()
        .arg("--ignore-config")
        .arg(tmp.path())
        .arg("--tree")
        .arg("-L")
        .assert()
        .stdout(
            predicate::str::is_match(
                "├── link\n│   └── samplefile\n└── one.d\n    └── samplefile\n$",
            )
            .unwrap(),
        );
}

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}

#[cfg(unix)]
#[test]
fn test_lower_case_name_icon_match() {
    let dir = tempdir();
    dir.child(".trash").touch().unwrap();
    let test_file = dir.path().join(".trash");

    cmd()
        .arg("--icon")
        .arg("always")
        .arg("--ignore-config")
        .arg(test_file)
        .assert()
        .stdout(predicate::str::contains("\u{f1f8}"));
}

#[cfg(unix)]
#[test]
fn test_upper_case_name_icon_match() {
    let dir = tempdir();
    dir.child(".TRASH").touch().unwrap();
    let test_file = dir.path().join(".TRASH");

    cmd()
        .arg("--icon")
        .arg("always")
        .arg("--ignore-config")
        .arg(test_file)
        .assert()
        .stdout(predicate::str::contains("\u{f1f8}"));
}

#[cfg(unix)]
#[test]
fn test_lower_case_ext_icon_match() {
    let dir = tempdir();
    dir.child("test.7z").touch().unwrap();
    let test_file = dir.path().join("test.7z");

    cmd()
        .arg("--icon")
        .arg("always")
        .arg("--ignore-config")
        .arg(test_file)
        .assert()
        .stdout(predicate::str::contains("\u{f410}"));
}

#[cfg(unix)]
#[test]
fn test_upper_case_ext_icon_match() {
    let dir = tempdir();
    dir.child("test.7Z").touch().unwrap();
    let test_file = dir.path().join("test.7Z");

    cmd()
        .arg("--icon")
        .arg("always")
        .arg("--ignore-config")
        .arg(test_file)
        .assert()
        .stdout(predicate::str::contains("\u{f410}"));
}

#[cfg(unix)]
#[test]
fn test_truncate_owner() {
    let dir = tempdir();
    dir.child("foo").touch().unwrap();

    cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg("--truncate-owner-after")
        .arg("1")
        .arg("--truncate-owner-marker")
        .arg("…")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(" .… .… ").unwrap());
}

#[cfg(unix)]
#[test]
fn test_custom_config_file_parsing() {
    let dir = tempdir();
    dir.child("config.yaml").write_str("layout: tree").unwrap();
    dir.child("folder").create_dir_all().unwrap();
    dir.child("folder/file").touch().unwrap();
    let custom_config = dir.path().join("config.yaml");

    cmd()
        .arg("--config-file")
        .arg(custom_config)
        .arg(dir.child("folder").path())
        .assert()
        .stdout(predicate::str::is_match("folder\n└── file").unwrap());
}

#[test]
fn test_cannot_access_file_exit_status() {
    let dir = tempdir();
    let does_not_exist = dir.path().join("does_not_exist");

    let status = cmd()
        .arg("-l")
        .arg("--ignore-config")
        .arg(does_not_exist)
        .status()
        .unwrap()
        .code()
        .unwrap();

    assert_eq!(status, 2)
}

#[cfg(unix)]
#[test]
fn test_cannot_access_subdir_exit_status() {
    let tmp = tempdir();

    let readonly = std::fs::Permissions::from_mode(0o400);
    tmp.child("d/subdir/onemore").create_dir_all().unwrap();

    std::fs::set_permissions(tmp.child("d").path().join("subdir"), readonly).unwrap();

    let status = cmd()
        .arg("--tree")
        .arg("--ignore-config")
        .arg(tmp.child("d").path())
        .status()
        .unwrap()
        .code()
        .unwrap();

    assert_eq!(status, 1)
}

#[test]
fn test_date_custom_format_supports_nanos_with_length() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    cmd()
        .arg("-l")
        .arg("--date")
        .arg("+testDateFormat%.3f")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(
            predicate::str::is_match("testDateFormat\\.[0-9]{3}")
                .unwrap()
                .count(2),
        );
}

#[test]
fn test_date_custom_format_supports_padding() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    cmd()
        .arg("-l")
        .arg("--date")
        .arg("+testDateFormat%_d")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(
            predicate::str::is_match("testDateFormat[\\s0-9]{2}")
                .unwrap()
                .count(2),
        );
}

#[test]
fn test_all_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    cmd()
        .arg("-a")
        .arg("-d")
        .arg("--ignore-config")
        .arg(dir.path())
        .assert()
        .stdout(predicate::str::is_match(".").unwrap());
}

#[test]
fn test_multiple_files() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();

    cmd()
        .arg(dir.path().join("one"))
        .arg(dir.path().join("two"))
        .assert()
        .stdout(predicate::str::is_match(".").unwrap());
}
