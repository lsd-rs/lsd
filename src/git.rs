use crate::meta::git_file_status::GitFileStatus;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GitStatus {
    /// No status info
    #[default]
    Default,
    /// No changes (got from git status)
    Unmodified,
    /// Entry is ignored item in workdir
    Ignored,
    /// Entry does not exist in old version (now in stage)
    NewInIndex,
    /// Entry does not exist in old version (not in stage)
    NewInWorkdir,
    /// Type of entry changed between old and new
    Typechange,
    /// Entry does not exist in new version
    Deleted,
    /// Entry was renamed between old and new
    Renamed,
    /// Entry content changed between old and new
    Modified,
    /// Entry in the index is conflicted
    Conflicted,
}

pub struct GitCache {
    #[cfg(not(feature = "no-git"))]
    statuses: Vec<(PathBuf, git2::Status)>,
}

#[cfg(feature = "no-git")]
impl GitCache {
    pub fn new(_: &Path) -> Self {
        Self {}
    }

    pub fn get(&self, _filepath: &PathBuf, _is_directory: bool) -> Option<GitFileStatus> {
        None
    }
}

#[cfg(not(feature = "no-git"))]
impl GitCache {
    pub fn new(path: &Path) -> GitCache {
        let repo = match git2::Repository::discover(path) {
            Ok(r) => r,
            Err(_e) => {
                // Unable to retrieve Git info; it doesn't seem to be a git directory
                return Self::empty();
            }
        };

        if let Some(workdir) = repo.workdir().and_then(|x| std::fs::canonicalize(x).ok()) {
            let mut statuses = Vec::new();
            // Retrieving Git statuses for workdir
            match repo.statuses(None) {
                Ok(status_list) => {
                    for status_entry in status_list.iter() {
                        // git2-rs provides / separated path even on Windows. We have to rebuild it
                        let str_path = status_entry.path().unwrap();
                        let path: PathBuf =
                            str_path.split('/').collect::<Vec<_>>().iter().collect();
                        let path = workdir.join(path);
                        let elem = (path, status_entry.status());
                        statuses.push(elem);
                    }
                }
                Err(err) => {
                    crate::print_error!(
                        "Cannot retrieve Git statuses for directory {:?}: {}",
                        workdir,
                        err
                    );
                }
            }

            GitCache { statuses }
        } else {
            // No workdir
            Self::empty()
        }
    }

    pub fn empty() -> Self {
        GitCache {
            statuses: Vec::new(),
        }
    }

    pub fn get(&self, filepath: &PathBuf, is_directory: bool) -> Option<GitFileStatus> {
        match std::fs::canonicalize(filepath) {
            Ok(filename) => Some(self.inner_get(&filename, is_directory)),
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    crate::print_error!("Cannot get git status for {:?}:  {}", filepath, err);
                }
                None
            }
        }
    }

    fn inner_get(&self, filepath: &PathBuf, is_directory: bool) -> GitFileStatus {
        if is_directory {
            self.statuses
                .iter()
                .filter(|&x| x.0.starts_with(filepath))
                .map(|x| GitFileStatus::new(x.1))
                .fold(GitFileStatus::default(), |acc, x| GitFileStatus {
                    index: std::cmp::max(acc.index, x.index),
                    workdir: std::cmp::max(acc.workdir, x.workdir),
                })
        } else {
            self.statuses
                .iter()
                .find(|&x| filepath == &x.0)
                .map(|e| GitFileStatus::new(e.1))
                .unwrap_or_default()
        }
    }
}

#[cfg(not(feature = "no-git"))]
#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use git2::build::CheckoutBuilder;
    use git2::{CherrypickOptions, Index, Oid, Repository, RepositoryInitOptions};
    use std::collections::HashMap;
    use std::fs::remove_file;
    #[allow(unused)]
    use std::process::Command;

    #[test]
    fn compare_git_status() {
        assert!(GitStatus::Unmodified < GitStatus::Conflicted);
    }

    macro_rules! t {
        ($e:expr) => {
            match $e {
                Ok(e) => e,
                Err(e) => panic!("{} failed with {}", stringify!($e), e),
            }
        };
    }

    fn repo_init() -> (TempDir, Repository) {
        let td = t!(TempDir::new());
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("master");
        let repo = Repository::init_opts(td.path(), &opts).unwrap();
        {
            let mut config = t!(repo.config());
            t!(config.set_str("user.name", "name"));
            t!(config.set_str("user.email", "email"));
            let mut index = t!(repo.index());
            let id = t!(index.write_tree());
            let tree = t!(repo.find_tree(id));
            let sig = t!(repo.signature());
            t!(repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[]));
        }
        (td, repo)
    }

    fn commit(repo: &Repository, index: &mut Index, msg: &str) -> (Oid, Oid) {
        let tree_id = t!(index.write_tree());
        let tree = t!(repo.find_tree(tree_id));
        let sig = t!(repo.signature());
        let head_id = t!(repo.refname_to_id("HEAD"));
        let parent = t!(repo.find_commit(head_id));
        let commit = t!(repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent]));
        (commit, tree_id)
    }

    fn check_cache(root: &Path, statuses: &HashMap<&PathBuf, GitFileStatus>, msg: &str) {
        let cache = GitCache::new(root);
        for (&path, status) in statuses.iter() {
            if let Ok(filename) = std::fs::canonicalize(root.join(path)) {
                let is_directory = filename.is_dir();
                assert_eq!(
                    &cache.inner_get(&filename, is_directory),
                    status,
                    "Invalid status for file {} at stage {}",
                    filename.to_string_lossy(),
                    msg
                );
            }
        }
    }

    #[test]
    fn test_git_workflow() {
        // rename as test_git_workflow
        let (root, repo) = repo_init();
        let mut index = repo.index().unwrap();
        let mut expected_statuses = HashMap::new();

        // Check now
        check_cache(root.path(), &expected_statuses, "initialization");

        let f0 = PathBuf::from(".gitignore");
        root.child(&f0).write_str("*.bak").unwrap();
        expected_statuses.insert(
            &f0,
            GitFileStatus {
                index: GitStatus::Unmodified,
                workdir: GitStatus::NewInWorkdir,
            },
        );

        let _success = Command::new("git")
            .current_dir(root.path())
            .arg("status")
            .status()
            .expect("Git status failed")
            .success();

        // Check now
        check_cache(root.path(), &expected_statuses, "new .gitignore");

        index.add_path(f0.as_path()).unwrap();

        // Check now
        check_cache(root.path(), &expected_statuses, "unstaged .gitignore");

        index.write().unwrap();
        *expected_statuses.get_mut(&f0).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Unmodified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "staged .gitignore");

        commit(&repo, &mut index, "Add gitignore");
        *expected_statuses.get_mut(&f0).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Committed .gitignore");

        let d1 = PathBuf::from("d1");
        let f1 = d1.join("f1");
        root.child(&f1).touch().unwrap();
        let f2 = d1.join("f2.bak");
        root.child(&f2).touch().unwrap();
        expected_statuses.insert(
            &d1,
            GitFileStatus {
                index: GitStatus::Unmodified,
                workdir: GitStatus::NewInWorkdir,
            },
        );
        expected_statuses.insert(
            &f1,
            GitFileStatus {
                index: GitStatus::Unmodified,
                workdir: GitStatus::NewInWorkdir,
            },
        );
        expected_statuses.insert(
            &f2,
            GitFileStatus {
                index: GitStatus::Unmodified,
                workdir: GitStatus::Ignored,
            },
        );

        // Check now
        check_cache(root.path(), &expected_statuses, "New files");

        index.add_path(f1.as_path()).unwrap();
        index.write().unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Ignored,
        };
        *expected_statuses.get_mut(&f1).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Unmodified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Unstaged new files");

        index.add_path(f2.as_path()).unwrap();
        index.write().unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Unmodified,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Unmodified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Staged new files");

        let (commit1_oid, _) = commit(&repo, &mut index, "Add new files");
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };
        *expected_statuses.get_mut(&f1).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Committed new files");

        remove_file(root.child(&f2).path()).unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Deleted,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Deleted,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Remove file");

        root.child(&f1).write_str("New content").unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Modified,
        }; // more important to see modified vs deleted ?
        *expected_statuses.get_mut(&f1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Modified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Change file");

        index.remove_path(&f2).unwrap();
        index.write().unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Deleted,
            workdir: GitStatus::Modified,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Deleted,
            workdir: GitStatus::Unmodified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses, "Staged changes");

        commit(&repo, &mut index, "Remove backup file");
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Modified,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };

        // Check now
        check_cache(
            root.path(),
            &expected_statuses,
            "Committed changes (first part)",
        );

        index.add_path(&f1).unwrap();
        index.write().unwrap();
        commit(&repo, &mut index, "Save modified file");
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };
        *expected_statuses.get_mut(&f1).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };

        // Check now
        check_cache(
            root.path(),
            &expected_statuses,
            "Committed changes (second part)",
        );

        let branch_commit = repo.find_commit(commit1_oid).unwrap();
        let branch = repo
            .branch("conflict-branch", &branch_commit, true)
            .unwrap();
        repo.set_head(format!("refs/heads/{}", branch.name().unwrap().unwrap()).as_str())
            .unwrap();
        let mut checkout_opts = CheckoutBuilder::new();
        checkout_opts.force();
        repo.checkout_head(Some(&mut checkout_opts)).unwrap();

        root.child(&f1)
            .write_str("New conflicting content")
            .unwrap();
        root.child(&f2)
            .write_str("New conflicting content")
            .unwrap();
        index.add_path(&f1).unwrap();
        index.add_path(&f2).unwrap();
        index.write().unwrap();
        let (commit2_oid, _) = commit(&repo, &mut index, "Save conflicting changes");

        // Check now
        check_cache(
            root.path(),
            &expected_statuses,
            "Committed changes in branch",
        );

        repo.set_head("refs/heads/master").unwrap();
        repo.checkout_head(Some(&mut checkout_opts)).unwrap();
        let mut cherrypick_opts = CherrypickOptions::new();
        let branch_commit = repo.find_commit(commit2_oid).unwrap();
        repo.cherrypick(&branch_commit, Some(&mut cherrypick_opts))
            .unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Conflicted,
        };
        *expected_statuses.get_mut(&f1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Conflicted,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Conflicted,
        };

        // let _success = Command::new("git")
        //     .current_dir(root.path())
        //     .arg("status")
        //     .status()
        //     .expect("Git status failed")
        //     .success();

        // Check now
        check_cache(
            root.path(),
            &expected_statuses,
            "Conflict between master and branch",
        );
    }
}
