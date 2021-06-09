use std::collections::HashSet;
use std::process::exit;

use git2::{Oid, Repository, TreeWalkMode, TreeWalkResult};
use regex::Regex;

const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
const ANY_CHARACTER_REGEX: &str = "([[:digit:]]|[[:alpha:]])+";
const OPTIONAL_ANY_REGEX: &str = "([[:digit:]]|[[:alpha:]]|_|-|[[:space:]])*";

lazy_static! {
    static ref OPTIONAL_SCOPE_REGEX: String = format!(r"(\({}\))?", OPTIONAL_ANY_REGEX);
    static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}({})(!{}|{}!):",
            &*OPTIONAL_PRECEDING_WHITESPACE,
            &*ANY_CHARACTER_REGEX,
            &*OPTIONAL_SCOPE_REGEX,
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new("\nBREAKING( |-)CHANGE:").unwrap();
    static ref PATCH_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}fix{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MINOR_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}feat{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
}

pub struct Commit {
    message: String,
}

impl Commit {
    pub fn from(repository: &Repository, oid: Oid, monorepo: &Option<String>) -> Option<Self> {
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

        match repository.find_commit(oid) {
            Ok(commit) => match commit.message().map(|m| m.to_string()) {
                Some(message) => {
                    trace!(
                        "Found the commit message {:?} for the commit with the hash '{}'.",
                        message,
                        commit.id()
                    );

                    if let Some(monorepo) = &monorepo {
                        let all_files_in_commit =
                            get_all_files_changed_in_commit(repository, commit);
                        trace!(
                            "commit with the hash '{}' changes the files {:?}.",
                            oid,
                            all_files_in_commit
                        );

                        for file_in_commit in all_files_in_commit {
                            if file_in_commit.starts_with(monorepo) {
                                trace!(
                                    "Using commit with the hash '{}' as {:?} matches {:?}.",
                                    oid,
                                    file_in_commit,
                                    monorepo
                                );
                                return Some(Commit { message });
                            }
                        }

                        debug!(
                            "Ignoring commit with the hash '{}' as {:?} was not found.",
                            oid, monorepo
                        );
                        return None;
                    }

                    Some(Commit { message })
                }
                None => {
                    error!(
                        "Can not find commit message for the commit with the hash '{}'.",
                        oid
                    );
                    None
                }
            },
            Err(error) => {
                error!("{:?}", error);
                error!("Can not find commit with the hash '{}'.", oid);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    pub fn is_major_increment(&self) -> bool {
        self.is_major_title_increment() || self.is_major_footer_increment()
    }

    fn is_major_title_increment(&self) -> bool {
        match MAJOR_TITLE_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a major title Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    fn is_major_footer_increment(&self) -> bool {
        match MAJOR_FOOTER_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a major footer Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    pub fn is_minor_increment(&self) -> bool {
        match MINOR_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a minor Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    pub fn is_patch_increment(&self) -> bool {
        match PATCH_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a patch Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }
}

#[cfg(test)]
mod tests;
