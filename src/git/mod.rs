pub fn get_commit_messages_from(from_commit_hash: &str) -> Vec<String> {
    let mut commit_messages = vec![];

    let repository = get_local_repository();
    let mut revwalk = get_revwalk(&repository, get_oid(from_commit_hash));

    loop {
        match revwalk.next() {
            Some(Ok(oid)) => match get_commit_message(&repository, oid) {
                Some(commit_message) => {
                    trace!("Adding commit '{}'s message {:?}.", oid, commit_message);
                    commit_messages.push(commit_message);
                }
                None => {
                    warn!("Commit '{}' has no message.", oid);
                }
            },
            Some(Err(error)) => {
                error!("Revwalk threw error while walking the commits.");
                error!("{:?}", error);
                std::process::exit(1);
            }
            None => break,
        }
    }

    commit_messages.reverse();
    return commit_messages;
}

fn get_revwalk(repository: &git2::Repository, from_commit_hash: git2::Oid) -> git2::Revwalk {
    match repository.revwalk() {
        Ok(mut revwalk) => {
            match revwalk.push_head() {
                Ok(_result) => {}
                Err(_error) => {
                    error!("Unable to push HEAD onto the revwalk.");
                    std::process::exit(1);
                }
            }

            match revwalk.hide(from_commit_hash) {
                Ok(_result) => {}
                Err(_error) => {
                    error!("Can not find commit '{}' on the revwalk.", from_commit_hash);
                    std::process::exit(1);
                }
            }

            return revwalk;
        }
        Err(error) => {
            error!("Unable to get revwalk from local repository.");
            error!("{:?}", error);
            std::process::exit(1);
        }
    };
}

fn get_commit_message(repository: &git2::Repository, oid: git2::Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => {
            return commit.message().map(|m| m.to_string());
        }
        Err(_error) => {
            error!("Can not find commit '{}' in current repository.", oid);
            std::process::exit(1);
        }
    }
}

fn get_local_repository() -> git2::Repository {
    match git2::Repository::open(std::path::PathBuf::from(".")) {
        Ok(repository) => return repository,
        Err(_error) => {
            error!("Current directory is not a Git repository.");
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
