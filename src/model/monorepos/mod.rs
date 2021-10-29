use git2::{Repository, TreeWalkMode, TreeWalkResult};
use std::collections::HashSet;
use std::process::exit;

pub(crate) struct Monorepos {
    monorepos: Vec<String>,
}

impl Monorepos {
    pub(crate) fn from(monorepos: Vec<String>) -> Self {
        Monorepos { monorepos }
    }

    pub(crate) fn does_commit_effect(
        &self,
        repository: &Repository,
        commit: &git2::Commit,
    ) -> bool {
        fn get_all_files_changed_in_commit(
            repository: &Repository,
            commit: &git2::Commit,
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

        if !self.monorepos.is_empty() {
            let files_in_commit = get_all_files_changed_in_commit(repository, commit);
            trace!(
                "Commit with the hash '{}' changes the files {:?}.",
                commit.id(),
                files_in_commit
            );
            return self.is_files_within(files_in_commit);
        }

        true
    }

    fn is_files_within(&self, files_in_commit: HashSet<String>) -> bool {
        match &self.monorepos.len() {
            0 => true,
            _ => {
                for file_in_commit in files_in_commit {
                    for monorepo in &self.monorepos {
                        if file_in_commit.starts_with(monorepo) {
                            debug!(
                                "The file {:?} is within the monorepo {:?}.",
                                file_in_commit, monorepo
                            );
                            return true;
                        }
                    }
                }

                false
            }
        }
    }
}

#[cfg(test)]
mod tests;
