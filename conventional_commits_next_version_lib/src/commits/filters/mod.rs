use std::collections::HashSet;

use git2::{Repository, TreeWalkMode, TreeWalkResult};

pub(super) struct Filters {
    commits_must_effect: Vec<String>,
}

impl Filters {
    pub(super) fn from(commits_must_effect: Vec<String>) -> Self {
        Filters {
            commits_must_effect,
        }
    }

    pub(crate) fn does_commit_effect(
        &self,
        repository: &Repository,
        commit: &git2::Commit,
    ) -> Result<bool, git2::Error> {
        fn get_all_files_changed_in_commit(
            repository: &Repository,
            commit: &git2::Commit,
        ) -> Result<HashSet<String>, git2::Error> {
            let mut files = HashSet::new();

            let commit_tree = commit.tree()?;
            if commit.parent_count() == 0 {
                // Root Commit
                commit_tree
                    .walk(TreeWalkMode::PostOrder, |directory, entry| {
                        match entry.name() {
                            Some(name) => {
                                let file = format!("{directory}{name}");
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
                    let parent_tree = parent.tree()?;
                    let diff = repository.diff_tree_to_tree(
                        Some(&parent_tree),
                        Some(&commit_tree),
                        None,
                    )?;
                    for delta in diff.deltas() {
                        if let Some(new_file) = delta.new_file().path() {
                            files.insert(new_file.display().to_string());
                        }

                        if let Some(old_file) = delta.old_file().path() {
                            files.insert(old_file.display().to_string());
                        }
                    }
                }
            }

            Ok(files)
        }

        if !self.commits_must_effect.is_empty() {
            let files_in_commit = get_all_files_changed_in_commit(repository, commit)?;
            trace!(
                "Commit with the hash '{}' changes the files {files_in_commit:?}.",
                commit.id(),
            );
            return Ok(self.is_files_within(files_in_commit));
        }

        Ok(true)
    }

    fn is_files_within(&self, files_in_commit: HashSet<String>) -> bool {
        for file_in_commit in files_in_commit {
            for filter in &self.commits_must_effect {
                if file_in_commit.starts_with(filter) {
                    debug!("The file {file_in_commit:?} affects the path {filter:?}.");
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests;
