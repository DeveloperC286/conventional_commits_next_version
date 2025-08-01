use std::collections::VecDeque;

use anyhow::{bail, Context, Result};
use git2::{Oid, Repository, Revwalk};
use semver::{BuildMetadata, Prerelease, Version};

use crate::calculation_mode::CalculationMode;
use crate::commits::commit::Commit;
use crate::commits::filters::Filters;
use crate::history_mode::HistoryMode;

mod commit;
mod filters;

pub struct Commits {
    commits: VecDeque<Commit>,
}

impl Commits {
    pub fn from_commit_message<T: Into<String>>(commit_message: T) -> Commits {
        Commits {
            commits: VecDeque::from(vec![Commit::from_commit_message(commit_message)]),
        }
    }

    pub fn from_git<T: AsRef<str>>(
        repository: &Repository,
        git: T,
        commit_filters: Vec<String>,
        history_mode: HistoryMode,
    ) -> Result<Commits> {
        let oid = parse_to_oid(repository, git.as_ref()).or_else(|error| {
            get_reference_oid(repository, git.as_ref()).map_err(|e| error.context(e))
        })?;

        get_commits_till_head_from_oid(repository, oid, commit_filters, history_mode)
    }

    pub fn get_next_version(
        &self,
        mut from_version: Version,
        calculation_mode: CalculationMode,
    ) -> Version {
        let pre_major = from_version.major.eq(&0);

        match calculation_mode {
            CalculationMode::Batch => {
                info!("Calculating in batch mode.");
                self.increment_version_batch(&mut from_version, pre_major);
            }
            CalculationMode::Consecutive => {
                info!("Calculating in consecutive mode.");
                self.increment_version_consecutive(&mut from_version, pre_major);
            }
        }

        from_version
    }

    fn increment_version_batch(&self, version: &mut Version, pre_major: bool) {
        if self
            .commits
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
        } else if self
            .commits
            .iter()
            .filter(|commit| commit.is_minor_increment())
            .count()
            > 0
        {
            increment_minor(version);
        } else if self
            .commits
            .iter()
            .filter(|commit| commit.is_patch_increment())
            .count()
            > 0
        {
            increment_patch(version);
        }
    }

    fn increment_version_consecutive(&self, version: &mut Version, pre_major: bool) {
        self.commits.iter().for_each(|commit| {
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
}

fn increment_patch(v: &mut Version) {
    v.patch += 1;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
}

fn increment_minor(v: &mut Version) {
    v.minor += 1;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
}

fn increment_major(v: &mut Version) {
    v.major += 1;
    v.minor = 0;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
}

fn get_commits_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
    commit_filters: Vec<String>,
    history_mode: HistoryMode,
) -> Result<Commits> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
        history_mode: HistoryMode,
    ) -> Result<Revwalk> {
        let mut commits = repository.revwalk()?;
        if history_mode == HistoryMode::First {
            commits.simplify_first_parent()?;
        }
        commits.push_head()?;

        commits.hide(from_commit_hash).context(format!(
            "Can not find a commit with the hash '{from_commit_hash}'."
        ))?;
        Ok(commits)
    }

    let revwalker = get_revwalker(repository, from_commit_hash, history_mode)?;
    let mut commits = VecDeque::new();
    let filters = Filters::from(commit_filters);

    for oid in revwalker {
        let oid = oid?;
        let commit = repository.find_commit(oid)?;

        let is_commit_filtered_out = !filters.does_commit_effect(repository, &commit)?;

        if is_commit_filtered_out {
            debug!("Commit with the hash {oid:?} is being filtered out.");
        } else {
            let commit = Commit::from_git(&commit);
            commits.push_front(commit);
        }
    }

    if commits.is_empty() {
        bail!("No Git commits within the provided range.");
    }

    info!("Found {} commits within the provided range.", commits.len());
    Ok(Commits { commits })
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid> {
    let reference = repository
        .resolve_reference_from_short_name(matching)
        .context(format!(
            "Could not find a reference with the name {matching:?}."
        ))?;
    debug!(
        "Matched {:?} to the reference {:?}.",
        matching,
        reference.name().unwrap()
    );
    let commit = reference.peel_to_commit()?;
    Ok(commit.id())
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid> {
    match oid.len() {
        1..=39 => {
            debug!("Attempting to find a match for the short commit hash {oid:?}.");
            let matching_oid_lowercase = oid.to_lowercase();

            let mut revwalker = repository.revwalk()?;
            revwalker.push_head()?;

            let matched_commit_hashes: Vec<Oid> = revwalker
                .filter_map(|result| match result {
                    Ok(oid) => {
                        let oid_lowercase = oid.to_string().to_lowercase();

                        if oid_lowercase.starts_with(&matching_oid_lowercase) {
                            debug!("Found a match for the short commit hash {oid:?}.");
                            return Some(oid);
                        }

                        None
                    }
                    Err(_) => None,
                })
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    bail!(
                        "No commit hashes start with the provided short commit hash {:?}.",
                        matching_oid_lowercase
                    );
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    bail!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
                }
            }
        }
        _ => git2::Oid::from_str(oid).context(format!("{oid:?} is not a valid commit hash.")),
    }
}
