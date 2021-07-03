use std::process::exit;

use git2::{Oid, Repository, Revwalk};
use semver::Version;

use crate::model::commits::commit::Commit;
use crate::model::monorepos::Monorepos;
use crate::utilities::version::*;

mod commit;

pub struct Commits {
    commits: Vec<Commit>,
}

impl Commits {
    pub fn from(
        from_commit_message: Option<String>,
        from_commit_hash: Option<String>,
        from_reference: Option<String>,
        monorepos: Vec<String>,
    ) -> Self {
        fn from_git(
            from_commit_hash: Option<String>,
            from_reference: Option<String>,
            monorepos: Vec<String>,
        ) -> Commits {
            fn get_commits_till_head_from_oid(
                repository: &Repository,
                from_commit_hash: Oid,
                monorepos: &Monorepos,
            ) -> Commits {
                fn get_commit_revwalker(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
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

                let mut commits: Vec<Commit> = get_commit_revwalker(repository, from_commit_hash)
                    .map(|oid| match oid {
                        Ok(oid) => Commit::from_git(repository, oid, monorepos),
                        Err(error) => {
                            error!("{:?}", error);
                            exit(crate::ERROR_EXIT_CODE);
                        }
                    })
                    .flatten()
                    .collect();

                commits.reverse();
                Commits { commits }
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

            fn parse_to_oid(repository: &Repository, oid: String) -> Oid {
                match oid.len() {
                    0 => {
                        error!("Provided Git commit hash is empty and can not be parsed.");
                        exit(crate::ERROR_EXIT_CODE);
                    }
                    1..=39 => {
                        trace!(
                            "Attempting to find a match for the short commit hash {:?}",
                            oid
                        );
                        let matching_oid_lowercase = oid.to_lowercase();

                        match repository.revwalk() {
                            Ok(mut revwalk) => {
                                match revwalk.push_head() {
                                    Ok(_) => {}
                                    Err(_) => {
                                        error!("Unable to push HEAD onto the Git revision walker.");
                                        exit(crate::ERROR_EXIT_CODE);
                                    }
                                }

                                let matched_commit_hashes: Vec<Oid> = revwalk
                                    .into_iter()
                                    .map(|result| {
                                        return match result {
                                            Ok(oid) => {
                                                let oid_lowercase = oid.to_string().to_lowercase();

                                                if oid_lowercase
                                                    .starts_with(&matching_oid_lowercase)
                                                {
                                                    return Some(oid);
                                                }

                                                None
                                            }
                                            Err(error) => {
                                                error!("{:?}", error);

                                                None
                                            }
                                        };
                                    })
                                    .flatten()
                                    .collect();

                                match matched_commit_hashes.len() {
                                    0 => {
                                        error!("No actual commit hashes start with the provided short commit hash {:?}.", matching_oid_lowercase);
                                        exit(crate::ERROR_EXIT_CODE);
                                    }
                                    1 => *matched_commit_hashes.first().unwrap(),
                                    _ => {
                                        error!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
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
                    40 => match git2::Oid::from_str(&oid) {
                        Ok(oid) => oid,
                        Err(error) => {
                            error!("{:?}", error);
                            exit(crate::ERROR_EXIT_CODE);
                        }
                    },
                    _ => {
                        error!("Provided Git commit hash is too long and can not be parsed.");
                        exit(crate::ERROR_EXIT_CODE);
                    }
                }
            }

            let repository = get_repository();
            let monorepos = Monorepos::from(monorepos);

            if let Some(from_commit_hash) = from_commit_hash {
                return get_commits_till_head_from_oid(
                    &repository,
                    parse_to_oid(&repository, from_commit_hash),
                    &monorepos,
                );
            }

            if let Some(reference) = from_reference {
                return get_commits_till_head_from_oid(
                    &repository,
                    get_reference(&repository, &reference),
                    &monorepos,
                );
            }

            error!("Provide either the --from-reference or --from-commit-hash argument.");
            exit(crate::ERROR_EXIT_CODE);
        }

        match from_commit_message {
            Some(message) => {
                if !monorepos.is_empty() {
                    warn!("Ignoring the provided --monorepo arguments because you used from-commit-message.")
                }
                Commits {
                    commits: vec![Commit::from(message)],
                }
            }
            None => from_git(from_commit_hash, from_reference, monorepos),
        }
    }

    pub fn get_next_version(&self, mut from_version: Version, batch_commits: bool) -> Version {
        fn increment_version_batch(commits: &[Commit], version: &mut Version, pre_major: bool) {
            if commits
                .iter()
                .filter(|commit| commit.is_major_increment())
                .count()
                > 0
            {
                if pre_major {
                    increment_minor(version);
                } else {
                    increment_major(version);
                }
            } else if commits
                .iter()
                .filter(|commit| commit.is_minor_increment())
                .count()
                > 0
            {
                increment_minor(version);
            } else if commits
                .iter()
                .filter(|commit| commit.is_patch_increment())
                .count()
                > 0
            {
                increment_patch(version);
            }
        }

        fn increment_version_consecutive(
            commits: &[Commit],
            version: &mut Version,
            pre_major: bool,
        ) {
            commits.iter().for_each(|commit| {
                if commit.is_major_increment() {
                    if pre_major {
                        increment_minor(version);
                    } else {
                        increment_major(version);
                    }
                } else if commit.is_minor_increment() {
                    increment_minor(version);
                } else if commit.is_patch_increment() {
                    increment_patch(version);
                }
            });
        }

        let pre_major = from_version.major.eq(&0);

        if batch_commits {
            info!("Operating in batch mode.");
            increment_version_batch(&self.commits, &mut from_version, pre_major);
        } else {
            info!("Operating in consecutive mode.");
            increment_version_consecutive(&self.commits, &mut from_version, pre_major);
        }

        from_version
    }
}
