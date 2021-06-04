use semver::Version;

use crate::increment::regex::*;
use crate::utilities::version::*;

mod regex;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    version: Version,
    batch_commits: bool,
) -> Version {
    if batch_commits {
        trace!("Operating in batch mode.");
        get_next_version_from_commits_batch(commit_messages, version)
    } else {
        trace!("Operating in consecutive mode.");
        get_next_version_from_commits_consecutive(commit_messages, version)
    }
}

fn get_next_version_from_commits_batch(
    commit_messages: Vec<String>,
    mut version: Version,
) -> Version {
    let major_commits_count = commit_messages
        .iter()
        .filter(|commit_message| match is_major_increment(commit_message) {
            true => {
                log_major_increment(commit_message);
                true
            }
            false => false,
        })
        .count();

    if major_commits_count > 0 {
        increment_major(&mut version);
        return version;
    }

    let minor_commits_count = commit_messages
        .iter()
        .filter(|commit_message| match is_minor_increment(commit_message) {
            true => {
                log_minor_increment(commit_message);
                true
            }
            false => false,
        })
        .count();

    if minor_commits_count > 0 {
        increment_minor(&mut version);
        return version;
    }

    let patch_commits_count = commit_messages
        .iter()
        .filter(|commit_message| match is_patch_increment(commit_message) {
            true => {
                log_patch_increment(commit_message);
                true
            }
            false => false,
        })
        .count();

    if patch_commits_count > 0 {
        increment_patch(&mut version);
        return version;
    }

    version
}

fn get_next_version_from_commits_consecutive(
    commit_messages: Vec<String>,
    mut version: Version,
) -> Version {
    commit_messages.iter().for_each(|commit_message| {
        if is_major_increment(commit_message) {
            log_major_increment(commit_message);
            increment_major(&mut version);
        } else if is_minor_increment(commit_message) {
            log_minor_increment(commit_message);
            increment_minor(&mut version);
        } else if is_patch_increment(commit_message) {
            log_patch_increment(commit_message);
            increment_patch(&mut version);
        }
    });

    version
}

fn log_major_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning major because of commit message {:?}.",
        commit_message
    );
}

fn log_minor_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning minor because of commit {:?}.",
        commit_message
    );
}

fn log_patch_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning patch because of commit {:?}.",
        commit_message
    );
}

fn is_major_increment(commit_message: &str) -> bool {
    is_major_title_increment(commit_message) || is_major_footer_increment(commit_message)
}

fn is_major_title_increment(commit_message: &str) -> bool {
    MAJOR_TITLE_INCREMENT_REGEX.is_match(commit_message)
}

fn is_major_footer_increment(commit_message: &str) -> bool {
    MAJOR_FOOTER_INCREMENT_REGEX.is_match(commit_message)
}

fn is_minor_increment(commit_message: &str) -> bool {
    MINOR_INCREMENT_REGEX.is_match(commit_message)
}

fn is_patch_increment(commit_message: &str) -> bool {
    PATCH_INCREMENT_REGEX.is_match(commit_message)
}

#[cfg(test)]
mod tests;
