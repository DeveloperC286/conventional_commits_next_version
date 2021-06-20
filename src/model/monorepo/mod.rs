use std::collections::HashSet;

use git2::{Repository, TreeWalkMode, TreeWalkResult};
use std::process::exit;

pub struct Monorepo {
    monorepo: Option<String>,
}

impl Monorepo {
    pub fn from(monorepo: Option<String>) -> Self {
        Monorepo { monorepo }
    }

    pub fn does_commit_effect(&self, repository: &Repository, commit: git2::Commit) -> bool {
        fn get_all_files_changed_in_commit(
            repository: &Repository,
            commit: git2::Commit,
        ) -> HashSet<String> {
            let mut files = HashSet::new();

            match commit.tree() {
                Ok(commit_tree) => {
                    if commit.parent_count() == 0 {
                        // Root Commit
                        commit_tree
                            .walk(TreeWalkMode::PostOrder, |directory, entry| {
                                match entry.name() {
                                    Some(name) => {
                                        let file = format!("{}{}", directory, name);
                                        files.insert(file);
                                    }
                                    None => {
                                        warn!(
                                            "Commit with the hash '{}' has not valid utf-8 files.",
                                            commit.id()
                                        )
                                    }
                                }
                                TreeWalkResult::Ok
                            })
                            .unwrap();
                    } else {
                        // Some merge commits can have multiple parents.
                        for parent in commit.parents() {
                            match parent.tree() {
                                Ok(parent_tree) => {
                                    match repository.diff_tree_to_tree(
                                        Some(&parent_tree),
                                        Some(&commit_tree),
                                        None,
                                    ) {
                                        Ok(diff) => {
                                            for delta in diff.deltas() {
                                                if let Some(new_file) = delta.new_file().path() {
                                                    files.insert(new_file.display().to_string());
                                                }

                                                if let Some(old_file) = delta.old_file().path() {
                                                    files.insert(old_file.display().to_string());
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            error!("{:?}", error);
                                            exit(crate::ERROR_EXIT_CODE);
                                        }
                                    }
                                }
                                Err(error) => {
                                    error!("{:?}", error);
                                    exit(crate::ERROR_EXIT_CODE);
                                }
                            }
                        }
                    }
                }
                Err(error) => {
                    error!("{:?}", error);
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            files
        }

        if self.monorepo.is_some() {
            return self.is_files_within(get_all_files_changed_in_commit(repository, commit));
        }

        true
    }

    fn is_files_within(&self, files_in_commit: HashSet<String>) -> bool {
        match &self.monorepo {
            Some(monorepo) => {
                files_in_commit
                    .iter()
                    .filter(|file_in_commit| file_in_commit.starts_with(monorepo))
                    .inspect(|file_in_commit| {
                        trace!(
                            "File {:?} within the commit is inside {:?}.",
                            file_in_commit,
                            monorepo
                        )
                    })
                    .count()
                    > 0
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests;
