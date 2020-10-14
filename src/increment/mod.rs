use regex::Regex;
use semver::Version;

pub const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
pub const ANY_REGEX: &str = "([[:digit:]]|[[:alpha:]]|_|-|[[:space:]])*";

lazy_static! {
    static ref OPTIONAL_SCOPE_REGEX: String = format!(r"(\({}\))?", ANY_REGEX);
    static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}({})(!{}|{}!):",
            &*OPTIONAL_PRECEDING_WHITESPACE,
            &*ANY_REGEX,
            &*OPTIONAL_SCOPE_REGEX,
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i)^({}){}:(.)*(\n)*BREAKING CHANGE:",
            &*ANY_REGEX, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref PATCH_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}fix{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MINOR_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}feat{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
}

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
        version.increment_major();
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
        version.increment_minor();
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
        version.increment_patch();
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
            version.increment_major();
        } else if is_minor_increment(commit_message) {
            log_minor_increment(commit_message);
            version.increment_minor();
        } else if is_patch_increment(commit_message) {
            log_patch_increment(commit_message);
            version.increment_patch();
        }
    });

    version
}

fn log_major_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning major because of commit message '{:?}'.",
        commit_message
    );
}

fn log_minor_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning minor because of commit '{:?}'.",
        commit_message
    );
}

fn log_patch_increment(commit_message: &str) {
    debug!(
        "Incrementing semantic versioning patch because of commit '{:?}'.",
        commit_message
    );
}

fn is_major_increment(commit_message: &str) -> bool {
    MAJOR_TITLE_INCREMENT_REGEX.is_match(commit_message)
        || MAJOR_FOOTER_INCREMENT_REGEX.is_match(commit_message)
}

fn is_minor_increment(commit_message: &str) -> bool {
    MINOR_INCREMENT_REGEX.is_match(commit_message)
}

fn is_patch_increment(commit_message: &str) -> bool {
    PATCH_INCREMENT_REGEX.is_match(commit_message)
}

#[cfg(test)]
mod tests;
