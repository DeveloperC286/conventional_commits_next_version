use git2::{Oid, Repository, Revwalk};
use semver::Version;
use std::process::exit;

use crate::model::commits::commit::Commit;
use crate::model::monorepo::Monorepo;
use crate::utilities::version::*;

mod commit;

pub struct Commits {
    commits: Vec<Commit>,
}

impl Commits {
    pub fn from(
        from_commit_hash: Option<Oid>,
        from_reference: Option<String>,
        monorepo: Option<String>,
    ) -> Self {
        fn get_commits_till_head_from_oid(
            repository: &Repository,
            from_commit_hash: Oid,
            monorepo: &Monorepo,
        ) -> Commits {
            let mut commits: Vec<Commit> = get_commit_oids(repository, from_commit_hash)
                .map(|oid| match oid {
                    Ok(oid) => Commit::from(repository, oid, monorepo),
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

        let repository = get_repository();
        let monorepo = Monorepo::from(monorepo);

        if let Some(oid) = from_commit_hash {
            return get_commits_till_head_from_oid(&repository, oid, &monorepo);
        }

        if let Some(reference) = from_reference {
            return get_commits_till_head_from_oid(
                &repository,
                get_reference(&repository, &reference),
                &monorepo,
            );
        }

        error!("Provide either the --from-reference or --from-commit-hash argument.");
        exit(crate::ERROR_EXIT_CODE);
    }

    pub fn get_next_version(&self, mut from_version: Version, batch_commits: bool) -> Version {
        fn increment_version_batch(commits: &[Commit], version: &mut Version) {
            if commits
                .iter()
                .filter(|commit| commit.is_major_increment())
                .count()
                > 0
            {
                increment_major(version);
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

        fn increment_version_consecutive(commits: &[Commit], version: &mut Version) {
            commits.iter().for_each(|commit| {
                if commit.is_major_increment() {
                    increment_major(version);
                } else if commit.is_minor_increment() {
                    increment_minor(version);
                } else if commit.is_patch_increment() {
                    increment_patch(version);
                }
            });
        }

        if batch_commits {
            trace!("Operating in batch mode.");
            increment_version_batch(&self.commits, &mut from_version);
        } else {
            trace!("Operating in consecutive mode.");
            increment_version_consecutive(&self.commits, &mut from_version);
        }

        from_version
    }
}
