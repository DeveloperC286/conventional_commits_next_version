pub fn get_commit_messages_from(from_commit_hash: &str) -> Vec<String> {
    let mut commit_messages = vec![];

    let repository = get_local_repository();
    let mut walker = repository.revwalk().unwrap();
    walker.push_head().unwrap();
    walker.hide(get_oid(from_commit_hash)).unwrap();

    loop {
        match walker.next() {
            Some(Ok(oid)) => {
                let commit_message_optional = get_commit_message(&repository, oid);
                match commit_message_optional {
                    Some(commit_message) => {
                        trace!(
                            "For the commit id '{}' found the message {:?}.",
                            oid,
                            commit_message
                        );
                        commit_messages.push(commit_message);
                    }
                    None => {
                        warn!("Commit with the id '{}' has no commit message.", oid);
                    }
                }
            }
            Some(Err(error)) => error!("{:?}", error),
            None => break,
        }
    }

    commit_messages.reverse();
    return commit_messages;
}

fn get_commit_message(repository: &git2::Repository, oid: git2::Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => {
            return commit.message().map(|m| m.to_string());
        }
        Err(_error) => {
            error!(
                "Can not find commit with id '{}' in current repository.",
                oid
            );
            std::process::exit(1);
        }
    }
}

fn get_local_repository() -> git2::Repository {
    match git2::Repository::open(std::path::PathBuf::from(".")) {
        Ok(repository) => return repository,
        Err(_error) => {
            error!("Not in a Git repository.");
            std::process::exit(1);
        }
    }
}

fn get_oid(oid_string: &str) -> git2::Oid {
    match git2::Oid::from_str(oid_string) {
        Ok(oid) => return oid,
        Err(_error) => {
            error!("'{}' is not a valid commit id.", oid_string);
            std::process::exit(1);
        }
    }
}
