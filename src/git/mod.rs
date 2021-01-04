use std::collections::HashSet;
use std::process::exit;

use git2::{Commit, Oid, Repository, Revwalk, TreeWalkMode, TreeWalkResult};

pub fn get_commit_messages_till_head_from(
    from_commit_hash: Option<git2::Oid>,
    from_reference: Option<String>,
    monorepo: Option<String>,
) -> Vec<String> {
    let repository = get_repository();

    if let Some(oid) = from_commit_hash {
        return get_commit_messages_till_head_from_oid(&repository, oid, monorepo);
    }

    if let Some(reference) = from_reference {
        return get_commit_messages_till_head_from_oid(
            &repository,
            get_reference(&repository, &reference),
            monorepo,
        );
    }

    error!("Provide either the --from-reference or --from-commit-hash argument.");
    exit(crate::ERROR_EXIT_CODE);
}

fn get_commit_messages_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
    monorepo: Option<String>,
) -> Vec<String> {
    let mut commit_messages: Vec<String> = get_commit_oids(repository, from_commit_hash)
        .map(|oid| match oid {
            Ok(oid) => match get_commit(&repository, oid) {
                Some(commit) => {
                    let commit_message = get_commit_message(&commit);
                    trace!(
                        "Found the commit message {:?} for the commit with the hash '{}'.",
                        commit_message,
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
                                return commit_message;
                            }
                        }

                        debug!(
                            "Ignoring commit with the hash '{}' as {:?} was not found.",
                            oid, monorepo
                        );
                        return None;
                    }

                    commit_message
                }
                None => {
                    exit(crate::ERROR_EXIT_CODE);
                }
            },
            Err(error) => {
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        })
        .filter(|commit_message| commit_message.is_some())
        .map(|commit_message| commit_message.unwrap())
        .collect();

    commit_messages.reverse();
    commit_messages
}

fn get_all_files_changed_in_commit(repository: &Repository, commit: Commit) -> HashSet<String> {
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

fn get_commit_oids(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
    match repository.revwalk() {
        Ok(mut commits) => {
            match commits.push_head() {
                Ok(_) => {}
                Err(_) => {
                    error!("Unable to push HEAD onto the Git revision walker.");
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            match commits.hide(from_commit_hash) {
                Ok(_) => {}
                Err(_) => {
                    error!(
                        "Can not find commit hash '{}' on the Git revision walker.",
                        from_commit_hash
                    );
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            commits
        }
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_commit(repository: &Repository, oid: Oid) -> Option<Commit> {
    match repository.find_commit(oid) {
        Ok(commit) => Some(commit),
        Err(_) => {
            error!("Can not find commit with the hash '{}'.", oid);
            None
        }
    }
}

fn get_commit_message(commit: &Commit) -> Option<String> {
    commit.message().map(|m| m.to_string())
}

fn get_repository() -> Repository {
    match Repository::open_from_env() {
        Ok(repository) => repository,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_reference(repository: &Repository, matching: &str) -> Oid {
    match repository.resolve_reference_from_short_name(matching) {
        Ok(reference) => {
            trace!("Found reference '{}'.", reference.name().unwrap());
            match reference.peel_to_commit() {
                Ok(commit) => commit.id(),
                Err(error) => {
                    error!("{:?}", error);
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }
        Err(_) => {
            error!("Could not find a reference with the name {:?}.", matching);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}
