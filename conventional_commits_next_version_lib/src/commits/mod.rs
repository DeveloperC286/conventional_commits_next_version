use std::collections::VecDeque;

use git2::{Oid, Repository, Revwalk};
use semver::{BuildMetadata, Prerelease, Version};

use crate::calculation_mode::CalculationMode;
use crate::commits::commit::Commit;
use crate::commits::filters::Filters;
use crate::git_history_mode::GitHistoryMode;

mod commit;
mod filters;

/// A representation of a range of commits within a Git repository.
pub struct Commits {
    commits: VecDeque<Commit>,
}

impl Commits {
    /// Create a new range of commits containing a singular commit created from the commit_message.
    ///
    /// This functionality is intended to allow you to lint a commit message before creating the
    /// commit, e.g. in a Git hook etc.
    ///
    ///```
    ///use conventional_commits_next_version_lib::Commits;
    ///
    ///let commits = Commits::from_commit_message("feat: adding stdin support");
    ///```
    pub fn from_commit_message<T: Into<String>>(commit_message: T) -> Commits {
        Commits {
            commits: VecDeque::from(vec![Commit::from_commit_message(commit_message)]),
        }
    }

    /// Create a new range of commits from a reference exclusively from the commit specified by the reference till inclusively of `HEAD`.
    ///
    /// Supports providing either the full or short name of the reference.
    ///
    /// E.g. short name.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, GitHistoryMode};
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_reference(&repository, "2.5.0", vec![], GitHistoryMode::FirstParent).unwrap();
    /// ```
    ///
    /// E.g. full name.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, GitHistoryMode};
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_reference(&repository, "refs/tags/2.5.0", vec![], GitHistoryMode::FirstParent).unwrap();
    /// ```
    pub fn from_reference<T: AsRef<str>>(
        repository: &Repository,
        reference: T,
        commit_filters: Vec<String>,
        git_history_mode: GitHistoryMode,
    ) -> Result<Commits, git2::Error> {
        let reference_oid = get_reference_oid(repository, reference.as_ref())?;
        get_commits_till_head_from_oid(repository, reference_oid, commit_filters, git_history_mode)
    }

    /// Create a new range of commits from a commit hash exclusively from the commit specified till inclusively of `HEAD`.
    ///
    /// Supports providing either the full commit hash or a shortened commit hash.
    ///
    /// E.g. shortened commit hash.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, GitHistoryMode};
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "2c4aa4d", vec![], GitHistoryMode::FirstParent).unwrap();
    /// ```
    ///
    /// E.g. full commit hash.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, GitHistoryMode};
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "2e785d13a988e95658ace5bf9027aa678eb73c5f", vec![], GitHistoryMode::FirstParent).unwrap();
    /// ```
    pub fn from_commit_hash<T: AsRef<str>>(
        repository: &Repository,
        commit_hash: T,
        commit_filters: Vec<String>,
        git_history_mode: GitHistoryMode,
    ) -> Result<Commits, git2::Error> {
        let commit_oid = parse_to_oid(repository, commit_hash.as_ref())?;
        get_commits_till_head_from_oid(repository, commit_oid, commit_filters, git_history_mode)
    }

    /// Calculate the next semantic version based upon the provided from version and the commits
    /// conforming to the Conventional Commits v1.0.0 specification wihtin the range of commits.
    ///
    /// E.g. calculate the next semantic version in batch mode.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, CalculationMode, GitHistoryMode};
    /// use git2::Repository;
    /// use semver::Version;
    ///
    /// let from_version = Version::parse("1.3.0").unwrap();
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "2e785d13a988e95658ace5bf9027aa678eb73c5f", vec![], GitHistoryMode::FirstParent).unwrap();
    /// let returned_version = commits.get_next_version(from_version, CalculationMode::Batch);
    /// ```
    ///
    /// E.g. calculate the next semantic version in consecutive mode.
    ///
    /// ```
    /// use conventional_commits_next_version_lib::{Commits, CalculationMode, GitHistoryMode};
    /// use git2::Repository;
    /// use semver::Version;
    ///
    /// let from_version = Version::parse("1.3.0").unwrap();
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "2e785d13a988e95658ace5bf9027aa678eb73c5f", vec![], GitHistoryMode::FirstParent).unwrap();
    /// let returned_version = commits.get_next_version(from_version, CalculationMode::Consecutive);
    /// ```
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
    git_history_mode: GitHistoryMode,
) -> Result<Commits, git2::Error> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
        git_history_mode: GitHistoryMode,
    ) -> Result<Revwalk, git2::Error> {
        let mut commits = repository.revwalk()?;
        if git_history_mode == GitHistoryMode::FirstParent {
            commits.simplify_first_parent()?;
        }
        commits.push_head()?;

        match commits.hide(from_commit_hash) {
            Ok(_) => Ok(commits),
            Err(error) => {
                error!("Can not find a commit with the hash '{from_commit_hash}'.");
                Err(error)
            }
        }
    }

    let revwalker = get_revwalker(repository, from_commit_hash, git_history_mode)?;
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

    debug!("Operating upon {} commits.", commits.len());

    Ok(Commits { commits })
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid, git2::Error> {
    match repository.resolve_reference_from_short_name(matching) {
        Ok(reference) => {
            trace!(
                "Matched {matching:?} to the reference {:?}.",
                reference.name().unwrap()
            );
            let commit = reference.peel_to_commit()?;
            Ok(commit.id())
        }
        Err(error) => {
            error!("Could not find a reference with the name {matching:?}.");
            Err(error)
        }
    }
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid, git2::Error> {
    match oid.len() {
        1..=39 => {
            trace!("Attempting to find a match for the short commit hash {oid:?}.");
            let matching_oid_lowercase = oid.to_lowercase();

            let mut revwalker = repository.revwalk()?;
            revwalker.push_head()?;

            let matched_commit_hashes: Vec<Oid> = revwalker
                .filter_map(|result| match result {
                    Ok(oid) => {
                        let oid_lowercase = oid.to_string().to_lowercase();

                        if oid_lowercase.starts_with(&matching_oid_lowercase) {
                            return Some(oid);
                        }

                        None
                    }
                    Err(error) => {
                        error!("{error:?}");
                        None
                    }
                })
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    let error_message = format!(
                        "No commit hashes start with the provided short commit hash {matching_oid_lowercase:?}."
                    );
                    error!("{error_message}");
                    Err(git2::Error::from_str(&error_message))
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    let error_message = format!("Ambiguous short commit hash, the commit hashes {matched_commit_hashes:?} all start with the provided short commit hash {matching_oid_lowercase:?}.");
                    error!("{error_message}");
                    Err(git2::Error::from_str(&error_message))
                }
            }
        }
        _ => match git2::Oid::from_str(oid) {
            Ok(oid) => Ok(oid),
            Err(error) => {
                error!("{oid:?} is not a valid commit hash.");
                Err(error)
            }
        },
    }
}
