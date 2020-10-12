use git2::{Oid, Repository, Revwalk};

pub fn get_commit_messages_till_head_from(
    from_commit_hash: Option<git2::Oid>,
    from_tag: Option<String>,
) -> Vec<String> {
    let repository = get_repository();

    if let Some(oid) = from_commit_hash {
        return get_commit_messages_till_head_from_oid(&repository, oid);
    }

    if let Some(tag_name) = from_tag {
        match get_tag_oid(&repository, &tag_name) {
            Some(oid) => {
                return get_commit_messages_till_head_from_oid(&repository, oid);
            }
            None => {
                error!("Could not find a tag with the name '{}'.", tag_name);
                std::process::exit(1);
            }
        }
    }

    error!("Provide either the --from-tag or --from-commit-hash argument.");
    std::process::exit(1);
}

fn get_commit_messages_till_head_from_oid(repository: &Repository, from_commit_hash: Oid) -> Vec<String> {
    let mut commit_messages = vec![];

    for oid in get_commit_oids(repository, from_commit_hash).next() {
        match oid {
            Ok(oid) => match get_commit_message(&repository, oid) {
                Some(commit_message) => {
                    trace!("Found commit '{}'s message '{:?}'.", oid, commit_message);
                    commit_messages.push(commit_message);
                }
                None => {
                    warn!("Commit hash '{}' has no message.", oid);
                }
            }
            Err(error) => {
                error!("{:?}", error);
                std::process::exit(1);
            }
        }
    }

    debug!("{} commit messages found.", commit_messages.len());
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
                    std::process::exit(1);
                }
            }

            match commits.hide(from_commit_hash) {
                Ok(_) => {}
                Err(_) => {
                    error!(
                        "Can not find commit hash '{}' on the revision walker.",
                        from_commit_hash
                    );
                    std::process::exit(1);
                }
            }

            commits
        }
        Err(error) => {
            error!("{:?}", error);
            std::process::exit(1);
        }
    }
}

fn get_commit_message(repository: &Repository, oid: Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => commit.message().map(|m| m.to_string()),
        Err(_) => {
            error!("Can not find commit hash '{}'.", oid);
            std::process::exit(1);
        }
    }
}

fn get_repository() -> Repository {
    match Repository::open_from_env() {
        Ok(repository) => repository,
        Err(error) => {
            error!("{:?}", error);
            std::process::exit(1);
        }
    }
}

fn get_tag_oid(repository: &Repository, matching: &str) -> Option<Oid> {
    let mut oid: Option<Oid> = None;
    let matching = format!("refs/tags/{}", matching);

    match repository.tag_foreach(|tag_oid: Oid, tag_name: &[u8]| -> bool {
        match std::str::from_utf8(tag_name) {
            Ok(tag_name) => {
                if tag_name == matching {
                    debug!("Matching '{}' at commit id '{}'.", tag_name, tag_oid);
                    oid = Some(tag_oid);
                    return false;
                }
            }
            Err(error) => {
                error!("{:?}", error);
                std::process::exit(1);
            }
        }

        true
    }) {
        Ok(_) => {}
        Err(error) => {
            error!("{:?}", error);
            std::process::exit(1);
        }
    }

    oid
}