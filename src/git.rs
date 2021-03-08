use crate::meta::git_file_status::GitFileStatus;
use log::{debug, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GitStatus {
    /// No status info
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
    statuses: Vec<(PathBuf, git2::Status)>,
    _cached_dir: Option<PathBuf>,
}

impl GitCache {
    pub fn new(path: &Path) -> GitCache {
        let cachedir = fs::canonicalize(&path).unwrap();
        info!("Trying to retrieve Git statuses for {:?}", cachedir);

        let repo = match git2::Repository::discover(&path) {
            Ok(r) => r,
            Err(e) => {
                warn!("Git discovery error: {:?}", e);
                return Self::empty();
            }
        };

        if let Some(workdir) = repo.workdir() {
            let mut statuses = Vec::new();
            info!("Retrieving Git statuses for workdir {:?}", workdir);
            match repo.statuses(None) {
                Ok(status_list) => {
                    for status_entry in status_list.iter() {
                        let path = workdir.join(Path::new(status_entry.path().unwrap()));
                        let elem = (path, status_entry.status());
                        debug!("{:?}", elem);
                        statuses.push(elem);
                    }
                }
                Err(e) => {
                    warn!("Git retrieve statuses error: {:?}", e)
                }
            }
            info!("GitCache path: {:?}", cachedir);

            GitCache {
                statuses,
                _cached_dir: Some(cachedir),
            }
        } else {
            debug!("No workdir");
            Self::empty()
        }
    }

    pub fn empty() -> Self {
        GitCache {
            statuses: Vec::new(),
            _cached_dir: None,
        }
    }

    pub fn get(&self, filepath: &PathBuf, is_directory: bool) -> Option<GitFileStatus> {
        match std::fs::canonicalize(filepath) {
            Ok(filename) => Some(self.inner_get(&filename, is_directory)),
            Err(err) => {
                log::debug!("error {}", err);
                None
            }
        }
    }

    fn inner_get(&self, filepath: &PathBuf, is_directory: bool) -> GitFileStatus {
        debug!("Look for [recurse={}] {:?}", is_directory, filepath);

        if is_directory {
            self.statuses
                .iter()
                .filter(|&x| x.0.starts_with(filepath))
                .inspect(|&x| debug!("\t{:?}", x.0))
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

    fn check_cache(root: &Path, statuses: &HashMap<&PathBuf, GitFileStatus>) {
        let cache = GitCache::new(root);
        for (&path, status) in statuses.iter() {
            match std::fs::canonicalize(&root.join(path)) {
                Ok(filename) => {
                    let is_directory = filename.is_dir();
                    assert_eq!(
                        &cache.inner_get(&filename, is_directory),
                        status,
                        "Invalid status for file {:?}",
                        filename
                    );
                }
                Err(_) => {}
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
        check_cache(root.path(), &expected_statuses);

        let f0 = PathBuf::from(".gitignore");
        root.child(&f0).write_str("*.bak").unwrap();
        expected_statuses.insert(
            &f0,
            GitFileStatus {
                index: GitStatus::Unmodified,
                workdir: GitStatus::NewInWorkdir,
            },
        );

        // Check now
        check_cache(root.path(), &expected_statuses);

        index.add_path(f0.as_path()).unwrap();

        // Check now
        check_cache(root.path(), &expected_statuses);

        index.write().unwrap();
        *expected_statuses.get_mut(&f0).unwrap() = GitFileStatus {
            index: GitStatus::NewInIndex,
            workdir: GitStatus::Unmodified,
        };

        // Check now
        check_cache(root.path(), &expected_statuses);

        commit(&repo, &mut index, "Add gitignore");
        *expected_statuses.get_mut(&f0).unwrap() = GitFileStatus {
            index: GitStatus::Default,
            workdir: GitStatus::Default,
        };

        // Check now
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

        remove_file(&root.child(&f2).path()).unwrap();
        *expected_statuses.get_mut(&d1).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Deleted,
        };
        *expected_statuses.get_mut(&f2).unwrap() = GitFileStatus {
            index: GitStatus::Unmodified,
            workdir: GitStatus::Deleted,
        };

        // Check now
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);

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
        check_cache(root.path(), &expected_statuses);
    }
}
