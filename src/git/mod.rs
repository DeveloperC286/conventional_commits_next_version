use git2::{Oid, Repository, Revwalk};

pub fn get_commit_messages_till_head_from(
    from_commit_hash: Option<git2::Oid>,
    from_reference: Option<String>,
) -> Vec<String> {
    let repository = get_repository();

    if let Some(oid) = from_commit_hash {
        return get_commit_messages_till_head_from_oid(&repository, oid);
    }

    if let Some(reference) = from_reference {
        return get_commit_messages_till_head_from_oid(
            &repository,
            get_reference(&repository, &reference),
        );
    }

    error!("Provide either the --from-reference or --from-commit-hash argument.");
    std::process::exit(crate::ERROR_EXIT_CODE);
}

fn get_commit_messages_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
) -> Vec<String> {
    let mut commit_messages: Vec<String> = get_commit_oids(repository, from_commit_hash)
        .map(|oid| match oid {
            Ok(oid) => match get_commit_message(&repository, oid) {
                Some(commit_message) => {
                    trace!("Found commit '{}'s message '{:?}'.", oid, commit_message);
                    Some(commit_message)
                }
                None => {
                    warn!("Commit hash '{}' has no message.", oid);
                    None
                }
            },
            Err(error) => {
                error!("{:?}", error);
                std::process::exit(crate::ERROR_EXIT_CODE);
            }
        })
        .filter(|commit_message| commit_message.is_some())
        .map(|commit_message| commit_message.unwrap())
        .collect();

    commit_messages.reverse();
    commit_messages
}

fn get_commit_oids(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
    match repository.revwalk() {
        Ok(mut commits) => {
            match commits.push_head() {
                Ok(_) => {}
                Err(_) => {
                    error!("Unable to push HEAD onto the revision walker.");
                    std::process::exit(crate::ERROR_EXIT_CODE);
                }
            }

            match commits.hide(from_commit_hash) {
                Ok(_) => {}
                Err(_) => {
                    error!(
                        "Can not find commit hash '{}' on the revision walker.",
                        from_commit_hash
                    );
                    std::process::exit(crate::ERROR_EXIT_CODE);
                }
            }

            commits
        }
        Err(error) => {
            error!("{:?}", error);
            std::process::exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_commit_message(repository: &Repository, oid: Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => commit.message().map(|m| m.to_string()),
        Err(_) => {
            error!("Can not find commit hash '{}'.", oid);
            std::process::exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_repository() -> Repository {
    match Repository::open_from_env() {
        Ok(repository) => repository,
        Err(error) => {
            error!("{:?}", error);
            std::process::exit(crate::ERROR_EXIT_CODE);
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
                    std::process::exit(crate::ERROR_EXIT_CODE);
                }
            }
        }
        Err(_) => {
            error!("Could not find a reference with the name {:?}.", matching);
            std::process::exit(crate::ERROR_EXIT_CODE);
        }
    }
}
